// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;

pub struct StructWithZeroFieldTwinRustAsync {}

pub async fn func_struct_with_zero_field_twin_rust_async(
    arg: StructWithZeroFieldTwinRustAsync,
) -> StructWithZeroFieldTwinRustAsync {
    arg
}

pub struct StructWithOneFieldTwinRustAsync {
    pub a: i32,
}

pub async fn func_struct_with_one_field_twin_rust_async(
    arg: StructWithOneFieldTwinRustAsync,
) -> StructWithOneFieldTwinRustAsync {
    arg
}

pub struct StructWithTwoFieldTwinRustAsync {
    pub a: i32,
    pub b: i32,
}

pub async fn func_struct_with_two_field_twin_rust_async(
    arg: StructWithTwoFieldTwinRustAsync,
) -> StructWithTwoFieldTwinRustAsync {
    arg
}

pub struct TupleStructWithOneFieldTwinRustAsync(pub i32);

pub async fn func_tuple_struct_with_one_field_twin_rust_async(
    arg: TupleStructWithOneFieldTwinRustAsync,
) -> TupleStructWithOneFieldTwinRustAsync {
    arg
}

pub struct TupleStructWithTwoFieldTwinRustAsync(pub i32, pub i32);

pub async fn func_tuple_struct_with_two_field_twin_rust_async(
    arg: TupleStructWithTwoFieldTwinRustAsync,
) -> TupleStructWithTwoFieldTwinRustAsync {
    arg
}

#[frb]
pub struct StructWithFieldRenameTwinRustAsync {
    #[frb(name = "renamed_field")]
    pub class: i32,
}

pub async fn func_for_struct_with_field_rename_twin_rust_async(
    arg: StructWithFieldRenameTwinRustAsync,
) -> StructWithFieldRenameTwinRustAsync {
    arg
}

pub struct StructWithDartKeywordFieldTwinRustAsync {
    pub class: i32,
    pub interface: i64,
}

pub async fn func_for_struct_with_dart_keyword_field_twin_rust_async(
    arg: StructWithDartKeywordFieldTwinRustAsync,
) -> StructWithDartKeywordFieldTwinRustAsync {
    arg
}
