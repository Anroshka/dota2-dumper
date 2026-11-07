pub use buttons::*;
pub use interfaces::*;
pub use offsets::*;
pub use schemas::*;

use anyhow::{Context, Result, ensure};

use log::info;

use memflow::prelude::v1::*;

mod buttons;
mod interfaces;
mod offsets;
mod schemas;

#[derive(Debug)]
pub struct AnalysisResult {
    pub buttons: ButtonMap,
    pub interfaces: InterfaceMap,
    pub offsets: OffsetMap,
    pub schemas: SchemaMap,
}

pub fn analyze_all<P: Process + MemoryView>(process: &mut P) -> Result<AnalysisResult> {
    let buttons = buttons(process).context("button analysis failed")?;
    ensure!(!buttons.is_empty(), "no registered buttons found");

    info!("found {} buttons", buttons.len());

    let interfaces = interfaces(process).context("interface analysis failed")?;
    ensure!(!interfaces.is_empty(), "no interfaces found");

    info!(
        "found {} interfaces across {} modules",
        interfaces
            .iter()
            .map(|(_, ifaces)| ifaces.len())
            .sum::<usize>(),
        interfaces.len()
    );

    let offsets = offsets(process).context("offset analysis failed")?;

    info!(
        "found {} offsets across {} modules",
        offsets
            .iter()
            .map(|(_, offsets)| offsets.len())
            .sum::<usize>(),
        offsets.len()
    );

    let schemas = schemas(process).context("schema analysis failed")?;
    ensure!(
        schemas.contains_key("client.dll"),
        "client.dll schemas missing"
    );

    let (class_count, enum_count) =
        schemas
            .values()
            .fold((0, 0), |(classes, enums), (class_vec, enum_vec)| {
                (classes + class_vec.len(), enums + enum_vec.len())
            });

    info!(
        "found {} classes and {} enums across {} modules",
        class_count,
        enum_count,
        schemas.len()
    );

    Ok(AnalysisResult {
        buttons,
        interfaces,
        offsets,
        schemas,
    })
}
