// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub enum EnumSimpleTwinSync {
    A,
    B,
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_simple_twin_sync(arg: EnumSimpleTwinSync) -> EnumSimpleTwinSync {
    arg
}

pub enum EnumWithItemMixedTwinSync {
    A,
    B(Vec<u8>),
    C { c_field: String },
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_item_mixed_twin_sync(
    arg: EnumWithItemMixedTwinSync,
) -> EnumWithItemMixedTwinSync {
    arg
}

pub enum EnumWithItemTupleTwinSync {
    A(Vec<u8>),
    B(Vec<i32>),
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_item_tuple_twin_sync(
    arg: EnumWithItemTupleTwinSync,
) -> EnumWithItemTupleTwinSync {
    arg
}

pub enum EnumWithItemStructTwinSync {
    A { a_field: Vec<u8> },
    B { b_field: Vec<i32> },
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_item_struct_twin_sync(
    arg: EnumWithItemStructTwinSync,
) -> EnumWithItemStructTwinSync {
    arg
}
