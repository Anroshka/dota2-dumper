use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, ensure};

use log::debug;

use memflow::prelude::v1::*;

use pelite::pe64::exports::Export;
use pelite::pe64::{Pe, PeView};

use crate::memory::{address, read_c_string};
use crate::source2::InterfaceReg;

pub type InterfaceMap = BTreeMap<String, BTreeMap<String, umem>>;

pub fn interfaces<P: Process + MemoryView>(process: &mut P) -> Result<InterfaceMap> {
    let mut result = InterfaceMap::new();
    for module in process.module_list()? {
        let name = module.name.to_string();
        let entries = module_interfaces(process, &module)
            .with_context(|| format!("reading interfaces in {name}"))?;
        if !entries.is_empty() {
            ensure!(
                !result.contains_key(&name),
                "duplicate interface module {name}"
            );
            result.insert(name, entries);
        }
    }
    Ok(result)
}

fn module_interfaces(
    process: &mut impl MemoryView,
    module: &ModuleInfo,
) -> Result<BTreeMap<String, umem>> {
    // Some loaded images contain unmapped pages outside the export directory.
    let buf = process
        .read_raw(module.base, module.size as _)
        .data_part()?;
    let view = PeView::from_bytes(&buf)?;
    let exports = match view.exports() {
        Ok(exports) => exports.by()?,
        Err(pelite::Error::Null) => return Ok(BTreeMap::new()),
        Err(err) => return Err(err.into()),
    };
    let symbol = match exports.name("CreateInterface") {
        Ok(Export::Symbol(symbol)) => symbol,
        Ok(_) => {
            log::warn!("skipping forwarded CreateInterface in {}", module.name);
            return Ok(BTreeMap::new());
        }
        Err(pelite::Error::Null) => return Ok(BTreeMap::new()),
        Err(err) => return Err(err.into()),
    };
    let entry = module.base + symbol;
    let opcode: [u8; 3] = process.read(entry).data()?;
    if !matches!(opcode, [0x4C | 0x48, 0x8B, op] if op & 0xC7 == 0x05) {
        log::warn!("unsupported CreateInterface entry point in {}", module.name);
        return Ok(BTreeMap::new());
    }
    let list_ptr = address::resolve_rip(process, entry)?;
    let list_head = process.read_addr64(list_ptr).data()?;
    read_interfaces(process, module, list_head)
}

fn read_interfaces(
    mem: &mut impl MemoryView,
    module: &ModuleInfo,
    list_head: Address,
) -> Result<BTreeMap<String, umem>> {
    let mut result = BTreeMap::new();

    let mut seen = BTreeSet::new();
    let mut reg_ptr = Pointer64::<InterfaceReg>::from(list_head);

    while !reg_ptr.is_null() {
        ensure!(
            seen.insert(reg_ptr.address()),
            "cycle in {} interface list",
            module.name
        );
        let reg = mem.read_ptr(reg_ptr).data()?;
        reg_ptr = reg.next;
        let name = read_c_string(mem, reg.name.address())?;
        ensure!(!name.is_empty(), "empty interface name");
        let function = reg.create_fn.address();
        let bytes: [u8; 8] = mem.read(function).data()?;
        if !matches!(bytes, [0x48, 0x8D, 0x05, .., 0xC3]) {
            debug!("skipping non-static interface {}::{name}", module.name);
            continue;
        }

        let instance_addr = address::resolve_rip(mem, function)?;
        if let Some(instance_rva) = instance_addr
            .to_umem()
            .checked_sub(module.base.to_umem())
            .filter(|&rva| rva < module.size)
        {
            debug!(
                "found \"{}\" at {:#X} ({} + {:#X})",
                name,
                instance_addr.to_umem(),
                module.name,
                instance_rva
            );

            ensure!(
                !result.contains_key(&name),
                "duplicate interface {}::{name}",
                module.name
            );
            result.insert(name, instance_rva);
        }
    }

    Ok(result)
}
