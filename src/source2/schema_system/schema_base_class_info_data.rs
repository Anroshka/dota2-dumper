use memflow::prelude::v1::*;

use super::SchemaClassBinding;

#[repr(C)]
#[derive(Pod)]
pub struct SchemaBaseClassInfoData {
    pub offset: i32,
    pad_0: [u8; 4],
    pub class: Pointer64<SchemaClassBinding>,
}
