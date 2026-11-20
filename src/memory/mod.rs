pub mod address;

use anyhow::{Context, Result, ensure};
use memflow::prelude::v1::*;

pub fn read_c_string(mem: &mut impl MemoryView, mut address: Address) -> Result<String> {
    ensure!(!address.is_null(), "null string pointer");
    let mut bytes = Vec::new();
    let mut buf = [0u8; 128];
    while bytes.len() < 4096 {
        // Stop at a page boundary so a valid terminator before an unreadable page is accepted.
        let to_page_end = 4096 - (address.to_umem() as usize % 4096);
        let count = buf.len().min(to_page_end).min(4096 - bytes.len());
        let chunk = &mut buf[..count];
        mem.read_raw_into(address, chunk)
            .data()
            .with_context(|| format!("cannot read string at {address:?}"))?;
        if let Some(end) = chunk.iter().position(|&byte| byte == 0) {
            bytes.extend_from_slice(&chunk[..end]);
            return Ok(String::from_utf8(bytes)?);
        }
        bytes.extend_from_slice(chunk);
        address += count;
    }
    anyhow::bail!("unterminated string");
}

pub fn read_code_image(mem: &mut impl MemoryView, module: &ModuleInfo) -> Result<Vec<u8>> {
    use pelite::pe64::{Pe, PeView};

    let mut bytes = match mem.read_raw(module.base, module.size as _) {
        Ok(bytes) => return Ok(bytes),
        Err(PartialError::PartialVirtualRead(bytes)) => bytes,
        Err(err) => return Err(err.into()),
    };
    // The loader may discard non-code sections. Only the headers and the scanner's
    // code range have to be completely readable for a signature scan.
    let view = PeView::from_bytes(&bytes)?;
    let header_size = view.optional_header().SizeOfHeaders as usize;
    let code = view.headers().code_range();
    let header = bytes
        .get_mut(..header_size)
        .context("PE headers outside image")?;
    mem.read_raw_into(module.base, header)
        .data()
        .context("reading PE headers")?;
    let code_bytes = bytes
        .get_mut(code.start as usize..code.end as usize)
        .context("PE code range outside image")?;
    mem.read_raw_into(module.base + code.start, code_bytes)
        .data()
        .context("reading PE code section")?;
    Ok(bytes)
}
