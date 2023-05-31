use flutter_rust_bridge::SyncReturn;

use crate::block_specific_module::StructOnlyForBlock1;
use crate::shared_type_module::{
    CrossSharedStructInBlock1And2, SharedStructInAllBlocks, SharedStructInBlock1And2,
    SharedStructOnlyForSyncTest,
};

pub struct StructDefinedInBlock1 {
    pub name: String,
}
impl StructDefinedInBlock1 {
    #[allow(unused)]
    pub fn test_method(&self, message: String) -> String {
        message
    }
    #[allow(unused)]
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

pub fn test_string_in_sync_in_block_1(s: String, i: u64) -> SyncReturn<String> {
    SyncReturn(format!("{}_{}", s, i))
}

pub fn test_optional_string_in_block_1(s: Option<String>, i: i32) -> Option<String> {
    s.map(|s| format!("{}{}", s, i))
}

pub fn test_optional_string_in_sync_in_block_1(
    s: Option<String>,
    i: i32,
) -> SyncReturn<Option<String>> {
    match s {
        Some(s) => SyncReturn(Some(format!("{}{}", s, i))),
        None => SyncReturn(None),
    }
}

pub fn test_shared_struct_only_for_sync_with_sync_return_in_block_1(
    name: String,
    score: f64,
) -> SyncReturn<SharedStructOnlyForSyncTest> {
    SyncReturn(SharedStructOnlyForSyncTest { name, score })
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
    custom.name = if s.is_empty() { None } else { Some(s) };
    custom.id = Some(i);
    custom.num = Some(i as f64);
    custom
}

pub fn test_struct_defined_in_block_1(custom: StructDefinedInBlock1) -> String {
    custom.name
}
