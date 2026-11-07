# dota2-dumper

An external offset/interface/schema dumper for the Windows x64 build of Dota 2. Powered
by [memflow](https://github.com/memflow/memflow).

## Getting Started

You can download the latest release or compile it yourself.
Use a current stable Rust toolchain; this refresh was validated with Rust 1.96.1.

## Usage

1. Ensure the game is running (Being in the main menu should suffice).
2. Run the `dota2-dumper` executable.

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
  in the current Dota build.

Buttons such as `jump`, `duck`, and `reload` are enumerated from Dota's registered
Source 2 button list. Their presence does not mean that the corresponding action is
useful to a Dota hero. Shared Source 2 classes are also kept when registered by Dota;
they are not filtered by CS-like names.

Unsigned enum output preserves the underlying-width bit pattern, including Dota
states such as -2/-3 and high-bit flags. Rust/Zig enum aliases that become equal
after conversion are emitted once; JSON retains all original names and values.

A missing/ambiguous required offset signature, failed analysis stage, or missing
client schema now stops the capture before generated files are written. Field-level
schema reads and interface enumeration remain best-effort; a successful capture is
not proof that every field's semantics have been independently verified.

Generate each refresh into a **new directory**. Schema files depend on loaded modules
(including the active renderer); old files in an existing output directory are not
automatically deleted. The checked-in snapshot uses Vulkan, so it contains
`rendersystemvulkan_dll.*` rather than stale `rendersystemdx11_dll.*` files.

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

Check generated fields, cross-format offsets, and compile every Rust/C++ output
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
