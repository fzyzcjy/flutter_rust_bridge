// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;

pub struct StructWithZeroFieldTwinSse {}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_struct_with_zero_field_twin_sse(
    arg: StructWithZeroFieldTwinSse,
) -> StructWithZeroFieldTwinSse {
    arg
}

pub struct StructWithOneFieldTwinSse {
    pub a: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_struct_with_one_field_twin_sse(
    arg: StructWithOneFieldTwinSse,
) -> StructWithOneFieldTwinSse {
    arg
}

pub struct StructWithTwoFieldTwinSse {
    pub a: i32,
    pub b: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_struct_with_two_field_twin_sse(
    arg: StructWithTwoFieldTwinSse,
) -> StructWithTwoFieldTwinSse {
    arg
}

pub struct TupleStructWithOneFieldTwinSse(pub i32);

#[flutter_rust_bridge::frb(serialize)]
pub fn func_tuple_struct_with_one_field_twin_sse(
    arg: TupleStructWithOneFieldTwinSse,
) -> TupleStructWithOneFieldTwinSse {
    arg
}

pub struct TupleStructWithTwoFieldTwinSse(pub i32, pub i32);

#[flutter_rust_bridge::frb(serialize)]
pub fn func_tuple_struct_with_two_field_twin_sse(
    arg: TupleStructWithTwoFieldTwinSse,
) -> TupleStructWithTwoFieldTwinSse {
    arg
}

#[frb]
pub struct StructWithFieldRenameTwinSse {
    #[frb(name = "renamed_field")]
    pub class: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_for_struct_with_field_rename_twin_sse(
    arg: StructWithFieldRenameTwinSse,
) -> StructWithFieldRenameTwinSse {
    arg
}

pub struct StructWithDartKeywordFieldTwinSse {
    pub class: i32,
    pub interface: i64,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_for_struct_with_dart_keyword_field_twin_sse(
    arg: StructWithDartKeywordFieldTwinSse,
) -> StructWithDartKeywordFieldTwinSse {
    arg
}
