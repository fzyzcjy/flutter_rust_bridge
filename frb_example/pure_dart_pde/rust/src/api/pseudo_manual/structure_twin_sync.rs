// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;

pub struct StructWithZeroFieldTwinSync {}

#[flutter_rust_bridge::frb(sync)]
pub fn func_struct_with_zero_field_twin_sync(
    arg: StructWithZeroFieldTwinSync,
) -> StructWithZeroFieldTwinSync {
    arg
}

pub struct StructWithOneFieldTwinSync {
    pub a: i32,
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_struct_with_one_field_twin_sync(
    arg: StructWithOneFieldTwinSync,
) -> StructWithOneFieldTwinSync {
    arg
}

pub struct StructWithTwoFieldTwinSync {
    pub a: i32,
    pub b: i32,
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_struct_with_two_field_twin_sync(
    arg: StructWithTwoFieldTwinSync,
) -> StructWithTwoFieldTwinSync {
    arg
}

pub struct TupleStructWithOneFieldTwinSync(pub i32);

#[flutter_rust_bridge::frb(sync)]
pub fn func_tuple_struct_with_one_field_twin_sync(
    arg: TupleStructWithOneFieldTwinSync,
) -> TupleStructWithOneFieldTwinSync {
    arg
}

pub struct TupleStructWithTwoFieldTwinSync(pub i32, pub i32);

#[flutter_rust_bridge::frb(sync)]
pub fn func_tuple_struct_with_two_field_twin_sync(
    arg: TupleStructWithTwoFieldTwinSync,
) -> TupleStructWithTwoFieldTwinSync {
    arg
}

#[frb]
pub struct StructWithFieldRenameTwinSync {
    #[frb(name = "renamed_field")]
    pub class: i32,
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_for_struct_with_field_rename_twin_sync(
    arg: StructWithFieldRenameTwinSync,
) -> StructWithFieldRenameTwinSync {
    arg
}

pub struct StructWithDartKeywordFieldTwinSync {
    pub class: i32,
    pub interface: i64,
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_for_struct_with_dart_keyword_field_twin_sync(
    arg: StructWithDartKeywordFieldTwinSync,
) -> StructWithDartKeywordFieldTwinSync {
    arg
}
