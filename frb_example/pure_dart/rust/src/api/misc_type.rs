// TODO corresponding tests in Dart

pub fn func_string_twin_normal(arg: String) -> String {
    arg
}

#[allow(clippy::unused_unit)]
pub fn func_return_unit_twin_normal() -> () {}

// TODO move to tests related to `tuple struct`?
pub struct NewTypeIntTwinNormal(pub i32);

pub fn func_new_type_int_twin_normal(arg: NewTypeIntTwinNormal) -> NewTypeIntTwinNormal {
    arg
}
