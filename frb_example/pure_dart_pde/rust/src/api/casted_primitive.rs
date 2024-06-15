// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::frb;

#[frb(serialize)]
#[frb(type_64bit_int)]
pub fn casted_primitive_i64_twin_normal(arg: i64) -> i64 {
    arg
}

#[frb(serialize)]
#[frb(type_64bit_int)]
pub fn casted_primitive_u64_twin_normal(arg: u64) -> u64 {
    arg
}

#[frb(serialize)]
#[frb(type_64bit_int)]
pub fn casted_primitive_isize_twin_normal(arg: isize) -> isize {
    arg
}

#[frb(serialize)]
#[frb(type_64bit_int)]
pub fn casted_primitive_usize_twin_normal(arg: usize) -> usize {
    arg
}

#[frb(serialize)]
#[frb(type_64bit_int)]
pub fn casted_primitive_multi_arg_twin_normal(a: i32, b: i64, c: usize, d: i128) {
    let _ = (a, b, c, d);
}

#[frb(type_64bit_int)]
pub struct StructWithCastedPrimitiveTwinNormal {
    pub field_i64: i64,
    pub field_u64: u64,
    pub field_i32: i32,
    pub field_vec_u8: Vec<u8>,
}

#[frb(serialize)]
pub fn function_for_struct_with_casted_primitive_twin_normal(
    arg: StructWithCastedPrimitiveTwinNormal,
) -> StructWithCastedPrimitiveTwinNormal {
    arg
}
