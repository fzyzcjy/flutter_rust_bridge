use crate::custom::{CrossSharedStruct, OnlyForApi2Struct, SharedStruct};

pub struct StructDefinedInApi2 {
    pub name: String,
}
impl StructDefinedInApi2 {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

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

pub fn test_StructDefinedInApi2(custom: StructDefinedInApi2) -> String {
    custom.name
}
