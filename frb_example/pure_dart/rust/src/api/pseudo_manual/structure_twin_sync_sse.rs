// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;

pub struct StructWithZeroFieldTwinSyncSse {}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_struct_with_zero_field_twin_sync_sse(
    arg: StructWithZeroFieldTwinSyncSse,
) -> StructWithZeroFieldTwinSyncSse {
    arg
}

pub struct StructWithOneFieldTwinSyncSse {
    pub a: i32,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_struct_with_one_field_twin_sync_sse(
    arg: StructWithOneFieldTwinSyncSse,
) -> StructWithOneFieldTwinSyncSse {
    arg
}

pub struct StructWithTwoFieldTwinSyncSse {
    pub a: i32,
    pub b: i32,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_struct_with_two_field_twin_sync_sse(
    arg: StructWithTwoFieldTwinSyncSse,
) -> StructWithTwoFieldTwinSyncSse {
    arg
}

pub struct TupleStructWithOneFieldTwinSyncSse(pub i32);

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_tuple_struct_with_one_field_twin_sync_sse(
    arg: TupleStructWithOneFieldTwinSyncSse,
) -> TupleStructWithOneFieldTwinSyncSse {
    arg
}

pub struct TupleStructWithTwoFieldTwinSyncSse(pub i32, pub i32);

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_tuple_struct_with_two_field_twin_sync_sse(
    arg: TupleStructWithTwoFieldTwinSyncSse,
) -> TupleStructWithTwoFieldTwinSyncSse {
    arg
}

#[frb]
pub struct StructWithFieldRenameTwinSyncSse {
    #[frb(name = "renamed_field")]
    pub class: i32,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_for_struct_with_field_rename_twin_sync_sse(
    arg: StructWithFieldRenameTwinSyncSse,
) -> StructWithFieldRenameTwinSyncSse {
    arg
}

pub struct StructWithDartKeywordFieldTwinSyncSse {
    pub class: i32,
    pub interface: i64,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_for_struct_with_dart_keyword_field_twin_sync_sse(
    arg: StructWithDartKeywordFieldTwinSyncSse,
) -> StructWithDartKeywordFieldTwinSyncSse {
    arg
}
