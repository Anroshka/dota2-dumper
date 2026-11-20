"""Check a snapshot without running Dota. Optional Rust/C++ compilation needs compilers."""
import argparse
import json
from pathlib import Path
import re
import shutil
import subprocess


FORMATS = ('cs', 'hpp', 'json', 'rs', 'zig')
MODULE = re.compile(r'^\s*// Module: (\S+)$', re.MULTILINE)
CONSTANT = re.compile(r'(?:r#|@")?(\w+)"?(?:: usize)?\s*=\s*(0x[\dA-Fa-f]+);')


def read_json(path):
    def unique_object(pairs):
        result = {}
        for name, value in pairs:
            if name in result:
                raise ValueError(f'{path}: duplicate key {name!r}')
            result[name] = value
        return result

    return json.loads(path.read_text(encoding='utf-8'), object_pairs_hook=unique_object)


def check_constants(root, stem):
    expected = read_json(root / f'{stem}.json')
    for extension in FORMATS:
        if extension == 'json':
            continue
        path = root / f'{stem}.{extension}'
        sections = MODULE.split(path.read_text(encoding='utf-8'))
        actual = {}
        for module, body in zip(sections[1::2], sections[2::2]):
            if module in actual:
                raise ValueError(f'{path}: duplicate module {module}')
            entries = CONSTANT.findall(body)
            values = {name: int(value, 16) for name, value in entries}
            if len(entries) != len(values):
                raise ValueError(f'{path}: duplicate constants in {module}')
            actual[module] = values
        if actual != expected:
            raise ValueError(f'{path} disagrees with {stem}.json')
    return sum(map(len, expected.values()))


IDENT = r'(?:r#)?\w+|@"\w+"'
TYPE_HEADERS = {
    'cs': (rf'public static class ({IDENT})', rf'public enum ({IDENT}) : (\w+)'),
    'hpp': (rf'namespace ({IDENT})', rf'enum class ({IDENT}) : (\w+)'),
    'rs': (rf'pub mod ({IDENT})', rf'pub enum ({IDENT})'),
    'zig': (rf'pub const ({IDENT}) = struct', rf'pub const ({IDENT}) = enum\((\w+)\)'),
}
FIELD_START = {'cs': 'public const nint ', 'hpp': 'constexpr std::ptrdiff_t ',
               'rs': 'pub const ', 'zig': 'pub const '}
WIDTHS = {
    'byte': 1, 'ushort': 2, 'uint': 4, 'ulong': 8,
    **{f'{prefix}{width * 8}{suffix}': width for width in (1, 2, 4, 8)
       for prefix, suffix in (('u', ''), ('uint', '_t'))},
}


def spelling(name, extension):
    if extension == 'rs' and name in ('Self', 'self', 'super', 'crate'):
        return name + '_'
    return name


def parse_schema(text, extension, path):
    class_header, enum_header = TYPE_HEADERS[extension]
    member_pattern = re.compile(
        rf'({IDENT}) = (?:unchecked\(\(\w+\)(-?\d+)\)|(0x[\dA-Fa-f]+)),?'
    )
    field_pattern = re.compile(
        rf'{FIELD_START[extension]}({IDENT})(?:: usize)? = (0x[\dA-Fa-f]+);(?: // .*)?'
    )
    classes, enums = {}, {}
    body = MODULE.split(text)[2].splitlines()
    # The first structural declaration after the module comment wraps all schema types.
    wrapper = next(i for i, line in enumerate(body) if re.fullmatch(class_header + r' \{', line.strip()))
    rows = iter(body[wrapper + 1:])
    repr_width = None
    for line in rows:
        line = line.strip()
        repr_match = re.fullmatch(r'#\[repr\((\w+)\)\]', line)
        if repr_match:
            repr_width = WIDTHS[repr_match[1]]
            continue
        enum = re.fullmatch(enum_header + r' \{', line)
        cls = re.fullmatch(class_header + r' \{', line)
        if not (enum or cls):
            continue

        name = (enum or cls)[1].removeprefix('r#').removeprefix('@"').removesuffix('"')
        table = enums if enum else classes
        if name in table:
            raise ValueError(f'{path}: duplicate schema type {name}')
        fields = {}
        width = (repr_width if extension == 'rs' else WIDTHS[enum[2]]) if enum else None
        repr_width = None
        for declaration in rows:
            declaration = declaration.strip()
            if declaration in ('}', '};'):
                break
            if not declaration:
                continue
            match = (member_pattern if enum else field_pattern).fullmatch(declaration)
            if not match:
                raise ValueError(f'{path}::{name}: invalid declaration {declaration!r}')
            field = match[1].removeprefix('r#').removeprefix('@"').removesuffix('"')
            if field in fields:
                raise ValueError(f'{path}::{name}: duplicate member {field}')
            if enum:
                value = int(match[2], 10) if match[2] is not None else int(match[3], 16)
                if match[2] is not None:
                    value &= (1 << (8 * width)) - 1
            else:
                value = int(match[2], 16)
            fields[field] = value
        table[name] = (width, fields) if enum else fields
    return classes, enums


def check_schema_values(text, extension, path, body):
    classes, enums = parse_schema(text, extension, path)
    if classes.keys() != {spelling(name, extension) for name in body['classes']}:
        raise ValueError(f'{path}: class names disagree with JSON')
    for name, cls in body['classes'].items():
        expected = {spelling(field, extension): value for field, value in cls['fields'].items()}
        if classes[spelling(name, extension)] != expected:
            raise ValueError(f'{path}::{name}: field offsets disagree with JSON')
    if enums.keys() != {spelling(name, extension) for name in body['enums']}:
        raise ValueError(f'{path}: enum names disagree with JSON')
    for name, item in body['enums'].items():
        width, actual = enums[spelling(name, extension)]
        if width != item['alignment']:
            raise ValueError(f'{path}::{name}: enum width disagrees with JSON')
        mask = (1 << (8 * width)) - 1
        expected = {spelling(member, extension): value & mask
                    for member, value in item['members'].items()}
        if extension in ('rs', 'zig'):
            # Rust and Zig use one declared name per discriminant. Any one of the
            # original names for a value is a valid choice.
            values = set(actual.values())
            agrees = (len(actual) == len(values)
                       and values == set(expected.values())
                       and all(expected.get(member) == value for member, value in actual.items()))
        else:
            agrees = actual == expected
        if not agrees:
            raise ValueError(f'{path}::{name}: enum members disagree with JSON')


def check(root, build_dir, compile_rust=False, compile_cpp=False):
    info = read_json(root / 'info.json')
    timestamp = info['timestamp'].replace('T', ' ').removesuffix('+00:00') + ' UTC'
    counts = {stem: check_constants(root, stem)
              for stem in ('offsets', 'buttons', 'interfaces')}

    modules = classes = enums = 0
    stems = set(counts)
    for path in root.glob('*_dll.json'):
        document = read_json(path)
        module = path.stem.removesuffix('_dll') + '.dll'
        if set(document) != {module}:
            raise ValueError(f'{path}: expected schema for {module}')
        body = document[module]
        if module == 'client.dll' and not (body['classes'] and body['enums']):
            raise ValueError(f'{path}: missing client classes or enums')
        stems.add(path.stem)
        modules += 1
        classes += len(body['classes'])
        enums += len(body['enums'])
        for name, cls in body['classes'].items():
            for field, offset in cls['fields'].items():
                if not re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*', field) or type(offset) is not int or offset < 0:
                    raise ValueError(f'{path}: invalid schema field in {name}: {field!r}={offset}')
        for extension in FORMATS:
            if extension == 'json':
                continue
            schema_path = path.with_suffix('.' + extension)
            text = schema_path.read_text(encoding='utf-8')
            if MODULE.findall(text) != [module]:
                raise ValueError(f'{schema_path}: expected schema for {module}')
            for label, key in (('Class', 'classes'), ('Enum', 'enums')):
                counts_in_header = re.findall(rf'// {label} count: (\d+)', text)
                if counts_in_header != [str(len(body[key]))]:
                    raise ValueError(f'{schema_path}: {key} count disagrees with JSON')
            check_schema_values(text, extension, schema_path, body)
    if 'client_dll' not in stems:
        raise ValueError('no client.dll schema found')

    expected_files = {f'{stem}.{extension}' for stem in stems for extension in FORMATS}
    expected_files.add('info.json')
    actual_files = {path.name for path in root.iterdir() if path.suffix[1:] in FORMATS}
    if actual_files != expected_files:
        missing = sorted(expected_files - actual_files)
        unexpected = sorted(actual_files - expected_files)
        raise ValueError(f'snapshot files differ: missing={missing}, unexpected={unexpected}')
    for name in sorted(expected_files):
        if not name.endswith('.json'):
            with (root / name).open(encoding='utf-8') as file:
                file.readline()
                if file.readline().strip() != '// ' + timestamp:
                    raise ValueError(f'{root / name}: timestamp disagrees with info.json')

    build_dir.mkdir(parents=True, exist_ok=True)
    compiled = {}
    for enabled, compiler, extension in ((compile_rust, 'rustc', 'rs'),
                                         (compile_cpp, 'g++', 'hpp')):
        if not enabled:
            continue
        if not shutil.which(compiler):
            raise ValueError(f'{compiler} is required for .{extension} compilation')
        files = sorted(root.glob('*.' + extension))
        for path in files:
            if compiler == 'rustc':
                command = [compiler, '--edition', '2024', '--crate-type', 'lib',
                           '--crate-name', path.stem, '--emit', 'metadata', '-A', 'warnings',
                           str(path), '-o', str(build_dir / (path.stem + '.rmeta'))]
            else:
                command = [compiler, '-std=c++17', '-fsyntax-only', '-x', 'c++', str(path)]
            result = subprocess.run(command, capture_output=True, text=True,
                                    encoding='utf-8', errors='replace', timeout=120)
            if result.returncode:
                raise ValueError(f'{path} failed compilation:\n{result.stderr}')
        compiled[extension] = len(files)
    return dict(**counts, modules=modules, classes=classes, enums=enums,
                compiled=compiled)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('snapshot', type=Path)
    parser.add_argument('--build-dir', type=Path, default=Path('target/generated-checks'))
    parser.add_argument('--compile-rust', action='store_true')
    parser.add_argument('--compile-cpp', action='store_true')
    args = parser.parse_args()
    print(json.dumps(check(args.snapshot, args.build_dir, args.compile_rust, args.compile_cpp)))
