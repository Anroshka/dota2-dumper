use std::fmt::{self, Write};
use std::fs;
use std::path::Path;

use anyhow::{Result, anyhow};

use chrono::{DateTime, Utc};

use memflow::prelude::v1::*;

use serde_json::json;

use formatter::Formatter;

use crate::analysis::*;

mod buttons;
mod formatter;
mod interfaces;
mod offsets;
mod schemas;

enum Item<'a> {
    Buttons(&'a ButtonMap),
    Interfaces(&'a InterfaceMap),
    Offsets(&'a OffsetMap),
    Schemas(&'a SchemaMap),
}

impl<'a> Item<'a> {
    fn write(&self, fmt: &mut Formatter<'a>, file_type: &str) -> fmt::Result {
        match file_type {
            "cs" => self.write_cs(fmt),
            "hpp" => self.write_hpp(fmt),
            "json" => self.write_json(fmt),
            "rs" => self.write_rs(fmt),
            "zig" => self.write_zig(fmt),
            _ => unimplemented!(),
        }
    }
}

trait CodeWriter {
    fn write_cs(&self, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn write_hpp(&self, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn write_json(&self, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn write_rs(&self, fmt: &mut Formatter<'_>) -> fmt::Result;
    fn write_zig(&self, fmt: &mut Formatter<'_>) -> fmt::Result;
}

impl<'a> CodeWriter for Item<'a> {
    fn write_cs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Item::Buttons(buttons) => buttons.write_cs(fmt),
            Item::Interfaces(ifaces) => ifaces.write_cs(fmt),
            Item::Offsets(offsets) => offsets.write_cs(fmt),
            Item::Schemas(schemas) => schemas.write_cs(fmt),
        }
    }

    fn write_hpp(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Item::Buttons(buttons) => buttons.write_hpp(fmt),
            Item::Interfaces(ifaces) => ifaces.write_hpp(fmt),
            Item::Offsets(offsets) => offsets.write_hpp(fmt),
            Item::Schemas(schemas) => schemas.write_hpp(fmt),
        }
    }

    fn write_json(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Item::Buttons(buttons) => buttons.write_json(fmt),
            Item::Interfaces(ifaces) => ifaces.write_json(fmt),
            Item::Offsets(offsets) => offsets.write_json(fmt),
            Item::Schemas(schemas) => schemas.write_json(fmt),
        }
    }

    fn write_rs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Item::Buttons(buttons) => buttons.write_rs(fmt),
            Item::Interfaces(ifaces) => ifaces.write_rs(fmt),
            Item::Offsets(offsets) => offsets.write_rs(fmt),
            Item::Schemas(schemas) => schemas.write_rs(fmt),
        }
    }

    fn write_zig(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Item::Buttons(buttons) => buttons.write_zig(fmt),
            Item::Interfaces(ifaces) => ifaces.write_zig(fmt),
            Item::Offsets(offsets) => offsets.write_zig(fmt),
            Item::Schemas(schemas) => schemas.write_zig(fmt),
        }
    }
}

pub struct Output<'a> {
    file_types: &'a [String],
    indent_size: usize,
    out_dir: &'a Path,
    result: &'a AnalysisResult,
    timestamp: DateTime<Utc>,
    read_local_files: bool,
}

impl<'a> Output<'a> {
    pub fn new(
        file_types: &'a [String],
        indent_size: usize,
        out_dir: &'a Path,
        result: &'a AnalysisResult,
        read_local_files: bool,
    ) -> Result<Self> {
        fs::create_dir_all(&out_dir)?;

        Ok(Self {
            file_types,
            indent_size,
            out_dir,
            result,
            timestamp: Utc::now(),
            read_local_files,
        })
    }

    pub fn dump_all<P: MemoryView + Process>(&self, process: &mut P) -> Result<()> {
        let items = [
            ("buttons", Item::Buttons(&self.result.buttons)),
            ("interfaces", Item::Interfaces(&self.result.interfaces)),
            ("offsets", Item::Offsets(&self.result.offsets)),
        ];

        for (file_name, item) in &items {
            self.dump_item(file_name, item)?;
        }

        self.dump_schemas()?;
        self.dump_info(process)?;

        Ok(())
    }

    fn dump_info<P: MemoryView + Process>(&self, process: &mut P) -> Result<()> {
        let file_path = self.out_dir.join("info.json");

        // The old dwBuildNumber signature reads 48 in build 6972, not ClientVersion.
        // Do not disguise a failed read with the latest version of a different build
        // fetched from Steam. Native captures can identify the installed build locally.
        let build_number = if self.read_local_files {
            process
                .module_by_name("engine2.dll")
                .ok()
                .and_then(|module| {
                    let path = std::path::PathBuf::from(module.path.to_string());
                    let game_dir = path.parent()?.parent()?.parent()?;
                    let content =
                        fs::read_to_string(game_dir.join("dota").join("steam.inf")).ok()?;
                    parse_client_version(&content)
                })
        } else {
            None
        };
        if build_number.is_none() {
            log::warn!("installed ClientVersion unavailable; info.json build_number will be null");
        }

        let content = serde_json::to_string_pretty(&json!({
            "timestamp": self.timestamp.to_rfc3339(),
            "build_number": build_number,
            "build_number_source": build_number.map(|_| "local_steam_inf"),
        }))?;

        fs::write(&file_path, &content)?;

        Ok(())
    }

    fn dump_item(&self, file_name: &str, item: &Item) -> Result<()> {
        for file_type in self.file_types {
            let mut out = String::new();
            let mut fmt = Formatter::new(&mut out, self.indent_size);

            if file_type != "json" {
                self.write_banner(&mut fmt)?;
            }

            item.write(&mut fmt, file_type)?;

            let file_path = self.out_dir.join(format!("{}.{}", file_name, file_type));

            fs::write(&file_path, out)?;
        }

        Ok(())
    }

    fn dump_schemas(&self) -> Result<()> {
        for (module_name, (classes, enums)) in &self.result.schemas {
            let map = SchemaMap::from([(module_name.clone(), (classes.clone(), enums.clone()))]);

            self.dump_item(&slugify(&module_name), &Item::Schemas(&map))?;
        }

        Ok(())
    }

    fn write_banner(&self, fmt: &mut Formatter<'_>) -> Result<()> {
        writeln!(
            fmt,
            "// Generated using https://github.com/Anroshka/dota2-dumper"
        )?;
        writeln!(fmt, "// {}\n", self.timestamp)?;

        Ok(())
    }
}

#[inline]
fn slugify(input: &str) -> String {
    input.replace(|c: char| !c.is_alphanumeric(), "_")
}

fn rust_ident(input: &str) -> String {
    match input {
        "Self" | "self" | "super" | "crate" => format!("{input}_"),
        "as" | "async" | "await" | "break" | "const" | "continue" | "dyn" | "else" | "enum"
        | "extern" | "false" | "fn" | "for" | "gen" | "if" | "impl" | "in" | "let" | "loop"
        | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "static" | "struct"
        | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" | "abstract"
        | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof"
        | "unsized" | "virtual" | "yield" | "try" => format!("r#{input}"),
        _ => input.to_string(),
    }
}

fn parse_client_version(content: &str) -> Option<u32> {
    let value = |key: &str| {
        content
            .lines()
            .filter_map(|line| line.split_once('='))
            .find_map(|(name, value)| (name.trim() == key).then_some(value.trim()))
    };
    if value("appID") != Some("570") {
        return None;
    }
    value("ClientVersion")?
        .parse::<u32>()
        .ok()
        .filter(|version| *version > 0)
}

#[cfg(test)]
mod tests {
    use super::{parse_client_version, rust_ident};

    #[test]
    fn escapes_rust_keywords_in_engine_names() {
        assert_eq!(rust_ident("type"), "r#type");
        assert_eq!(rust_ident("gen"), "r#gen");
        assert_eq!(rust_ident("Self"), "Self_");
        assert_eq!(rust_ident("m_name"), "m_name");
    }

    #[test]
    fn identifies_installed_dota_version_instead_of_server_or_steam_build_id() {
        assert_eq!(
            parse_client_version("ClientVersion=6972\r\nServerVersion=6971\r\nappID=570\r\n"),
            Some(6972)
        );
        assert_eq!(
            parse_client_version("ClientVersion=6972\nappID=730\n"),
            None
        );
        assert_eq!(
            parse_client_version("appID=570\nClientVersion=not-a-version\n"),
            None
        );
        assert_eq!(parse_client_version("appID=570\nClientVersion=0\n"), None);
        assert_eq!(
            parse_client_version("appID=570\nServerVersion=6972\n"),
            None
        );
    }
}

#[inline]
fn zig_ident(input: &str) -> String {
    if is_zig_identifier(input) && !is_zig_keyword(input) {
        input.to_string()
    } else {
        let escaped = input.replace('\\', "\\\\").replace('"', "\\\"");

        format!("@\"{}\"", escaped)
    }
}

#[inline]
fn is_zig_identifier(input: &str) -> bool {
    let mut chars = input.chars();

    match chars.next() {
        Some(c) if c == '_' || c.is_ascii_alphabetic() => {}
        _ => return false,
    }

    chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

#[inline]
fn is_zig_keyword(input: &str) -> bool {
    matches!(
        input,
        "addrspace"
            | "align"
            | "allowzero"
            | "and"
            | "anyframe"
            | "anytype"
            | "asm"
            | "async"
            | "await"
            | "break"
            | "callconv"
            | "catch"
            | "comptime"
            | "const"
            | "continue"
            | "defer"
            | "else"
            | "enum"
            | "errdefer"
            | "error"
            | "export"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "inline"
            | "linksection"
            | "noalias"
            | "noinline"
            | "nosuspend"
            | "null"
            | "opaque"
            | "or"
            | "orelse"
            | "packed"
            | "pub"
            | "resume"
            | "return"
            | "struct"
            | "suspend"
            | "switch"
            | "test"
            | "threadlocal"
            | "true"
            | "try"
            | "union"
            | "unreachable"
            | "usingnamespace"
            | "var"
            | "volatile"
            | "while"
    )
}
