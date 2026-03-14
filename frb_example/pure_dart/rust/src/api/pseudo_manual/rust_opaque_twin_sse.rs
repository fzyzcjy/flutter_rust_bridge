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

pub struct HideDataTwinSse(pub HideDataRaw);
pub struct NonCloneDataTwinSse(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinSse;

/// Opaque types
pub trait DartDebugTwinSse: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinSse for T {}

pub enum EnumOpaqueTwinSse {
    Struct(RustOpaque<HideDataTwinSse>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinSse>>),
    Mutex(RustOpaque<Mutex<HideDataTwinSse>>),
    RwLock(RustOpaque<RwLock<HideDataTwinSse>>),
    Nothing,
}

/// [`HideDataTwinSse`] has private fields.
pub struct OpaqueNestedTwinSse {
    pub first: RustOpaque<HideDataTwinSse>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: RustOpaqueNom<HideDataTwinSse>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_opaque_twin_sse() -> RustOpaque<HideDataTwinSse> {
    RustOpaque::new(HideDataTwinSse(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_option_opaque_twin_sse(
    opaque: Option<RustOpaque<HideDataTwinSse>>,
) -> Option<RustOpaque<HideDataTwinSse>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_opaque_twin_sse() -> SyncReturn<RustOpaque<HideDataTwinSse>> {
//     SyncReturn(RustOpaque::new(HideDataTwinSse(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(serialize)]
pub fn create_array_opaque_enum_twin_sse() -> [EnumOpaqueTwinSse; 5] {
    [
        EnumOpaqueTwinSse::Struct(RustOpaque::new(HideDataTwinSse(HideDataRaw::new()))),
        EnumOpaqueTwinSse::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinSse::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSse::Mutex(RustOpaque::new(Mutex::new(HideDataTwinSse(
            HideDataRaw::new(),
        )))),
        EnumOpaqueTwinSse::RwLock(RustOpaque::new(RwLock::new(HideDataTwinSse(
            HideDataRaw::new(),
        )))),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_enum_opaque_twin_sse(opaque: EnumOpaqueTwinSse) -> String {
    match opaque {
        EnumOpaqueTwinSse::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinSse::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSse::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSse::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinSse::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
        _ => "nothing".to_owned(),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_opaque_twin_sse(opaque: RustOpaque<HideDataTwinSse>) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_opaque_with_delay_twin_sse(opaque: RustOpaque<HideDataTwinSse>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_array_twin_sse() -> [RustOpaque<HideDataTwinSse>; 2] {
    [
        RustOpaque::new(HideDataTwinSse(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinSse(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_non_clone_twin_sse() -> SyncReturn<RustOpaque<NonCloneDataTwinSse>> {
//     SyncReturn(RustOpaque::new(NonCloneDataTwinSse::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(serialize)]
pub fn run_non_clone_twin_sse(clone: RustOpaque<NonCloneDataTwinSse>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_array_run_twin_sse(data: [RustOpaque<HideDataTwinSse>; 2]) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_vec_twin_sse() -> Vec<RustOpaque<HideDataTwinSse>> {
    vec![
        RustOpaque::new(HideDataTwinSse(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinSse(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_vec_run_twin_sse(data: Vec<RustOpaque<HideDataTwinSse>>) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_nested_opaque_twin_sse() -> OpaqueNestedTwinSse {
    OpaqueNestedTwinSse {
        first: RustOpaque::new(HideDataTwinSse(HideDataRaw::new())),
        second: RustOpaque::new(HideDataTwinSse(HideDataRaw::new())),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_nested_opaque_twin_sse(opaque: OpaqueNestedTwinSse) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(serialize)]
pub fn unwrap_rust_opaque_twin_sse(opaque: RustOpaque<HideDataTwinSse>) -> Result<String> {
    let data: HideDataTwinSse = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(serialize)]
pub fn frb_generator_test_twin_sse() -> RustOpaque<FrbOpaqueReturnTwinSse> {
    panic!("dummy code");
}
