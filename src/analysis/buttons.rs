use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, bail, ensure};

use log::debug;

use memflow::prelude::v1::*;

use pelite::pattern;
use pelite::pe64::{Pe, PeView};

use crate::memory::{read_c_string, read_code_image};
use crate::source2::KeyButton;

pub type ButtonMap = BTreeMap<String, umem>;

pub fn buttons<P: Process + MemoryView>(process: &mut P) -> Result<ButtonMap> {
    let module = process.module_by_name("client.dll")?;

    let buf = read_code_image(process, &module)?;

    let view = PeView::from_bytes(&buf)?;

    let mut save = [0; 2];

    let scanner = view.scanner();
    // Dota 7.42: the constructor links the previous head at KeyButton::next.
    if !scanner.finds_code(
        pattern!("488b05${'} 48898388000000 488bc3 48891d${} 4883c420 5b c3"),
        &mut save,
    ) && !scanner.finds_code(pattern!("488b15${'} 4885d2 74? 488b02 4885c0"), &mut save)
    {
        bail!("missing or ambiguous button list pattern");
    }

    let list_head = process
        .read_addr64(module.base + save[1])
        .data()
        .context("reading button list head")?;

    read_buttons(process, &module, list_head)
}

fn read_buttons(
    mem: &mut impl MemoryView,
    module: &ModuleInfo,
    list_head: Address,
) -> Result<ButtonMap> {
    let mut result = ButtonMap::new();

    let mut seen = BTreeSet::new();
    let mut button_ptr = Pointer64::<KeyButton>::from(list_head);

    while !button_ptr.is_null() {
        ensure!(
            seen.insert(button_ptr.address()),
            "cycle in registered button list"
        );
        let button = mem.read_ptr(button_ptr).data()?;
        let name = read_c_string(mem, button.name.address())?;
        ensure!(!name.is_empty(), "empty registered button name");

        let state_addr = button_ptr.address() + offset_of!(KeyButton.state);

        if let Some(state_rva) = state_addr.to_umem().checked_sub(module.base.to_umem()) {
            debug!(
                "found \"{}\" at {:#X} ({} + {:#X})",
                name,
                state_addr.to_umem(),
                module.name,
                state_rva
            );

            ensure!(
                state_rva < module.size,
                "button {name} is outside {}",
                module.name
            );
            ensure!(
                !result.contains_key(&name),
                "duplicate registered button {name}"
            );
            result.insert(name, state_rva);
        }

        button_ptr = button.next;
    }

    Ok(result)
}
