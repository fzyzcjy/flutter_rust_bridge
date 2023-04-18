use crate::custom::{CrossSharedStruct, OnlyForApi1Struct, SharedStruct};

pub struct StructDefinedInApi1 {
    pub name: String,
}
impl StructDefinedInApi1 {
    fn test_struct_method(message: String) -> String {
        message
    }
}

pub fn test_inbuilt_type_1(a: i32, b: f32) -> f32 {
    a as f32 + b
}

pub fn test_string_1(s: String, i: u64) -> String {
    format!("{}_{}", s, i)
}

pub fn test_shared_struct_1(mut custom: SharedStruct, s: String, i: i32) -> SharedStruct {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_unique_struct_1(mut custom: OnlyForApi1Struct, s: String, i: i16) -> OnlyForApi1Struct {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_cross_shared_struct_1(custom: CrossSharedStruct) -> String {
    custom.name
}


pub fn test_StructDefinedInApi1(custom: StructDefinedInApi1) -> String {
    custom.name
}