use std::collections::{BTreeMap, HashSet};
use std::ffi::CStr;

use anyhow::{Context, Result, bail, ensure};

use log::debug;

use memflow::prelude::v1::*;

use pelite::pattern;
use pelite::pe64::{Pe, PeView};

use serde::{Deserialize, Serialize};

use crate::memory::{read_c_string, read_code_image};
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
    read_type_scopes(process, &schema_system)
}

fn read_class_binding(
    mem: &mut impl MemoryView,
    binding_ptr: Pointer64<SchemaClassBinding>,
    expected_scope: Address,
    global_scope: Address,
) -> Result<Class> {
    let binding = mem.read_ptr(binding_ptr).data()?;
    ensure!(
        belongs_to_scope(binding.type_scope.address(), expected_scope, global_scope),
        "class candidate does not belong to this type scope"
    );
    let name = read_c_string(mem, binding.name.address())?;
    ensure!(
        !name.is_empty() && !name.chars().any(char::is_control),
        "invalid class name"
    );
    let module_name = format!("{}.dll", read_c_string(mem, binding.module_name.address())?);
    let fields = read_class_binding_fields(mem, &binding)
        .with_context(|| format!("reading fields for {name}"))?;
    let metadata = read_class_binding_metadata(mem, &binding)
        .with_context(|| format!("reading metadata for {name}"))?;
    let parent_name = if let Some(ptr) = binding.base_classes.non_null() {
        let base_class = mem.read_ptr(ptr).data()?;
        ensure!(!base_class.class.is_null(), "null base class for {name}");
        let parent_class = mem.read_ptr(base_class.class).data()?;
        let parent = read_c_string(mem, parent_class.name.address())?;
        ensure!(
            !parent.is_empty() && !parent.chars().any(char::is_control),
            "invalid base class name for {name}"
        );
        Some(parent)
    } else {
        ensure!(binding.has_base_class == 0, "missing base class for {name}");
        None
    };

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
    ensure!(binding.field_count >= 0, "negative field count");
    ensure!(
        binding.field_count == 0 || !binding.fields.is_null(),
        "null field table"
    );
    let mut fields = Vec::with_capacity(binding.field_count as usize);
    for i in 0..binding.field_count {
        let field = mem.read_ptr(binding.fields.at(i as _)).data()?;
        ensure!(!field.r#type.is_null(), "null type for field {i}");
        let name = read_c_string(mem, field.name.address())?;
        ensure!(
            is_field_name(&name) && field.offset >= 0,
            "invalid field {name:?} at offset {}",
            field.offset
        );
        let field_type = mem.read_ptr(field.r#type).data()?;
        let type_name = read_c_string(mem, field_type.name.address())?.replace(' ', "");
        ensure!(
            !type_name.is_empty() && !type_name.chars().any(char::is_control),
            "invalid type for field {name}"
        );
        fields.push(ClassField {
            name,
            type_name,
            offset: field.offset,
        });
    }
    Ok(fields)
}

fn read_class_binding_metadata(
    mem: &mut impl MemoryView,
    binding: &SchemaClassBinding,
) -> Result<Vec<ClassMetadata>> {
    ensure!(
        binding.static_metadata_count >= 0,
        "negative metadata count"
    );
    ensure!(
        binding.static_metadata_count == 0 || !binding.static_metadata.is_null(),
        "null metadata table"
    );
    let mut entries = Vec::with_capacity(binding.static_metadata_count as usize);
    for i in 0..binding.static_metadata_count {
        let metadata = mem.read_ptr(binding.static_metadata.at(i as _)).data()?;
        let name = read_c_string(mem, metadata.name.address())?;
        let entry = match name.as_str() {
            "MNetworkChangeCallback" => {
                ensure!(
                    !metadata.network_value.is_null(),
                    "missing value for {name}"
                );
                let value = mem.read_ptr(metadata.network_value).data()?;
                let name = read_c_string(mem, unsafe { value.value.name_ptr }.address())?;
                ClassMetadata::NetworkChangeCallback { name }
            }
            "MNetworkVarNames" => {
                ensure!(
                    !metadata.network_value.is_null(),
                    "missing value for {name}"
                );
                let value = mem.read_ptr(metadata.network_value).data()?;
                let var = unsafe { value.value.var_value };
                let name = read_c_string(mem, var.name.address())?;
                let type_name = read_c_string(mem, var.type_name.address())?.replace(' ', "");
                ClassMetadata::NetworkVarNames { name, type_name }
            }
            _ => ClassMetadata::Unknown { name },
        };
        entries.push(entry);
    }
    Ok(entries)
}

fn belongs_to_scope(owner: Address, scope: Address, global: Address) -> bool {
    owner == scope || (!global.is_null() && owner == global)
}

fn read_enum_binding(
    mem: &mut impl MemoryView,
    binding_ptr: Pointer64<SchemaEnumBinding>,
    expected_scope: Address,
    global_scope: Address,
) -> Result<Enum> {
    let binding = mem.read_ptr(binding_ptr).data()?;
    let owns = |scope| belongs_to_scope(scope, expected_scope, global_scope);
    if !owns(binding.type_scope.address()) && !owns(binding.legacy_type_scope.address()) {
        bail!("enum candidate does not belong to this type scope");
    }
    if !matches!(binding.size, 1 | 2 | 4 | 8) || !matches!(binding.alignment, 1 | 2 | 4 | 8) {
        bail!("invalid enum size or alignment");
    }

    let name = read_c_string(mem, binding.name.address())?;

    if name.is_empty() || name.chars().any(char::is_control) {
        bail!("invalid enum name");
    }

    let members = read_enum_binding_members(mem, &binding)
        .with_context(|| format!("reading members for {name}"))?;

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
    ensure!(
        binding.enumerator_count == 0 || !binding.enumerators.is_null(),
        "null enumerator table"
    );
    let mut members = Vec::with_capacity(binding.enumerator_count as usize);
    for i in 0..binding.enumerator_count {
        let entry = mem.read_ptr(binding.enumerators.at(i as _)).data()?;
        let name = read_c_string(mem, entry.name.address())?;
        ensure!(is_field_name(&name), "invalid enumerator name {name:?}");
        members.push(EnumMember {
            name,
            value: unsafe { entry.value.ulong } as i64,
        });
    }
    Ok(members)
}

fn read_schema_system<P: Process + MemoryView>(process: &mut P) -> Result<SchemaSystem> {
    let module = process.module_by_name("schemasystem.dll")?;

    let buf = read_code_image(process, &module)?;

    let view = PeView::from_bytes(&buf)?;

    let mut save = [0; 2];

    if !view
        .scanner()
        .finds_code(pattern!("4c8d35${'} 0f2845"), &mut save)
    {
        bail!("outdated schema system pattern");
    }

    let schema_system: SchemaSystem = process.read(module.base + save[1]).data()?;

    if schema_system.registration_count == 0 {
        bail!("no schema registrations");
    }

    Ok(schema_system)
}

fn read_type_scopes(mem: &mut impl MemoryView, schema_system: &SchemaSystem) -> Result<SchemaMap> {
    let type_scopes = &schema_system.type_scopes;
    ensure!(
        type_scopes.count > 0 && !type_scopes.data.is_null(),
        "missing schema type scopes"
    );

    let mut modules = BTreeMap::new();
    let mut global_registries = BTreeMap::new();
    for i in 0..type_scopes.count {
        let scope_ptr = type_scopes
            .element(mem, i as _)
            .with_context(|| format!("reading schema scope pointer {i}"))?;
        ensure!(!scope_ptr.is_null(), "null schema scope {i}");
        let scope = mem
            .read_ptr(scope_ptr)
            .data()
            .with_context(|| format!("reading schema scope {i}"))?;
        let module = CStr::from_bytes_until_nul(&scope.name)
            .with_context(|| format!("unterminated name for schema scope {i}"))?
            .to_str()
            .with_context(|| format!("invalid UTF-8 in schema scope {i}"))?;
        ensure!(
            !module.is_empty() && !module.chars().any(char::is_control),
            "invalid schema scope name"
        );
        let global = scope.global_scope.address();
        let registry = if let Some(global_ptr) = scope.global_scope.non_null() {
            use std::collections::btree_map::Entry;
            Some(match global_registries.entry(global) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    let global_scope = mem
                        .read_ptr(global_ptr)
                        .data()
                        .context("reading global schema scope")?;
                    let classes: HashSet<_> = global_scope
                        .class_bindings
                        .registered(mem)?
                        .into_iter()
                        .map(|ptr| ptr.address().to_umem())
                        .collect();
                    let enums: HashSet<_> = global_scope
                        .enum_bindings
                        .registered(mem)?
                        .into_iter()
                        .map(|ptr| ptr.address().to_umem())
                        .collect();
                    entry.insert((classes, enums))
                }
            })
        } else {
            None
        };

        let mut class_ptrs = scope
            .class_bindings
            .registered(mem)
            .with_context(|| format!("reading class registrations in {module}"))?;
        let mut enum_ptrs = scope
            .enum_bindings
            .registered(mem)
            .with_context(|| format!("reading enum registrations in {module}"))?;
        if let Some((global_classes, global_enums)) = registry {
            class_ptrs.extend(
                scope
                    .class_bindings
                    .pool_candidates(mem)?
                    .into_iter()
                    .filter(|ptr| global_classes.contains(&ptr.address().to_umem())),
            );
            enum_ptrs.extend(
                scope
                    .enum_bindings
                    .pool_candidates(mem)?
                    .into_iter()
                    .filter(|ptr| global_enums.contains(&ptr.address().to_umem())),
            );
        }

        let mut seen = HashSet::new();
        let classes = class_ptrs
            .into_iter()
            .filter(|ptr| seen.insert(ptr.address().to_umem()))
            .map(|ptr| read_class_binding(mem, ptr, scope_ptr.address(), global))
            .collect::<Result<Vec<_>>>()
            .with_context(|| format!("reading classes in {module}"))?;
        seen.clear();
        let enums = enum_ptrs
            .into_iter()
            .filter(|ptr| seen.insert(ptr.address().to_umem()))
            .map(|ptr| read_enum_binding(mem, ptr, scope_ptr.address(), global))
            .collect::<Result<Vec<_>>>()
            .with_context(|| format!("reading enums in {module}"))?;

        if classes.is_empty() && enums.is_empty() {
            continue;
        }
        debug!(
            "module {module:?} contains {} class(es) and {} enum(s)",
            classes.len(),
            enums.len()
        );
        ensure!(
            !modules.contains_key(module),
            "duplicate schema scope {module}"
        );
        modules.insert(module.to_string(), (classes, enums));
    }
    Ok(modules)
}
