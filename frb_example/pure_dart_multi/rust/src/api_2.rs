use crate::custom::SharedStructInAllBlocks;
use crate::custom::StructOnlyForBlock2;
use crate::custom::{CrossSharedStructInBlock1And2, SharedStructInBlock1And2};
use crate::custom::{CrossSharedStructInBlock2And3, SharedStructInBlock2And3};
pub struct StructDefinedInBlock2 {
    pub name: String,
}
impl StructDefinedInBlock2 {
    pub fn test_method(&self, message: String) -> String {
        message
    }
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
