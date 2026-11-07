use std::collections::{BTreeMap, HashSet};
use std::fmt::{self, Write};

use heck::{AsPascalCase, AsSnakeCase};

use serde_json::json;

use super::{CodeWriter, Formatter, SchemaMap, rust_ident, slugify, zig_ident};

use crate::analysis::ClassMetadata;

impl CodeWriter for SchemaMap {
    fn write_cs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.block("namespace Dota2Dumper.Schemas", false, |fmt| {
            for (module_name, (classes, enums)) in self {
                writeln!(fmt, "// Module: {}", module_name)?;
                writeln!(fmt, "// Class count: {}", classes.len())?;
                writeln!(fmt, "// Enum count: {}", enums.len())?;

                fmt.block(
                    &format!("public static class {}", AsPascalCase(slugify(module_name))),
                    false,
                    |fmt| {
                        for enum_ in enums {
                            let type_name = match enum_.alignment {
                                1 => "byte",
                                2 => "ushort",
                                4 => "uint",
                                8 => "ulong",
                                _ => continue,
                            };

                            writeln!(fmt, "// Alignment: {}", enum_.alignment)?;
                            writeln!(fmt, "// Member count: {}", enum_.size)?;

                            fmt.block(
                                &format!("public enum {} : {}", slugify(&enum_.name), type_name),
                                false,
                                |fmt| {
                                    let members = enum_
                                        .members
                                        .iter()
                                        .map(|member| {
                                            let formatted_value =
                                                if (0..=i32::MAX as i64).contains(&member.value) {
                                                    format!("{:#X}", member.value)
                                                } else {
                                                    format!(
                                                        "unchecked(({}){})",
                                                        type_name, member.value
                                                    )
                                                };

                                            format!("{} = {}", member.name, formatted_value)
                                        })
                                        .collect::<Vec<_>>()
                                        .join(",\n");

                                    writeln!(fmt, "{}", members)
                                },
                            )?;
                        }

                        for class in classes {
                            let parent_name = class
                                .parent_name
                                .as_deref()
                                .map(slugify)
                                .unwrap_or("None".to_string());

                            writeln!(fmt, "// Parent: {}", parent_name)?;
                            writeln!(fmt, "// Field count: {}", class.fields.len())?;

                            write_metadata(fmt, &class.metadata)?;

                            fmt.block(
                                &format!("public static class {}", slugify(&class.name)),
                                false,
                                |fmt| {
                                    for field in &class.fields {
                                        writeln!(
                                            fmt,
                                            "public const nint {} = {:#X}; // {}",
                                            field.name, field.offset, field.type_name
                                        )?;
                                    }

                                    Ok(())
                                },
                            )?;
                        }

                        Ok(())
                    },
                )?;
            }

            Ok(())
        })
    }

    fn write_hpp(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#pragma once\n")?;
        writeln!(fmt, "#include <cstddef>")?;
        writeln!(fmt, "#include <cstdint>\n")?;

        fmt.block("namespace dota2_dumper", false, |fmt| {
            fmt.block("namespace schemas", false, |fmt| {
                for (module_name, (classes, enums)) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;
                    writeln!(fmt, "// Class count: {}", classes.len())?;
                    writeln!(fmt, "// Enum count: {}", enums.len())?;

                    fmt.block(
                        &format!("namespace {}", AsSnakeCase(slugify(module_name))),
                        false,
                        |fmt| {
                            for enum_ in enums {
                                let type_name = match enum_.alignment {
                                    1 => "uint8_t",
                                    2 => "uint16_t",
                                    4 => "uint32_t",
                                    8 => "uint64_t",
                                    _ => continue,
                                };

                                writeln!(fmt, "// Alignment: {}", enum_.alignment)?;
                                writeln!(fmt, "// Member count: {}", enum_.size)?;

                                fmt.block(
                                    &format!("enum class {} : {}", slugify(&enum_.name), type_name),
                                    true,
                                    |fmt| {
                                        let members = enum_
                                            .members
                                            .iter()
                                            .map(|member| {
                                                let formatted_value = format!(
                                                    "{:#X}",
                                                    enum_member_bits(member.value, enum_.alignment)
                                                );

                                                format!("{} = {}", member.name, formatted_value)
                                            })
                                            .collect::<Vec<_>>()
                                            .join(",\n");

                                        writeln!(fmt, "{}", members)
                                    },
                                )?;
                            }

                            for class in classes {
                                let parent_name = class
                                    .parent_name
                                    .as_deref()
                                    .map(slugify)
                                    .unwrap_or("None".to_string());

                                writeln!(fmt, "// Parent: {}", parent_name)?;
                                writeln!(fmt, "// Field count: {}", class.fields.len())?;

                                write_metadata(fmt, &class.metadata)?;

                                fmt.block(
                                    &format!("namespace {}", slugify(&class.name)),
                                    false,
                                    |fmt| {
                                        for field in &class.fields {
                                            writeln!(
                                                fmt,
                                                "constexpr std::ptrdiff_t {} = {:#X}; // {}",
                                                field.name, field.offset, field.type_name
                                            )?;
                                        }

                                        Ok(())
                                    },
                                )?;
                            }

                            Ok(())
                        },
                    )?;
                }

                Ok(())
            })
        })
    }

    fn write_json(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let content: BTreeMap<_, _> = self
            .iter()
            .map(|(module_name, (classes, enums))| {
                let classes: BTreeMap<_, _> = classes
                    .iter()
                    .map(|class| {
                        let fields: BTreeMap<_, _> = class
                            .fields
                            .iter()
                            .map(|field| (&field.name, field.offset))
                            .collect();

                        let metadata: Vec<_> = class
                            .metadata
                            .iter()
                            .map(|metadata| match metadata {
                                ClassMetadata::NetworkChangeCallback { name } => json!({
                                    "type": "NetworkChangeCallback",
                                    "name": name,
                                }),
                                ClassMetadata::NetworkVarNames { name, type_name } => json!({
                                    "type": "NetworkVarNames",
                                    "name": name,
                                    "type_name": type_name,
                                }),
                                ClassMetadata::Unknown { name } => json!({
                                    "type": "Unknown",
                                    "name": name,
                                }),
                            })
                            .collect();

                        (
                            slugify(&class.name),
                            json!({
                                "parent": class.parent_name,
                                "fields": fields,
                                "metadata": metadata
                            }),
                        )
                    })
                    .collect();

                let enums: BTreeMap<_, _> = enums
                    .iter()
                    .map(|enum_| {
                        let members: BTreeMap<_, _> = enum_
                            .members
                            .iter()
                            .map(|member| (&member.name, member.value))
                            .collect();

                        let type_name = match enum_.alignment {
                            1 => "uint8",
                            2 => "uint16",
                            4 => "uint32",
                            8 => "uint64",
                            _ => "unknown",
                        };

                        (
                            slugify(&enum_.name),
                            json!({
                                "alignment": enum_.alignment,
                                "type": type_name,
                                "members": members,
                            }),
                        )
                    })
                    .collect();

                (
                    module_name,
                    json!({
                        "classes": classes,
                        "enums": enums,
                    }),
                )
            })
            .collect();

        fmt.write_str(&serde_json::to_string_pretty(&content).unwrap())
    }

    fn write_rs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            fmt,
            "#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]\n"
        )?;

        fmt.block("pub mod dota2_dumper", false, |fmt| {
            fmt.block("pub mod schemas", false, |fmt| {
                for (module_name, (classes, enums)) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;
                    writeln!(fmt, "// Class count: {}", classes.len())?;
                    writeln!(fmt, "// Enum count: {}", enums.len())?;

                    fmt.block(
                        &format!("pub mod {}", AsSnakeCase(slugify(module_name))),
                        false,
                        |fmt| {
                            for enum_ in enums {
                                let type_name = match enum_.alignment {
                                    1 => "u8",
                                    2 => "u16",
                                    4 => "u32",
                                    8 => "u64",
                                    _ => continue,
                                };

                                writeln!(fmt, "// Alignment: {}", enum_.alignment)?;
                                writeln!(fmt, "// Member count: {}", enum_.size)?;

                                fmt.block(
                                    &format!(
                                        "#[repr({})]\npub enum {}",
                                        type_name,
                                        rust_ident(&slugify(&enum_.name)),
                                    ),
                                    false,
                                    |fmt| {
                                        let mut used_values = HashSet::new();

                                        let members = enum_
                                            .members
                                            .iter()
                                            .filter_map(|member| {
                                                let bits =
                                                    enum_member_bits(member.value, enum_.alignment);
                                                // Deduplicate after narrowing: -1 and u32::MAX
                                                // describe the same discriminant in a u32 enum.
                                                if !used_values.insert(bits) {
                                                    return None;
                                                }

                                                let formatted_value = format!("{:#X}", bits);

                                                Some(format!(
                                                    "{} = {}",
                                                    rust_ident(&member.name),
                                                    formatted_value
                                                ))
                                            })
                                            .collect::<Vec<_>>()
                                            .join(",\n");

                                        writeln!(fmt, "{}", members)
                                    },
                                )?;
                            }

                            for class in classes {
                                let parent_name = class
                                    .parent_name
                                    .as_deref()
                                    .map(slugify)
                                    .unwrap_or("None".to_string());

                                writeln!(fmt, "// Parent: {}", parent_name)?;
                                writeln!(fmt, "// Field count: {}", class.fields.len())?;

                                write_metadata(fmt, &class.metadata)?;

                                fmt.block(
                                    &format!("pub mod {}", rust_ident(&slugify(&class.name))),
                                    false,
                                    |fmt| {
                                        for field in &class.fields {
                                            writeln!(
                                                fmt,
                                                "pub const {}: usize = {:#X}; // {}",
                                                rust_ident(&field.name),
                                                field.offset,
                                                field.type_name
                                            )?;
                                        }

                                        Ok(())
                                    },
                                )?;
                            }

                            Ok(())
                        },
                    )?;
                }

                Ok(())
            })
        })
    }

    fn write_zig(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.block("pub const dota2_dumper = struct", true, |fmt| {
            fmt.block("pub const schemas = struct", true, |fmt| {
                for (module_name, (classes, enums)) in self {
                    writeln!(fmt, "// Module: {}", module_name)?;
                    writeln!(fmt, "// Class count: {}", classes.len())?;
                    writeln!(fmt, "// Enum count: {}", enums.len())?;

                    let module_name = zig_ident(&AsSnakeCase(slugify(module_name)).to_string());

                    fmt.block(
                        &format!("pub const {} = struct", module_name),
                        true,
                        |fmt| {
                            for enum_ in enums {
                                let type_name = match enum_.alignment {
                                    1 => "u8",
                                    2 => "u16",
                                    4 => "u32",
                                    8 => "u64",
                                    _ => continue,
                                };

                                writeln!(fmt, "// Alignment: {}", enum_.alignment)?;
                                writeln!(fmt, "// Member count: {}", enum_.size)?;

                                let enum_name = zig_ident(&slugify(&enum_.name));

                                fmt.block(
                                    &format!("pub const {} = enum({})", enum_name, type_name),
                                    true,
                                    |fmt| {
                                        let mut used_values = HashSet::new();

                                        let members = enum_
                                            .members
                                            .iter()
                                            .filter_map(|member| {
                                                // Skip duplicate values.
                                                if !used_values.insert(enum_member_bits(
                                                    member.value,
                                                    enum_.alignment,
                                                )) {
                                                    return None;
                                                }

                                                let formatted_value = format_zig_enum_member_value(
                                                    member.value,
                                                    type_name,
                                                );

                                                Some(format!(
                                                    "{} = {}",
                                                    zig_ident(&member.name),
                                                    formatted_value
                                                ))
                                            })
                                            .collect::<Vec<_>>()
                                            .join(",\n");

                                        writeln!(fmt, "{}", members)
                                    },
                                )?;
                            }

                            for class in classes {
                                let parent_name = class
                                    .parent_name
                                    .as_deref()
                                    .map(slugify)
                                    .unwrap_or("None".to_string());

                                writeln!(fmt, "// Parent: {}", parent_name)?;
                                writeln!(fmt, "// Field count: {}", class.fields.len())?;

                                write_metadata(fmt, &class.metadata)?;

                                let class_name = zig_ident(&slugify(&class.name));

                                fmt.block(
                                    &format!("pub const {} = struct", class_name),
                                    true,
                                    |fmt| {
                                        for field in &class.fields {
                                            writeln!(
                                                fmt,
                                                "pub const {}: usize = {:#X}; // {}",
                                                zig_ident(&field.name),
                                                field.offset,
                                                field.type_name
                                            )?;
                                        }

                                        Ok(())
                                    },
                                )?;
                            }

                            Ok(())
                        },
                    )?;
                }

                Ok(())
            })
        })
    }
}

fn write_metadata(fmt: &mut Formatter<'_>, metadata: &[ClassMetadata]) -> fmt::Result {
    if metadata.is_empty() {
        return Ok(());
    }

    writeln!(fmt, "//")?;
    writeln!(fmt, "// Metadata:")?;

    for metadata in metadata {
        match metadata {
            ClassMetadata::NetworkChangeCallback { name } => {
                writeln!(fmt, "// NetworkChangeCallback: {}", name)?;
            }
            ClassMetadata::NetworkVarNames { name, type_name } => {
                writeln!(fmt, "// NetworkVarNames: {} ({})", name, type_name)?;
            }
            ClassMetadata::Unknown { name } => {
                writeln!(fmt, "// {}", name)?;
            }
        }
    }

    Ok(())
}

fn format_zig_enum_member_value(value: i64, type_name: &str) -> String {
    if value >= 0 {
        return format!("{:#X}", value);
    }

    let wrapped_value = match type_name {
        "u8" => value as u8 as u64,
        "u16" => value as u16 as u64,
        "u32" => value as u32 as u64,
        "u64" => value as u64,
        _ => 0,
    };

    format!("{:#X}", wrapped_value)
}

fn enum_member_bits(value: i64, width: u8) -> u64 {
    match width {
        1 => value as u8 as u64,
        2 => value as u16 as u64,
        4 => value as u32 as u64,
        8 => value as u64,
        _ => unreachable!("unsupported enum width"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::{Enum, EnumMember};

    #[test]
    fn serializes_negative_states_and_high_bit_flags_without_corruption() {
        let schema = SchemaMap::from([(
            "client.dll".to_string(),
            (
                Vec::new(),
                vec![Enum {
                    name: "PlayerUltimateStateOrTime_t".to_string(),
                    alignment: 4,
                    size: 5,
                    members: [
                        ("HIDDEN", -3),
                        ("NOT_LEVELED", -2),
                        ("MAX", -1),
                        ("HIGH_BIT", 0x80000000),
                        ("ALIAS_MAX", 0xFFFFFFFF),
                    ]
                    .into_iter()
                    .map(|(name, value)| EnumMember {
                        name: name.to_string(),
                        value,
                    })
                    .collect(),
                }],
            ),
        )]);
        for language in ["hpp", "rs", "zig"] {
            let mut out = String::new();
            let mut fmt = Formatter::new(&mut out, 4);
            match language {
                "hpp" => schema.write_hpp(&mut fmt).unwrap(),
                "rs" => schema.write_rs(&mut fmt).unwrap(),
                "zig" => schema.write_zig(&mut fmt).unwrap(),
                _ => unreachable!(),
            }
            assert!(out.contains("HIDDEN = 0xFFFFFFFD"), "{language}");
            assert!(out.contains("NOT_LEVELED = 0xFFFFFFFE"), "{language}");
            assert!(out.contains("HIGH_BIT = 0x80000000"), "{language}");
            if language != "hpp" {
                assert!(
                    !out.contains("ALIAS_MAX ="),
                    "duplicate discriminant in {language}"
                );
            }
        }
        assert_eq!(enum_member_bits(-3, 1), 0xFD);
        assert_eq!(enum_member_bits(-3, 2), 0xFFFD);
        assert_eq!(enum_member_bits(-3, 8), 0xFFFFFFFFFFFFFFFD);
    }
}
