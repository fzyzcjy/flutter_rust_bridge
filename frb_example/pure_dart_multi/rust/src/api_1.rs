use crate::custom::{
    CrossSharedStructInBlock1And2, SharedStructInAllBlocks, SharedStructInBlock1And2,
    StructOnlyForBlock1,
};
pub struct StructDefinedInBlock1 {
    pub name: String,
}
impl StructDefinedInBlock1 {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

pub fn test_inbuilt_type_in_block_1(a: i32, b: f32) -> f32 {
    a as f32 + b
}

pub fn test_string_in_block_1(s: String, i: u64) -> String {
    format!("{}_{}", s, i)
}

pub fn test_all_shared_struct_in_block_1(
    mut custom: SharedStructInAllBlocks,
    s: String,
    i: i32,
) -> SharedStructInAllBlocks {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_shared_struct_in_block_1_for_1_and_2(
    mut custom: SharedStructInBlock1And2,
    s: String,
    i: i32,
) -> SharedStructInBlock1And2 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_cross_shared_struct_in_block_1_for_1_and_2(
    custom: CrossSharedStructInBlock1And2,
) -> String {
    custom.name
}

pub fn test_unique_struct_1(
    mut custom: StructOnlyForBlock1,
    s: String,
    i: i8,
) -> StructOnlyForBlock1 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_struct_defined_in_block_1(custom: StructDefinedInBlock1) -> String {
    custom.name
}
