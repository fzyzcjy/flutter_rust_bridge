// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `external_type_in_crate.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::auxiliary::new_module_system::{use_new_module_system, NewSimpleStruct};
use crate::auxiliary::old_module_system::{use_old_module_system, OldSimpleStruct};
use crate::auxiliary::sample_types::{MyEnum, MyStruct};

// Function that uses imported struct (from within this crate)
#[flutter_rust_bridge::frb(sync)]
pub fn use_imported_struct_twin_sync(my_struct: MyStruct) -> bool {
    my_struct.content
}

// Function that uses imported enum (from within this crate)
#[flutter_rust_bridge::frb(sync)]
pub fn use_imported_enum_twin_sync(my_enum: MyEnum) -> bool {
    match my_enum {
        MyEnum::False => false,
        MyEnum::True => true,
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn call_old_module_system_twin_sync() -> OldSimpleStruct {
    use_old_module_system(2)
}

#[flutter_rust_bridge::frb(sync)]
pub fn call_new_module_system_twin_sync() -> NewSimpleStruct {
    use_new_module_system(1)
}
