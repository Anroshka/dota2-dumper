# Dota 2 Dumper

Dump Dota 2 offsets, interfaces, registered buttons, and Source 2 schemas on
Windows x64. Export to JSON, Rust, C++, C#, and Zig.

The pre-generated snapshot covers **Dota 2 patch 7.42, client 6985** with Vulkan.

[Download for Windows](https://github.com/Anroshka/dota2-dumper/releases/latest/download/dota2-dumper.exe) · [Download the 6985 snapshot](https://github.com/Anroshka/dota2-dumper/releases/download/v0.2.1/dota2-dumper-output-6985.zip) · [Browse the JSON offsets](output/offsets.json)

## Quick start

1. Launch Dota 2. The main menu is enough for a basic capture.
2. Open PowerShell in the folder containing the executable and run this command.

   ```powershell
   .\dota2-dumper.exe -o output -vv
   ```

Build from source with `cargo build --release`. The current build was validated with Rust 1.96.1.

## Usage

_Note:_ If you run the executable without specifying an optional memflow connector name, it will automatically use the
[memflow-native](https://github.com/memflow/memflow-native) OS layer to read the memory of the game process. If you
wish to use an existing memflow connector instead, such as **pcileech** or **kvm**, you can pass the `connector` and
optional `connector-args` arguments to the program. These connectors can be installed and managed using
the [memflowup](https://github.com/memflow/memflowup) tool.

E.g (for pcileech). `dota2-dumper -c pcileech -a :device=FPGA -vv`

Certain connectors, such as the [kvm](https://github.com/memflow/memflow-kvm) connector on Linux or
the [pcileech](https://github.com/memflow/memflow-pcileech) / [winio](https://github.com/a2x/memflow-winio)
connectors on Windows, require elevated privileges to work. So either run the `dota2-dumper` executable with `sudo` on
Linux or as an administrator on Windows.

### Available Arguments

- `-c, --connector <connector>`: The name of the memflow connector to use.
- `-a, --connector-args <connector-args>`: Additional arguments to pass to the memflow connector.
- `-f, --file-types <file-types>`: The types of files to generate. Default: `cs`, `hpp`,  `json`, `rs`, `zig`.
- `-i, --indent-size <indent-size>`: The number of spaces to use per indentation level. Default: `4`.
- `-o, --output <output>`: The output directory to write the generated files to. Default: `output`.
- `-p, --process-name <process-name>`: The name of the game process. Default: `dota2.exe`.
- `-v...`: Increase logging verbosity. Can be specified multiple times.
- `-h, --help`: Print help.
- `-V, --version`: Print version.

## Snapshot notes

The 6985 snapshot was captured from SourceRevision 11258601 on Windows x64 with Vulkan.
The 7.42 world-to-screen path reads the projection matrix from a view object.
The old static `dwViewMatrix` signature does not match this build, so the
6985 snapshot omits that offset. Older clients with a unique match still emit it.

Interface output contains module-relative addresses for factories that return a
static object directly. The previous `VMixEditTool001` entry was outside
`soundsystem.dll` and has been removed. `dwSoundSystem` is derived from the
verified static `SoundSystem001` registration.

## Offset semantics and build information

Module offsets are relative to the named DLL. Names containing an object suffix
(for example `dwNetworkGameClient_clientTickCount`) are offsets **inside that object**,
not DLL-relative addresses.

- `dwEntityList` and `dwGameEntitySystem` both identify a pointer to `CGameEntitySystem` in
  the validated Dota build. They are aliases, not two different lists.
- `dwInputSystem` and `dwSoundSystem` identify the static interface objects directly;
  do not add an extra pointer dereference. `dwGameRules`, `dwViewRender`, and
  `dwNetworkGameClient` identify pointer storage.
- `dwGameEntitySystem_entityCount` counts entities; it is **not** a highest entity index.
  The old, incorrectly named `dwGameEntitySystem_highestEntityIndex` was removed.
- `dwNetworkGameClient_localPlayer` is the first local-player slot value, not a Dota
  hero/controller pointer. The engine accesses additional slots with a 24-byte stride.
- The legacy `dwNetworkGameClient_deltaTick` and `dwNetworkGameClient_serverTickCount`
  signatures currently resolve to the same storage. Do not treat them as independent counters.
- `dwBuildNumber` was removed: its old signature reads `48` in client version `6972`,
  not the Dota client version. Native captures read `ClientVersion` from the installed
  game's `steam.inf`; `info.json` records `build_number_source: "local_steam_inf"`.
  This describes the installed files, not a verification that a running process was
  restarted after an update. Remote connector captures or missing metadata use `null`.
  The dumper does not fetch the latest online version and label it as the captured build.
- `dwSoundSystem_engineViewData` was removed because its signature is not confirmed
  in the audited 6972 build.

Buttons such as `jump`, `duck`, and `reload` are enumerated from Dota's registered
Source 2 button list. Their presence does not mean that the corresponding action is
useful to a Dota hero. Shared Source 2 classes are also kept when registered by Dota;
they are not filtered by CS-like names.

Unsigned enum output preserves the underlying-width bit pattern, including Dota
states such as -2/-3 and high-bit flags. Rust/Zig enum aliases that become equal
after conversion are emitted once; JSON retains all original names and values.

Missing or ambiguous required signatures, incomplete schema reads, and errors
reading a registered interface stop the capture. The dumper reports the affected
module when a schema or interface read fails.

The dumper writes a complete snapshot to a temporary directory and then replaces
the requested output directory. An existing directory may contain only files from
a previous dump. On a retry, the dumper restores the previous snapshot if a
replacement was interrupted after the old directory was moved aside.
Schema files depend on loaded modules, including the active renderer. The
checked-in snapshot uses Vulkan.

The native backend and live checks are validated on Windows x64. The current scanner
reads PE images; native Linux ELF Dota support is not implemented.

## Running Tests

Run offline tests with `cargo test`. Live tests are ignored by default so CI does not
need a running game. On Windows, load a match and run:

```text
cargo test --release -- --ignored --nocapture
```

The live test rescans the running process and checks object targets using RTTI and
checks basic network/window values. It does not trust the checked-in snapshot.

Check snapshot timestamps, duplicate names, module ownership, and values across
formats for offsets, buttons, interfaces, schema fields, and enum members. Compile every Rust/C++ output
file with the installed `rustc` and `g++` compilers:

```text
python scripts/check_generated.py output --compile-rust --compile-cpp
```

Omit compiler flags to run only the snapshot consistency checks. CI checks the
snapshot and compiles its Rust output as well as running offline unit tests.

## Credits & Acknowledgments

This project is a Dota 2 port based on the original [cs2-dumper](https://github.com/a2x/cs2-dumper) created by [@a2x](https://github.com/a2x). Special thanks to the original author for the Source 2 memory dumper architecture.

## License

Licensed under the MIT license ([LICENSE](./LICENSE)).
