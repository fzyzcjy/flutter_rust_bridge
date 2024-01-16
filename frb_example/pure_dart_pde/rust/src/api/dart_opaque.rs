// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#![allow(unused)]

use flutter_rust_bridge::DartOpaque;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// TODO about sync
// pub fn sync_accept_dart_opaque_twin_normal(opaque: DartOpaque) -> SyncReturn<String> {
//     drop(opaque);
//     SyncReturn("test".to_owned())
// }

pub fn async_accept_dart_opaque_twin_normal(opaque: DartOpaque) -> String {
    drop(opaque);
    "async test".to_owned()
}

pub fn loop_back_twin_normal(opaque: DartOpaque) -> DartOpaque {
    opaque
}

pub fn loop_back_option_twin_normal(opaque: DartOpaque) -> Option<DartOpaque> {
    Some(opaque)
}

pub fn loop_back_array_twin_normal(opaque: DartOpaque) -> [DartOpaque; 1] {
    [opaque]
}

pub fn loop_back_vec_twin_normal(opaque: DartOpaque) -> Vec<DartOpaque> {
    vec![opaque]
}

pub fn loop_back_option_get_twin_normal(opaque: Option<DartOpaque>) {}

pub fn loop_back_array_get_twin_normal(opaque: [DartOpaque; 1]) {}

pub fn loop_back_vec_get_twin_normal(opaque: Vec<DartOpaque>) {}

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
pub fn panic_unwrap_dart_opaque_twin_normal(opaque: DartOpaque) {
    let _handle = opaque.into_inner().unwrap();
}

pub enum EnumDartOpaqueTwinNormal {
    Primitive(i32),
    Opaque(DartOpaque),
}

pub struct DartOpaqueNestedTwinNormal {
    pub first: DartOpaque,
    pub second: DartOpaque,
}

// TODO about sync
// pub fn sync_loopback_twin_normal(opaque: DartOpaque) -> SyncReturn<DartOpaque> {
//     SyncReturn(opaque)
// }
//
// pub fn sync_option_loopback_twin_normal(opaque: Option<DartOpaque>) -> SyncReturn<Option<DartOpaque>> {
//     SyncReturn(opaque)
// }
//
// pub fn sync_option_dart_opaque_twin_normal(opaque: DartOpaque) -> Result<SyncReturn<Option<DartOpaque>>> {
//     Ok(SyncReturn(Some(opaque)))
// }

pub fn create_nested_dart_opaque_twin_normal(
    opaque1: DartOpaque,
    opaque2: DartOpaque,
) -> DartOpaqueNestedTwinNormal {
    DartOpaqueNestedTwinNormal {
        first: opaque1,
        second: opaque2,
    }
}

pub fn get_nested_dart_opaque_twin_normal(opaque: DartOpaqueNestedTwinNormal) {}

pub fn create_enum_dart_opaque_twin_normal(opaque: DartOpaque) -> EnumDartOpaqueTwinNormal {
    EnumDartOpaqueTwinNormal::Opaque(opaque)
}

pub fn get_enum_dart_opaque_twin_normal(opaque: EnumDartOpaqueTwinNormal) {}

lazy_static! {
    static ref DART_OPAQUE: Mutex<HashMap<i32, DartOpaque>> = Default::default();
}

pub fn set_static_dart_opaque_twin_normal(id: i32, opaque: DartOpaque) {
    DART_OPAQUE.lock().unwrap().insert(id, opaque);
}

pub fn drop_static_dart_opaque_twin_normal(id: i32) {
    drop(DART_OPAQUE.lock().unwrap().remove(&id));
}

pub fn clone_dart_opaque_twin_normal(opaque: DartOpaque) -> Vec<DartOpaque> {
    vec![opaque; 10]
}
