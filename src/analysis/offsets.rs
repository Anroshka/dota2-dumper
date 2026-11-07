use std::collections::BTreeMap;

use anyhow::{Context, Result, bail};

use log::debug;

use memflow::prelude::v1::*;

use pelite::pattern;
use pelite::pattern::{Atom, save_len};
use pelite::pe64::{Pe, PeView, Rva};

use phf::{Map, phf_map};

pub type OffsetMap = BTreeMap<String, BTreeMap<String, Rva>>;

macro_rules! pattern_map {
    ($($module:ident => {
        $($name:expr => $pattern:expr $(=> $callback:expr)?),+ $(,)?
    }),+ $(,)?) => {
        $(
            mod $module {
                use super::*;

                pub(super) const PATTERNS: Map<
                    &'static str,
                    (
                        &'static [Atom],
                        Option<fn(&PeView, &mut BTreeMap<String, Rva>, Rva)>,
                    ),
                > = phf_map! {
                    $($name => ($pattern, $($callback)?)),+
                };

                pub fn offsets(view: PeView<'_>) -> Result<BTreeMap<String, Rva>> {
                    let mut map = BTreeMap::new();

                    for (&name, (pat, callback)) in &PATTERNS {
                        let mut save = vec![0; save_len(pat)];

                        if !view.scanner().finds_code(pat, &mut save) {
                            bail!("missing or ambiguous pattern: {}.dll::{}", stringify!($module), name);
                        }

                        let rva = save[1];

                        map.insert(name.to_string(), rva);

                        if let Some(callback) = callback {
                            callback(&view, &mut map, rva);
                        }
                    }

                    for (name, value) in &map {
                        if let Some((owner, _)) = name.split_once('_') {
                            debug!("found field \"{}\" at {} + {:#X}", name, owner, value);
                        } else {
                            debug!(
                                "found \"{}\" at {:#X} ({}.dll + {:#X})",
                                name,
                                *value as u64 + view.optional_header().ImageBase,
                                stringify!($module),
                                value
                            );
                        }
                    }

                    Ok(map)
                }
            }
        )+
    };
}

pattern_map! {
    client => {
        "dwEntityList" => pattern!("48890d${'} e9${} cc") => None,
        "dwGameEntitySystem" => pattern!("488b1d${'} 48891d[4] 4c63b3") => None,
        // Incremented on entity creation; this is a count, not the highest handle index.
        "dwGameEntitySystem_entityCount" => pattern!("ff81u4 4885d2") => None,
        "dwGameRules" => pattern!("f6c1010f85${} 4c8b05${'} 4d85") => None,
        "dwGlobalVars" => pattern!("488915${'} 488942") => None,
        "dwViewMatrix" => pattern!("488d0d${'} 48c1e006") => None,
        "dwViewRender" => pattern!("488905${'} 488bc8 4885c0") => None,
    },
    engine2 => {
        "dwNetworkGameClient" => pattern!("48893d${'} ff87") => None,
        "dwNetworkGameClient_clientTickCount" => pattern!("8b81u4 c3 cccccccccccccccccc 8b81${} c3 cccccccccccccccccc 83b9") => None,
        "dwNetworkGameClient_deltaTick" => pattern!("4c8db7u4 4c897c24") => None,
        "dwNetworkGameClient_isBackgroundMap" => pattern!("0fb681u4 c3 cccccccccccccccc 0fb681${} c3 cccccccccccccccc 4883ec") => None,
        "dwNetworkGameClient_localPlayer" => pattern!("428b94d3u4 5b 49ffe3 32c0 5b c3 cccccccccccccccc 4053") => None,
        "dwNetworkGameClient_maxClients" => pattern!("8b81u4 c3????????? 8b81[4] c3????????? 8b81") => None,
        "dwNetworkGameClient_serverTickCount" => pattern!("8b81u4 c3 cccccccccccccccccc 83b9") => None,
        "dwNetworkGameClient_signOnState" => pattern!("448b81u4 488d0d") => None,
        "dwWindowHeight" => pattern!("8b05${'} 8903") => None,
        "dwWindowWidth" => pattern!("8b05${'} 8907") => None,
    },
    input_system => {
        "dwInputSystem" => pattern!("488905${'} 33c0") => None,
    },
    soundsystem => {
        "dwSoundSystem" => pattern!("488d0d${'} e8${} 488b0d${} [3] 4c8b82") => None,
    },
}

pub fn offsets<P: Process + MemoryView>(process: &mut P) -> Result<OffsetMap> {
    let mut map = BTreeMap::new();

    let modules: [(&str, fn(PeView) -> Result<BTreeMap<String, u32>>); 4] = [
        ("client.dll", client::offsets),
        ("engine2.dll", engine2::offsets),
        ("inputsystem.dll", input_system::offsets),
        ("soundsystem.dll", soundsystem::offsets),
    ];

    for (module_name, offsets) in &modules {
        let module = process
            .module_by_name(module_name)
            .with_context(|| format!("required module {module_name} not found"))?;
        let buf = process
            .read_raw(module.base, module.size as _)
            .data_part()
            .with_context(|| format!("failed to read {module_name}"))?;
        let view =
            PeView::from_bytes(&buf).with_context(|| format!("invalid PE image: {module_name}"))?;

        map.insert(module_name.to_string(), offsets(view)?);
    }

    Ok(map)
}

#[cfg(all(test, windows))]
mod tests {
    use std::sync::Once;

    use simplelog::*;

    use super::*;

    #[test]
    #[ignore = "requires Dota 2; GameRules RTTI is checked only while a map is loaded"]
    fn live_offset_targets() -> Result<()> {
        let mut process = setup()?;
        // Scan the running image, not a potentially stale output/offsets.json.
        let found = offsets(&mut process)?;
        assert!(!found["client.dll"].contains_key("dwGameEntitySystem_highestEntityIndex"));
        assert!(!found["engine2.dll"].contains_key("dwBuildNumber"));
        assert!(!found["soundsystem.dll"].contains_key("dwSoundSystem_engineViewData"));
        for (module_name, offset_name, class_name, indirect) in [
            (
                "client.dll",
                "dwEntityList",
                ".?AVCGameEntitySystem@@",
                true,
            ),
            (
                "client.dll",
                "dwGameEntitySystem",
                ".?AVCGameEntitySystem@@",
                true,
            ),
            ("client.dll", "dwGameRules", ".?AVC_DOTAGamerules@@", true),
            ("client.dll", "dwViewRender", ".?AVCDOTAViewRender@@", true),
            (
                "engine2.dll",
                "dwNetworkGameClient",
                ".?AVCNetworkGameClient@@",
                true,
            ),
            (
                "inputsystem.dll",
                "dwInputSystem",
                ".?AVCInputSystem@@",
                false,
            ),
            (
                "soundsystem.dll",
                "dwSoundSystem",
                ".?AVCSoundSystem@@",
                false,
            ),
        ] {
            let module = process.module_by_name(module_name)?;
            let address = module.base + found[module_name][offset_name];
            let object = if indirect {
                process
                    .read_addr64(address)
                    .data_part()
                    .with_context(|| format!("pointer read failed: {module_name}::{offset_name}"))?
            } else {
                address
            };
            // GameRules is intentionally null after leaving a match.
            if offset_name == "dwGameRules" && object.is_null() {
                eprintln!("GameRules is null (no map); its RTTI check is not applicable");
                continue;
            }
            let actual = (|| -> Result<String> {
                let vtable = process.read_addr64(object).data_part()?;
                let locator = process.read_addr64(vtable - 8).data_part()?;
                let descriptor: u32 = process.read(locator + 12).data_part()?;
                Ok(process
                    .read_utf8_lossy(module.base + descriptor + 16, 128)
                    .data_part()?)
            })()
            .with_context(|| format!("RTTI check failed: {module_name}::{offset_name}"))?;
            assert_eq!(actual, class_name, "{module_name}::{offset_name}");
        }
        let module = process.module_by_name("engine2.dll")?;
        for name in ["dwWindowWidth", "dwWindowHeight"] {
            let pixels: u32 = process
                .read(module.base + found["engine2.dll"][name])
                .data_part()?;
            assert!((1..=32768).contains(&pixels), "invalid {name}: {pixels}");
        }
        let network = process
            .read_addr64(module.base + found["engine2.dll"]["dwNetworkGameClient"])
            .data_part()?;
        let background: u8 = process
            .read(network + found["engine2.dll"]["dwNetworkGameClient_isBackgroundMap"])
            .data_part()?;
        assert!(background <= 1);
        let max_clients: u32 = process
            .read(network + found["engine2.dll"]["dwNetworkGameClient_maxClients"])
            .data_part()?;
        assert!((1..=64).contains(&max_clients));
        Ok(())
    }

    fn setup() -> Result<IntoProcessInstanceArcBox<'static>> {
        static LOGGER: Once = Once::new();

        LOGGER.call_once(|| {
            SimpleLogger::init(LevelFilter::Trace, Config::default()).ok();
        });

        let os = memflow_native::create_os(&OsArgs::default(), LibArc::default())?;

        let process = os.into_process_by_name("dota2.exe")?;

        Ok(process)
    }
}
