# Dota 2 offset audit — client 6972

Checked on 2026-11-07 using a locally running Windows x64 Dota 2 process.
The installed `steam.inf` identifies ClientVersion 6972, SourceRevision 11209448;
the Steam installation manifest identifies build ID 26466928. These are different
version namespaces and must not be substituted for one another.

Shipped files used for static instruction/RTTI inspection (SHA-256):

- `client.dll`: `9b1fc31173611dda15fe7c26b85e3db0cf8e0b890ec261d8bf060d4904bd55d0`
- `engine2.dll`: `afd11eadb99e7296bd56d0c5b77aa979914e73a549e8f5e71cd6c1a0334d15c4`

## Corrections

| Previous output | Evidence | Change |
|---|---|---|
| `dwGameEntitySystem_highestEntityIndex` | The matched instruction at client RVA `0x17B038D` increments `[this+0x20C0]` for each creation with index below `0x4000`. The nearby getter at RVA `0x17B0330` returns this count, optionally adding another counter. This is not a maximum handle index. | Rename to `dwGameEntitySystem_entityCount`. Do not use it as an entity-list traversal upper bound. |
| `dwBuildNumber` | The old signature resolves to engine RVA `0x5B80F8`. A live read returns 48 while the installed client version is 6972. The previous online fallback hid the mismatch. | Remove this unconfirmed build-number offset; report locally installed ClientVersion and its source separately. No online fallback. |
| `dwSoundSystem_engineViewData` | The existing signature has no unique match in this build. The old code logged an error but still returned success. | Remove the unsupported field; do not guess a replacement. Absence of this signature does not prove that the underlying engine data no longer exists. |

Generated enums also needed correction: Dota's `PlayerUltimateStateOrTime_t` has
values -3 and -2. The C++ writer collapsed both to the unsigned maximum; the Rust
writer sign-extended them beyond the u32 width. C++/Rust now preserve the actual
underlying-width bit patterns (`0xFFFFFFFD`, `0xFFFFFFFE`). Rust/Zig duplicate
discriminants are detected after width conversion, including -1/u32::MAX aliases.
JSON retains the engine's original integer values.

Compiling the first generated snapshot exposed a fabricated client schema with
random field names and offsets. The hash enumerator treated pooled allocation
candidates as proof of live registrations and did not traverse committed buckets.
It now walks both bucket chains and the pool candidates, deduplicates nodes/data,
and stops cycles. The schema reader validates each candidate's owning type-scope
pointer (the module scope or its referenced `!GlobalTypes` scope) before
interpreting strings or fields. Pool enumeration is retained:
using only bucket chains loses legitimate registrations in this Dota layout.
Schema strings use strict UTF-8 reads; invalid field identifiers and
negative offsets are rejected rather than published. Rust keyword identifiers
are escaped, with `Self`/`self`/`super`/`crate` suffixed because raw identifiers
cannot represent those names.

## Retained outputs

| Output | Evidence and limits |
|---|---|
| `dwEntityList`, `dwGameEntitySystem` | Both globals resolve to the same live `CGameEntitySystem` object by RTTI. This is an alias, not two separate lists. |
| `dwGameEntitySystem_entityCount` | Static counter increment/getter above; the initial live sample returned a plausible nonnegative count. |
| `dwGameRules` | The initial loaded-map sample resolves to `C_DOTAGamerules` by RTTI. Null after leaving a map is expected. |
| `dwGlobalVars` | Unique signature and readable non-null pointer. This audit does not independently validate every field of the pointed-to global-vars structure. |
| `dwViewMatrix` | Unique signature; live data is a finite, nonzero 4x4 projection/view matrix. Camera-dependent numerical semantics are not exhaustively validated. |
| `dwViewRender` | Live target identifies as `CDOTAViewRender` by RTTI. |
| `dwNetworkGameClient` | Live target identifies as `CNetworkGameClient` by RTTI. |
| `dwNetworkGameClient_clientTickCount` | Getter at engine RVA `0x244B0`, in the `CNetworkGameClient`/base vtables, reads `+0x398`. Initial live value is a plausible tick count. |
| `dwNetworkGameClient_serverTickCount`, `dwNetworkGameClient_deltaTick` | Getter at RVA `0x244C0` and packet-processing instructions around RVA `0x83ED8` access the same `+0x24C` storage. Retained as existing aliases; the audit does not establish that they represent independent concepts. |
| `dwNetworkGameClient_isBackgroundMap` | Getter at RVA `0x5D520` belongs to `CNetworkGameClient`/base vtables and reads `+0x2C143F`; live value is boolean. A large member offset alone is not evidence of an invalid signature. |
| `dwNetworkGameClient_localPlayer` | Method at RVA `0x881A0` belongs to `CNetworkGameClient`/base vtables and reads a 32-bit slot value at `+0xF8 + slot*24`. Initial slot zero was 0. This is not a hero/controller pointer. |
| `dwNetworkGameClient_maxClients` | Getter at RVA `0x57770` appears in the client vtables, reads `+0x240`; initial live value was 64. This is engine capacity, not the number of Dota heroes. |
| `dwNetworkGameClient_signOnState` | Unique signature; initial live value was 6 in a loaded game. No full state-machine validation was performed. |
| `dwWindowWidth`, `dwWindowHeight` | Live values agree with the game's 1920x1080 framebuffer. |
| `dwInputSystem`, `dwSoundSystem` | Static object addresses agree with enumerated interfaces; live RTTI checks verify the targets. These values are direct objects rather than pointer-storage locations. |

## CS2 leftovers and schema scope

All 14 button names are read from the running Dota client's registered list, not
hardcoded from a CS2 output. `attack`, `jump`, `duck`, `reload`, and similar shared
engine names are therefore retained. This does not establish useful hero actions
for every button.

The source offset table has no CS-specific pawn, aim-punch, C4, or weapon table.
Schemas/interfaces are enumerated from the running Dota process. Shared Source 2
types are retained instead of deleting names by substring. Names resembling
`C_CS...` can also be unrelated common classes such as `C_CSequenceTransitioner2`.

The refresh is generated into an empty directory with Vulkan active. An old DX11
schema snapshot must not be mixed with the current capture merely because the
generator does not delete obsolete files from an existing output directory.

## Validation boundary

Unique byte matches establish locations, not complete field semantics. RTTI,
getter ownership, and live values provide additional evidence where recorded
above. Source inspection also found swallowed stage failures, now propagated
before output generation. Individual schema/interface entries remain best-effort.

Offline tests check version parsing without a running game. An explicitly invoked
Windows live test rescans offsets and asserts object RTTI and basic dimensions/
network values. GameRules is checked when non-null; a no-map skip is printed.
Main-menu and in-match state can differ, and a running process need not match
newly updated files until restarted. No executable bytes or process memory are
modified by these checks.

Final snapshot: 19 offsets across 4 modules, 14 buttons, 110 interfaces across
30 modules, and 16,607 classes / 867 enums across 18 schema modules. Every
generated Rust file (21) and C++ header (21) passed compilation. Snapshot checks
also reject invalid field identifiers and compare offsets across all five formats.
The initial unfiltered capture had one extra fabricated client class; it is not
part of the refreshed snapshot.
