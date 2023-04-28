use flutter_rust_bridge::SyncReturn;

use crate::custom::{
    CrossSharedStructInBlock2And3, SharedStructInAllBlocks, SharedStructInBlock2And3,
    SharedStructOnlyForSyncTest, StructOnlyForBlock3,
};

pub struct StructDefinedInBlock3 {
    pub name: String,
}
impl StructDefinedInBlock3 {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

pub fn test_inbuilt_type_in_block_3(a: i32, b: f32) -> f32 {
    a as f32 + b
}

pub fn test_string_in_block_3(s: String, i: u64) -> String {
    format!("{}_{}", s, i)
}

pub fn test_shared_struct_only_for_sync_with_no_sync_return_in_block_3(
    score: f64,
) -> SharedStructOnlyForSyncTest {
    SharedStructOnlyForSyncTest {
        default_score: score,
    }
}

pub fn test_all_shared_struct_in_block_3(
    mut custom: SharedStructInAllBlocks,
    s: String,
    i: i32,
) -> SharedStructInAllBlocks {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_shared_struct_in_block_3_for_2_and_3(
    mut custom: SharedStructInBlock2And3,
    s: String,
    i: i32,
) -> SharedStructInBlock2And3 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_cross_shared_struct_in_block_3_for_2_and_3(
    name: String,
) -> CrossSharedStructInBlock2And3 {
    CrossSharedStructInBlock2And3 { name }
}

pub fn test_cross_shared_struct_in_sync_in_block_3_for_2_and_3(
    name: String,
) -> SyncReturn<CrossSharedStructInBlock2And3> {
    SyncReturn(CrossSharedStructInBlock2And3 { name })
}

pub fn test_unique_struct_3(
    mut custom: StructOnlyForBlock3,
    s: String,
    i: i64,
) -> StructOnlyForBlock3 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_struct_defined_in_block_3(custom: StructDefinedInBlock3) -> String {
    custom.name
}
