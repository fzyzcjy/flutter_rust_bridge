// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

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

pub struct HideDataTwinSyncSse(pub HideDataRaw);
pub struct NonCloneDataTwinSyncSse(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinSyncSse;

/// Opaque types
pub trait DartDebugTwinSyncSse: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinSyncSse for T {}

pub enum EnumOpaqueTwinSyncSse {
    Struct(RustOpaque<HideDataTwinSyncSse>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinSyncSse>>),
    Mutex(RustOpaque<Mutex<HideDataTwinSyncSse>>),
    RwLock(RustOpaque<RwLock<HideDataTwinSyncSse>>),
    Nothing,
}

/// [`HideDataTwinSyncSse`] has private fields.
pub struct OpaqueNestedTwinSyncSse {
    pub first: RustOpaque<HideDataTwinSyncSse>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: RustOpaqueNom<HideDataTwinSyncSse>,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_opaque_twin_sync_sse() -> RustOpaque<HideDataTwinSyncSse> {
    RustOpaque::new(HideDataTwinSyncSse(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_option_opaque_twin_sync_sse(
    opaque: Option<RustOpaque<HideDataTwinSyncSse>>,
) -> Option<RustOpaque<HideDataTwinSyncSse>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_opaque_twin_sync_sse() -> SyncReturn<RustOpaque<HideDataTwinSyncSse>> {
//     SyncReturn(RustOpaque::new(HideDataTwinSyncSse(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_array_opaque_enum_twin_sync_sse() -> [EnumOpaqueTwinSyncSse; 5] {
    [
        EnumOpaqueTwinSyncSse::Struct(RustOpaque::new(HideDataTwinSyncSse(HideDataRaw::new()))),
        EnumOpaqueTwinSyncSse::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinSyncSse::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSyncSse::Mutex(RustOpaque::new(Mutex::new(HideDataTwinSyncSse(
            HideDataRaw::new(),
        )))),
        EnumOpaqueTwinSyncSse::RwLock(RustOpaque::new(RwLock::new(HideDataTwinSyncSse(
            HideDataRaw::new(),
        )))),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_enum_opaque_twin_sync_sse(opaque: EnumOpaqueTwinSyncSse) -> String {
    match opaque {
        EnumOpaqueTwinSyncSse::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinSyncSse::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSyncSse::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSyncSse::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinSyncSse::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
        _ => "nothing".to_owned(),
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_twin_sync_sse(opaque: RustOpaque<HideDataTwinSyncSse>) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_with_delay_twin_sync_sse(opaque: RustOpaque<HideDataTwinSyncSse>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_twin_sync_sse() -> [RustOpaque<HideDataTwinSyncSse>; 2] {
    [
        RustOpaque::new(HideDataTwinSyncSse(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinSyncSse(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_non_clone_twin_sync_sse() -> SyncReturn<RustOpaque<NonCloneDataTwinSyncSse>> {
//     SyncReturn(RustOpaque::new(NonCloneDataTwinSyncSse::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_non_clone_twin_sync_sse(clone: RustOpaque<NonCloneDataTwinSyncSse>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_run_twin_sync_sse(data: [RustOpaque<HideDataTwinSyncSse>; 2]) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_twin_sync_sse() -> Vec<RustOpaque<HideDataTwinSyncSse>> {
    vec![
        RustOpaque::new(HideDataTwinSyncSse(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinSyncSse(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_run_twin_sync_sse(data: Vec<RustOpaque<HideDataTwinSyncSse>>) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_nested_opaque_twin_sync_sse() -> OpaqueNestedTwinSyncSse {
    OpaqueNestedTwinSyncSse {
        first: RustOpaque::new(HideDataTwinSyncSse(HideDataRaw::new())),
        second: RustOpaque::new(HideDataTwinSyncSse(HideDataRaw::new())),
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_nested_opaque_twin_sync_sse(opaque: OpaqueNestedTwinSyncSse) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn unwrap_rust_opaque_twin_sync_sse(opaque: RustOpaque<HideDataTwinSyncSse>) -> Result<String> {
    let data: HideDataTwinSyncSse = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn frb_generator_test_twin_sync_sse() -> RustOpaque<FrbOpaqueReturnTwinSyncSse> {
    panic!("dummy code");
}
