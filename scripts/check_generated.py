"""Check a snapshot without running Dota. Optional Rust/C++ compilation needs compilers."""
import argparse
import json
from pathlib import Path
import re
import shutil
import subprocess


def check(root, build_dir, compile_rust=False, compile_cpp=False):
    offsets = json.loads((root / 'offsets.json').read_text(encoding='utf-8'))
    expected = {name: value for module in offsets.values() for name, value in module.items()}
    for extension in ('cs', 'hpp', 'rs', 'zig'):
        text = (root / f'offsets.{extension}').read_text(encoding='utf-8')
        actual = {name: int(value, 16) for name, value in
                  re.findall(r'(dw\w+)[^\n=]*=\s*(0x[0-9A-Fa-f]+)', text)}
        if actual != expected:
            raise ValueError(f'offsets.{extension} disagrees with offsets.json')

    modules = classes = enums = 0
    for path in root.glob('*_dll.json'):
        document = json.loads(path.read_text(encoding='utf-8'))
        if len(document) != 1:
            raise ValueError(f'{path}: expected one schema module')
        body = next(iter(document.values()))
        modules += 1
        classes += len(body['classes'])
        enums += len(body['enums'])
        for name, cls in body['classes'].items():
            for field, offset in cls['fields'].items():
                if not re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*', field) or offset < 0:
                    raise ValueError(f'{path}: invalid schema field in {name}: {field!r}={offset}')
        for extension in ('cs', 'hpp', 'rs', 'zig'):
            if not path.with_suffix('.' + extension).is_file():
                raise ValueError(f'missing {path.with_suffix("." + extension)}')
    if not modules:
        raise ValueError('no schema modules found')

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
    return dict(offsets=len(expected), modules=modules, classes=classes, enums=enums,
                compiled=compiled)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('snapshot', type=Path)
    parser.add_argument('--build-dir', type=Path, default=Path('target/generated-checks'))
    parser.add_argument('--compile-rust', action='store_true')
    parser.add_argument('--compile-cpp', action='store_true')
    args = parser.parse_args()
    print(json.dumps(check(args.snapshot, args.build_dir, args.compile_rust, args.compile_cpp)))
