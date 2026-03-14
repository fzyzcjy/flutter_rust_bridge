// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;

pub struct StructWithZeroFieldTwinRustAsyncSse {}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_struct_with_zero_field_twin_rust_async_sse(
    arg: StructWithZeroFieldTwinRustAsyncSse,
) -> StructWithZeroFieldTwinRustAsyncSse {
    arg
}

pub struct StructWithOneFieldTwinRustAsyncSse {
    pub a: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_struct_with_one_field_twin_rust_async_sse(
    arg: StructWithOneFieldTwinRustAsyncSse,
) -> StructWithOneFieldTwinRustAsyncSse {
    arg
}

pub struct StructWithTwoFieldTwinRustAsyncSse {
    pub a: i32,
    pub b: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_struct_with_two_field_twin_rust_async_sse(
    arg: StructWithTwoFieldTwinRustAsyncSse,
) -> StructWithTwoFieldTwinRustAsyncSse {
    arg
}

pub struct TupleStructWithOneFieldTwinRustAsyncSse(pub i32);

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_tuple_struct_with_one_field_twin_rust_async_sse(
    arg: TupleStructWithOneFieldTwinRustAsyncSse,
) -> TupleStructWithOneFieldTwinRustAsyncSse {
    arg
}

pub struct TupleStructWithTwoFieldTwinRustAsyncSse(pub i32, pub i32);

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_tuple_struct_with_two_field_twin_rust_async_sse(
    arg: TupleStructWithTwoFieldTwinRustAsyncSse,
) -> TupleStructWithTwoFieldTwinRustAsyncSse {
    arg
}

#[frb]
pub struct StructWithFieldRenameTwinRustAsyncSse {
    #[frb(name = "renamed_field")]
    pub class: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_for_struct_with_field_rename_twin_rust_async_sse(
    arg: StructWithFieldRenameTwinRustAsyncSse,
) -> StructWithFieldRenameTwinRustAsyncSse {
    arg
}

pub struct StructWithDartKeywordFieldTwinRustAsyncSse {
    pub class: i32,
    pub interface: i64,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_for_struct_with_dart_keyword_field_twin_rust_async_sse(
    arg: StructWithDartKeywordFieldTwinRustAsyncSse,
) -> StructWithDartKeywordFieldTwinRustAsyncSse {
    arg
}
