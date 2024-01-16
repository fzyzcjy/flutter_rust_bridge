// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#![allow(unused)]

use flutter_rust_bridge::DartOpaque;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// TODO about sync
// pub async fn sync_accept_dart_opaque_twin_rust_async(opaque: DartOpaque) -> SyncReturn<String> {
//     drop(opaque);
//     SyncReturn("test".to_owned())
// }

pub async fn async_accept_dart_opaque_twin_rust_async(opaque: DartOpaque) -> String {
    drop(opaque);
    "async test".to_owned()
}

pub async fn loop_back_twin_rust_async(opaque: DartOpaque) -> DartOpaque {
    opaque
}

pub async fn loop_back_option_twin_rust_async(opaque: DartOpaque) -> Option<DartOpaque> {
    Some(opaque)
}

pub async fn loop_back_array_twin_rust_async(opaque: DartOpaque) -> [DartOpaque; 1] {
    [opaque]
}

pub async fn loop_back_vec_twin_rust_async(opaque: DartOpaque) -> Vec<DartOpaque> {
    vec![opaque]
}

pub async fn loop_back_option_get_twin_rust_async(opaque: Option<DartOpaque>) {}

pub async fn loop_back_array_get_twin_rust_async(opaque: [DartOpaque; 1]) {}

pub async fn loop_back_vec_get_twin_rust_async(opaque: Vec<DartOpaque>) {}

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
pub async fn panic_unwrap_dart_opaque_twin_rust_async(opaque: DartOpaque) {
    let _handle = opaque.into_inner().unwrap();
}

pub enum EnumDartOpaqueTwinRustAsync {
    Primitive(i32),
    Opaque(DartOpaque),
}

pub struct DartOpaqueNestedTwinRustAsync {
    pub first: DartOpaque,
    pub second: DartOpaque,
}

// TODO about sync
// pub async fn sync_loopback_twin_rust_async(opaque: DartOpaque) -> SyncReturn<DartOpaque> {
//     SyncReturn(opaque)
// }
//
// pub async fn sync_option_loopback_twin_rust_async(opaque: Option<DartOpaque>) -> SyncReturn<Option<DartOpaque>> {
//     SyncReturn(opaque)
// }
//
// pub async fn sync_option_dart_opaque_twin_rust_async(opaque: DartOpaque) -> Result<SyncReturn<Option<DartOpaque>>> {
//     Ok(SyncReturn(Some(opaque)))
// }

pub async fn create_nested_dart_opaque_twin_rust_async(
    opaque1: DartOpaque,
    opaque2: DartOpaque,
) -> DartOpaqueNestedTwinRustAsync {
    DartOpaqueNestedTwinRustAsync {
        first: opaque1,
        second: opaque2,
    }
}

pub async fn get_nested_dart_opaque_twin_rust_async(opaque: DartOpaqueNestedTwinRustAsync) {}

pub async fn create_enum_dart_opaque_twin_rust_async(
    opaque: DartOpaque,
) -> EnumDartOpaqueTwinRustAsync {
    EnumDartOpaqueTwinRustAsync::Opaque(opaque)
}

pub async fn get_enum_dart_opaque_twin_rust_async(opaque: EnumDartOpaqueTwinRustAsync) {}

lazy_static! {
    static ref DART_OPAQUE: Mutex<HashMap<i32, DartOpaque>> = Default::default();
}

pub async fn set_static_dart_opaque_twin_rust_async(id: i32, opaque: DartOpaque) {
    DART_OPAQUE.lock().unwrap().insert(id, opaque);
}

pub async fn drop_static_dart_opaque_twin_rust_async(id: i32) {
    drop(DART_OPAQUE.lock().unwrap().remove(&id));
}

pub async fn clone_dart_opaque_twin_rust_async(opaque: DartOpaque) -> Vec<DartOpaque> {
    vec![opaque; 10]
}
