// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#![allow(unused)]

use flutter_rust_bridge::DartOpaque;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_accept_dart_opaque_twin_sse(opaque: DartOpaque) -> SyncReturn<String> {
//     drop(opaque);
//     SyncReturn("test".to_owned())
// }

#[flutter_rust_bridge::frb(serialize)]
pub fn async_accept_dart_opaque_twin_sse(opaque: DartOpaque) -> String {
    drop(opaque);
    "async test".to_owned()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn loop_back_twin_sse(opaque: DartOpaque) -> DartOpaque {
    opaque
}

#[flutter_rust_bridge::frb(serialize)]
pub fn loop_back_option_twin_sse(opaque: DartOpaque) -> Option<DartOpaque> {
    Some(opaque)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn loop_back_array_twin_sse(opaque: DartOpaque) -> [DartOpaque; 1] {
    [opaque]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn loop_back_vec_twin_sse(opaque: DartOpaque) -> Vec<DartOpaque> {
    vec![opaque]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn loop_back_option_get_twin_sse(opaque: Option<DartOpaque>) {}

#[flutter_rust_bridge::frb(serialize)]
pub fn loop_back_array_get_twin_sse(opaque: [DartOpaque; 1]) {}

#[flutter_rust_bridge::frb(serialize)]
pub fn loop_back_vec_get_twin_sse(opaque: Vec<DartOpaque>) {}

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
#[flutter_rust_bridge::frb(serialize)]
pub fn panic_unwrap_dart_opaque_twin_sse(opaque: DartOpaque) {
    let _handle = opaque.into_inner().unwrap();
}

pub enum EnumDartOpaqueTwinSse {
    Primitive(i32),
    Opaque(DartOpaque),
}

pub struct DartOpaqueNestedTwinSse {
    pub first: DartOpaque,
    pub second: DartOpaque,
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_loopback_twin_sse(opaque: DartOpaque) -> SyncReturn<DartOpaque> {
//     SyncReturn(opaque)
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_option_loopback_twin_sse(opaque: Option<DartOpaque>) -> SyncReturn<Option<DartOpaque>> {
//     SyncReturn(opaque)
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_option_dart_opaque_twin_sse(opaque: DartOpaque) -> Result<SyncReturn<Option<DartOpaque>>> {
//     Ok(SyncReturn(Some(opaque)))
// }

#[flutter_rust_bridge::frb(serialize)]
pub fn create_nested_dart_opaque_twin_sse(
    opaque1: DartOpaque,
    opaque2: DartOpaque,
) -> DartOpaqueNestedTwinSse {
    DartOpaqueNestedTwinSse {
        first: opaque1,
        second: opaque2,
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn get_nested_dart_opaque_twin_sse(opaque: DartOpaqueNestedTwinSse) {}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_enum_dart_opaque_twin_sse(opaque: DartOpaque) -> EnumDartOpaqueTwinSse {
    EnumDartOpaqueTwinSse::Opaque(opaque)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn get_enum_dart_opaque_twin_sse(opaque: EnumDartOpaqueTwinSse) {}

lazy_static! {
    static ref DART_OPAQUE: Mutex<HashMap<i32, DartOpaque>> = Default::default();
}

#[flutter_rust_bridge::frb(serialize)]
pub fn set_static_dart_opaque_twin_sse(id: i32, opaque: DartOpaque) {
    DART_OPAQUE.lock().unwrap().insert(id, opaque);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn drop_static_dart_opaque_twin_sse(id: i32) {
    drop(DART_OPAQUE.lock().unwrap().remove(&id));
}

#[flutter_rust_bridge::frb(serialize)]
pub fn clone_dart_opaque_twin_sse(opaque: DartOpaque) -> Vec<DartOpaque> {
    vec![opaque; 10]
}
