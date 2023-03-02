use crate::custom::{CrossSharedStruct, OnlyForApi2Struct, SharedStruct};

pub fn test_inbuilt_type_2(a: i32, b: f32) -> f32 {
    a as f32 + b
}

pub fn test_string_2(s: String, i: u64) -> String {
    format!("{}_{}", s, i)
}

pub fn test_shared_struct_2(mut custom: SharedStruct, s: String, i: i32) -> SharedStruct {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_unique_struct_2(mut custom: OnlyForApi2Struct, s: String, i: i64) -> OnlyForApi2Struct {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_cross_shared_struct_2(name: String) -> CrossSharedStruct {
    CrossSharedStruct { name }
}
