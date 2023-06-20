use flutter_rust_bridge::SyncReturn;

use crate::block_specific_module::StructOnlyForBlock2;
use crate::shared_type_module::SharedStructInAllBlocks;
use crate::shared_type_module::{CrossSharedStructInBlock1And2, SharedStructInBlock1And2};
use crate::shared_type_module::{CrossSharedStructInBlock2And3, SharedStructInBlock2And3};

pub enum EnumDefinedInBlock2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl EnumDefinedInBlock2 {
    #[allow(unused)]
    pub fn test_method(&self, message: String) -> String {
        message
    }
    #[allow(unused)]
    pub fn test_static_method(message: String) -> String {
        message
    }
}

pub struct StructDefinedInBlock2 {
    pub name: String,
}
impl StructDefinedInBlock2 {
    #[allow(unused)]
    pub fn test_method(&self, message: String) -> String {
        message
    }
    #[allow(unused)]
    pub fn test_static_method(message: String) -> String {
        message
    }
}

pub fn test_inbuilt_type_in_block_2(a: i32, b: f32) -> f32 {
    a as f32 + b
}

pub fn test_string_in_block_2(s: String, i: u64) -> String {
    format!("{}_{}", s, i)
}

pub fn test_all_shared_struct_in_block_2(
    mut custom: SharedStructInAllBlocks,
    s: String,
    i: i32,
) -> SharedStructInAllBlocks {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_all_shared_struct_in_sync_in_block_2(
    mut custom: SharedStructInAllBlocks,
    s: String,
    i: i32,
) -> SyncReturn<SharedStructInAllBlocks> {
    custom.name = s;
    custom.id = i;
    SyncReturn(custom)
}

pub fn test_shared_struct_in_block_2_for_1_and_2(
    mut custom: SharedStructInBlock1And2,
    s: String,
    i: i32,
) -> SharedStructInBlock1And2 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_cross_shared_struct_in_block_2_for_1_and_2(
    name: String,
) -> CrossSharedStructInBlock1And2 {
    CrossSharedStructInBlock1And2 { name }
}

pub fn test_shared_struct_in_block_2_for_2_and_3(
    mut custom: SharedStructInBlock2And3,
    s: String,
    i: i32,
) -> SharedStructInBlock2And3 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_cross_shared_struct_in_block_2_for_2_and_3(
    custom: CrossSharedStructInBlock2And3,
) -> String {
    custom.name
}

pub fn test_unique_struct_2(
    mut custom: StructOnlyForBlock2,
    s: String,
    i: i16,
) -> StructOnlyForBlock2 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_struct_defined_in_block_2(custom: StructDefinedInBlock2) -> String {
    custom.name
}
