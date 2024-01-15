// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

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
