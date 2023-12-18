pub struct MyGenericStruct<T> {
    generic_field: T,
    generic_boxed_field: Box<T>,
    normal_field: i32,
}

pub fn func_struct_string(arg: MyGenericStruct<String>) {}

pub fn func_struct_string_repeated(arg: MyGenericStruct<String>) {}

pub fn func_struct_bool(arg: MyGenericStruct<bool>) {}

pub enum MyGenericEnum<A, B> {
    One(A),
    Two(B),
}

pub fn func_enum_string(arg: MyGenericEnum<String>) {}

pub fn func_enum_bool(arg: MyGenericEnum<bool>) {}
