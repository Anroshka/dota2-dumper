use std::collections::BTreeMap;
use std::ffi::CStr;

use anyhow::{Result, bail};

use log::debug;

use memflow::prelude::v1::*;

use pelite::pattern;
use pelite::pe64::{Pe, PeView};

use serde::{Deserialize, Serialize};

use crate::source2::*;

pub type SchemaMap = BTreeMap<String, (Vec<Class>, Vec<Enum>)>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ClassMetadata {
    Unknown { name: String },
    NetworkChangeCallback { name: String },
    NetworkVarNames { name: String, type_name: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Class {
    pub name: String,
    pub module_name: String,
    pub parent_name: Option<String>,
    pub metadata: Vec<ClassMetadata>,
    pub fields: Vec<ClassField>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClassField {
    pub name: String,
    pub type_name: String,
    pub offset: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Enum {
    pub name: String,
    pub alignment: u8,
    pub size: u16,
    pub members: Vec<EnumMember>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumMember {
    pub name: String,
    pub value: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypeScope {
    pub module_name: String,
    pub classes: Vec<Class>,
    pub enums: Vec<Enum>,
}

pub fn schemas<P: Process + MemoryView>(process: &mut P) -> Result<SchemaMap> {
    let schema_system = read_schema_system(process)?;
    let type_scopes = read_type_scopes(process, &schema_system)?;

    Ok(type_scopes
        .into_iter()
        .map(|type_scope| {
            (
                type_scope.module_name,
                (type_scope.classes, type_scope.enums),
            )
        })
        .collect())
}

fn read_class_binding(
    mem: &mut impl MemoryView,
    binding_ptr: Pointer64<SchemaClassBinding>,
    expected_scope: Address,
    global_scope: Address,
) -> Result<Class> {
    let binding = mem.read_ptr(binding_ptr).data_part()?;
    if binding.type_scope.address() != expected_scope
        && (global_scope.is_null() || binding.type_scope.address() != global_scope)
    {
        bail!("class candidate does not belong to this type scope");
    }

    let module_name = mem
        .read_utf8(binding.module_name.address(), 128)
        .data_part()
        .map(|m| format!("{}.dll", m))?;

    let name = mem.read_utf8(binding.name.address(), 128).data_part()?;

    if name.is_empty() || name.chars().any(char::is_control) {
        bail!("invalid class name");
    }

    let parent_name = binding.base_classes.non_null().and_then(|ptr| {
        let base_class = mem.read_ptr(ptr).data_part().ok()?;
        let parent_class = mem.read_ptr(base_class.class).data_part().ok()?;

        let parent_name = mem
            .read_utf8(parent_class.name.address(), 128)
            .data_part()
            .ok()?;

        (!parent_name.is_empty()).then_some(parent_name)
    });

    let fields = read_class_binding_fields(mem, &binding)?;
    let metadata = read_class_binding_metadata(mem, &binding)?;

    Ok(Class {
        name,
        module_name,
        parent_name,
        metadata,
        fields,
    })
}

fn read_class_binding_fields(
    mem: &mut impl MemoryView,
    binding: &SchemaClassBinding,
) -> Result<Vec<ClassField>> {
    if binding.fields.is_null() {
        return Ok(Vec::new());
    }

    Ok((0..binding.field_count).fold(Vec::new(), |mut acc, i| {
        let field = match mem.read_ptr(binding.fields.at(i as _)).data_part() {
            Ok(f) => f,
            Err(_) => return acc,
        };

        if field.r#type.is_null() {
            return acc;
        }

        let name = match mem.read_utf8(field.name.address(), 128).data_part() {
            Ok(n) => n,
            Err(_) => return acc,
        };

        if !is_field_name(&name) || field.offset < 0 {
            log::warn!(
                "discarding invalid schema field in binding at {:?}",
                binding.name.address()
            );
            return acc;
        }

        let r#type = match mem.read_ptr(field.r#type).data_part() {
            Ok(t) => t,
            Err(_) => return acc,
        };

        let type_name = match mem.read_utf8(r#type.name.address(), 128).data_part() {
            Ok(tn) => tn.replace(" ", ""),
            Err(_) => return acc,
        };
        if type_name.is_empty() || type_name.chars().any(char::is_control) {
            return acc;
        }

        acc.push(ClassField {
            name,
            type_name,
            offset: field.offset,
        });

        acc
    }))
}

fn read_class_binding_metadata(
    mem: &mut impl MemoryView,
    binding: &SchemaClassBinding,
) -> Result<Vec<ClassMetadata>> {
    if binding.static_metadata.is_null() {
        return Ok(Vec::new());
    }

    Ok(
        (0..binding.static_metadata_count).fold(Vec::new(), |mut acc, i| {
            let metadata = match mem.read_ptr(binding.static_metadata.at(i as _)).data_part() {
                Ok(m) => m,
                Err(_) => return acc,
            };

            if metadata.network_value.is_null() {
                return acc;
            }

            let name = match mem.read_utf8(metadata.name.address(), 128).data_part() {
                Ok(n) => n,
                Err(_) => return acc,
            };

            let network_value = match mem.read_ptr(metadata.network_value).data_part() {
                Ok(nv) => nv,
                Err(_) => return acc,
            };

            let metadata_entry = match name.as_str() {
                "MNetworkChangeCallback" => unsafe {
                    let name = match mem
                        .read_utf8(network_value.value.name_ptr.address(), 128)
                        .data_part()
                    {
                        Ok(n) => n,
                        Err(_) => return acc,
                    };

                    ClassMetadata::NetworkChangeCallback { name }
                },
                "MNetworkVarNames" => unsafe {
                    let var_value = network_value.value.var_value;

                    let name = match mem.read_utf8(var_value.name.address(), 128).data_part() {
                        Ok(n) => n,
                        Err(_) => return acc,
                    };

                    let type_name = match mem
                        .read_utf8(var_value.type_name.address(), 128)
                        .data_part()
                    {
                        Ok(tn) => tn.replace(" ", ""),
                        Err(_) => return acc,
                    };

                    ClassMetadata::NetworkVarNames { name, type_name }
                },
                _ => ClassMetadata::Unknown { name },
            };

            acc.push(metadata_entry);
            acc
        }),
    )
}

fn read_enum_binding(
    mem: &mut impl MemoryView,
    binding_ptr: Pointer64<SchemaEnumBinding>,
    expected_scope: Address,
    global_scope: Address,
) -> Result<Enum> {
    let binding = mem.read_ptr(binding_ptr).data_part()?;
    if binding.type_scope.address() != expected_scope
        && (global_scope.is_null() || binding.type_scope.address() != global_scope)
    {
        bail!("enum candidate does not belong to this type scope");
    }

    let name = mem.read_utf8(binding.name.address(), 128).data_part()?;

    if name.is_empty() || name.chars().any(char::is_control) {
        bail!("invalid enum name");
    }

    let members = read_enum_binding_members(mem, &binding)?;

    Ok(Enum {
        name,
        alignment: binding.alignment,
        size: binding.enumerator_count,
        members,
    })
}

fn is_field_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some(c) if c == '_' || c.is_ascii_alphabetic())
        && chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;
    use memflow::connector::FileIoMemory;
    use std::io::Cursor;

    #[test]
    fn rejects_pooled_bytes_without_the_expected_schema_owner() {
        let mut bytes = vec![0u8; 0x400];
        let class_owner = 0x100 + offset_of!(SchemaClassBinding.type_scope);
        let enum_owner = 0x200 + offset_of!(SchemaEnumBinding.type_scope);
        bytes[class_owner..class_owner + 8].copy_from_slice(&0xBADu64.to_le_bytes());
        bytes[enum_owner..enum_owner + 8].copy_from_slice(&0xBADu64.to_le_bytes());
        let mut mem = FileIoMemory::with_size(Cursor::new(bytes), 0x400)
            .unwrap()
            .into_phys_view();
        let scope = Address::from(0x300u64);
        let global = Address::from(0x350u64);
        let class = read_class_binding(
            &mut mem,
            Pointer64::from(Address::from(0x100u64)),
            scope,
            global,
        );
        let enum_ = read_enum_binding(
            &mut mem,
            Pointer64::from(Address::from(0x200u64)),
            scope,
            global,
        );
        assert!(class.unwrap_err().to_string().contains("type scope"));
        assert!(enum_.unwrap_err().to_string().contains("type scope"));
        assert!(is_field_name("type"));
        assert!(!is_field_name(""));
        assert!(!is_field_name("bad\nfield"));
        assert!(!is_field_name("bad\u{FFFD}field"));
    }
}

fn read_enum_binding_members(
    mem: &mut impl MemoryView,
    binding: &SchemaEnumBinding,
) -> Result<Vec<EnumMember>> {
    if binding.enumerators.is_null() {
        return Ok(Vec::new());
    }

    Ok(
        (0..binding.enumerator_count).fold(Vec::new(), |mut acc, i| {
            let r#enum = match mem.read_ptr(binding.enumerators.at(i as _)).data_part() {
                Ok(e) => e,
                Err(_) => return acc,
            };

            let name = match mem.read_utf8(r#enum.name.address(), 128).data_part() {
                Ok(n) => n,
                Err(_) => return acc,
            };

            acc.push(EnumMember {
                name,
                value: unsafe { r#enum.value.ulong } as i64,
            });

            acc
        }),
    )
}

fn read_schema_system<P: Process + MemoryView>(process: &mut P) -> Result<SchemaSystem> {
    let module = process.module_by_name("schemasystem.dll")?;

    let buf = process
        .read_raw(module.base, module.size as _)
        .data_part()?;

    let view = PeView::from_bytes(&buf)?;

    let mut save = [0; 2];

    if !view
        .scanner()
        .finds_code(pattern!("4c8d35${'} 0f2845"), &mut save)
    {
        bail!("outdated schema system pattern");
    }

    let schema_system: SchemaSystem = process.read(module.base + save[1]).data_part()?;

    if schema_system.registration_count == 0 {
        bail!("no schema registrations");
    }

    Ok(schema_system)
}

fn read_type_scopes(
    mem: &mut impl MemoryView,
    schema_system: &SchemaSystem,
) -> Result<Vec<TypeScope>> {
    let type_scopes = &schema_system.type_scopes;

    Ok((0..type_scopes.count).fold(Vec::new(), |mut acc, i| {
        let type_scope_ptr = match type_scopes.element(mem, i as _) {
            Ok(ptr) => ptr,
            Err(_) => return acc,
        };

        let type_scope = match mem.read_ptr(type_scope_ptr).data_part() {
            Ok(scope) => scope,
            Err(_) => return acc,
        };

        let module_name = unsafe { CStr::from_ptr(type_scope.name.as_ptr()) }
            .to_string_lossy()
            .to_string();

        let classes: Vec<_> = type_scope
            .class_bindings
            .elements(mem)
            .iter()
            .filter_map(|ptr| {
                read_class_binding(
                    mem,
                    *ptr,
                    type_scope_ptr.address(),
                    type_scope.global_scope.address(),
                )
                .ok()
            })
            .collect();

        let enums: Vec<_> = type_scope
            .enum_bindings
            .elements(mem)
            .iter()
            .filter_map(|ptr| {
                read_enum_binding(
                    mem,
                    *ptr,
                    type_scope_ptr.address(),
                    type_scope.global_scope.address(),
                )
                .ok()
            })
            .collect();

        if classes.is_empty() && enums.is_empty() {
            return acc;
        }

        debug!(
            "module \"{}\" contains {} class(es) and {} enum(s)",
            module_name,
            classes.len(),
            enums.len(),
        );

        acc.push(TypeScope {
            module_name,
            classes,
            enums,
        });

        acc
    }))
}
