// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

pub use crate::auxiliary::sample_types::{HideDataRaw, NonCloneDataRaw};
#[allow(unused_imports)]
use crate::frb_generated::{RustOpaque, RustOpaqueMoi};
use anyhow::Result;
#[allow(unused_imports)]
use flutter_rust_bridge::{opaque_dyn, RustOpaqueNom};
use std::fmt::Debug;
use std::ops::Deref;
pub use std::sync::{Mutex, RwLock};

pub struct HideDataTwinSync(pub HideDataRaw);
pub struct NonCloneDataTwinSync(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinSync;

/// Opaque types
pub trait DartDebugTwinSync: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinSync for T {}

pub enum EnumOpaqueTwinSync {
    Struct(RustOpaque<HideDataTwinSync>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinSync>>),
    Mutex(RustOpaque<Mutex<HideDataTwinSync>>),
    RwLock(RustOpaque<RwLock<HideDataTwinSync>>),
    Nothing,
}

/// [`HideDataTwinSync`] has private fields.
pub struct OpaqueNestedTwinSync {
    pub first: RustOpaque<HideDataTwinSync>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: RustOpaqueMoi<HideDataTwinSync>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_opaque_twin_sync() -> RustOpaque<HideDataTwinSync> {
    RustOpaque::new(HideDataTwinSync(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_option_opaque_twin_sync(
    opaque: Option<RustOpaque<HideDataTwinSync>>,
) -> Option<RustOpaque<HideDataTwinSync>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(sync)] pub fn sync_create_opaque_twin_sync() -> SyncReturn<RustOpaque<HideDataTwinSync>> {
//     SyncReturn(RustOpaque::new(HideDataTwinSync(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(sync)]
pub fn create_array_opaque_enum_twin_sync() -> [EnumOpaqueTwinSync; 5] {
    [
        EnumOpaqueTwinSync::Struct(RustOpaque::new(HideDataTwinSync(HideDataRaw::new()))),
        EnumOpaqueTwinSync::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinSync::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSync::Mutex(RustOpaque::new(Mutex::new(HideDataTwinSync(
            HideDataRaw::new(),
        )))),
        EnumOpaqueTwinSync::RwLock(RustOpaque::new(RwLock::new(HideDataTwinSync(
            HideDataRaw::new(),
        )))),
    ]
}

#[flutter_rust_bridge::frb(sync)]
pub fn run_enum_opaque_twin_sync(opaque: EnumOpaqueTwinSync) -> String {
    match opaque {
        EnumOpaqueTwinSync::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinSync::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSync::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSync::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinSync::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
        _ => "nothing".to_owned(),
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_twin_sync(opaque: RustOpaque<HideDataTwinSync>) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_with_delay_twin_sync(opaque: RustOpaque<HideDataTwinSync>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_twin_sync() -> [RustOpaque<HideDataTwinSync>; 2] {
    [
        RustOpaque::new(HideDataTwinSync(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinSync(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(sync)] pub fn sync_create_non_clone_twin_sync() -> SyncReturn<RustOpaque<NonCloneDataTwinSync>> {
//     SyncReturn(RustOpaque::new(NonCloneDataTwinSync::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_non_clone_twin_sync(clone: RustOpaque<NonCloneDataTwinSync>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_run_twin_sync(data: [RustOpaque<HideDataTwinSync>; 2]) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_twin_sync() -> Vec<RustOpaque<HideDataTwinSync>> {
    vec![
        RustOpaque::new(HideDataTwinSync(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinSync(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_run_twin_sync(data: Vec<RustOpaque<HideDataTwinSync>>) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_nested_opaque_twin_sync() -> OpaqueNestedTwinSync {
    OpaqueNestedTwinSync {
        first: RustOpaque::new(HideDataTwinSync(HideDataRaw::new())),
        second: RustOpaque::new(HideDataTwinSync(HideDataRaw::new())),
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn run_nested_opaque_twin_sync(opaque: OpaqueNestedTwinSync) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(sync)]
pub fn unwrap_rust_opaque_twin_sync(opaque: RustOpaque<HideDataTwinSync>) -> Result<String> {
    let data: HideDataTwinSync = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(sync)]
pub fn frb_generator_test_twin_sync() -> RustOpaque<FrbOpaqueReturnTwinSync> {
    panic!("dummy code");
}
