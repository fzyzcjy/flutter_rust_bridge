pub struct MyGenericStruct<T> {
    generic_field: T,
    generic_boxed_field: Box<T>,
    normal_field: i32,
}

pub fn generic_func_string(arg: MyGenericStruct<String>) {}

pub fn generic_func_string_repeated(arg: MyGenericStruct<String>) {}

pub fn generic_func_bool(arg: MyGenericStruct<bool>) {}
