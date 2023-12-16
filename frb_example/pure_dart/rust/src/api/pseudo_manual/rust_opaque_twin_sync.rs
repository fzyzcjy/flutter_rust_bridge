// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub use crate::auxiliary::sample_types::{
    FrbOpaqueReturn, HideData, NonCloneData, NonSendHideData,
};
use anyhow::Result;
use flutter_rust_bridge::{opaque_dyn, DartSafe, RustOpaque};
use std::fmt::Debug;
use std::ops::Deref;
pub use std::sync::{Mutex, RwLock};

/// Opaque types
pub trait DartDebugTwinSync: DartSafe + Debug + Send + Sync {}
impl<T: DartSafe + Debug + Send + Sync> DartDebugTwinSync for T {}

pub enum EnumOpaqueTwinSync {
    Struct(RustOpaque<HideData>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinSync>>),
    Mutex(RustOpaque<Mutex<HideData>>),
    RwLock(RustOpaque<RwLock<HideData>>),
}

/// [`HideData`] has private fields.
pub struct OpaqueNestedTwinSync {
    pub first: RustOpaque<HideData>,
    pub second: RustOpaque<HideData>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_opaque_twin_sync() -> RustOpaque<HideData> {
    RustOpaque::new(HideData::new())
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_option_opaque_twin_sync(
    opaque: Option<RustOpaque<HideData>>,
) -> Option<RustOpaque<HideData>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(sync)] pub fn sync_create_opaque_twin_sync() -> SyncReturn<RustOpaque<HideData>> {
//     SyncReturn(RustOpaque::new(HideData::new()))
// }

#[flutter_rust_bridge::frb(sync)]
pub fn create_array_opaque_enum_twin_sync() -> [EnumOpaqueTwinSync; 5] {
    [
        EnumOpaqueTwinSync::Struct(RustOpaque::new(HideData::new())),
        EnumOpaqueTwinSync::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinSync::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSync::Mutex(RustOpaque::new(Mutex::new(HideData::new()))),
        EnumOpaqueTwinSync::RwLock(RustOpaque::new(RwLock::new(HideData::new()))),
    ]
}

#[flutter_rust_bridge::frb(sync)]
pub fn run_enum_opaque_twin_sync(opaque: EnumOpaqueTwinSync) -> String {
    match opaque {
        EnumOpaqueTwinSync::Struct(s) => s.hide_data(),
        EnumOpaqueTwinSync::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSync::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSync::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaqueTwinSync::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_twin_sync(opaque: RustOpaque<HideData>) -> String {
    opaque.hide_data()
}

#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_with_delay_twin_sync(opaque: RustOpaque<HideData>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.hide_data()
}

#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_twin_sync() -> [RustOpaque<HideData>; 2] {
    [
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(sync)] pub fn sync_create_non_clone_twin_sync() -> SyncReturn<RustOpaque<NonCloneData>> {
//     SyncReturn(RustOpaque::new(NonCloneData::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_non_clone_twin_sync(clone: RustOpaque<NonCloneData>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_sync_opaque_twin_sync() -> RustOpaque<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

// TODO about sync
// #[flutter_rust_bridge::frb(sync)] pub fn sync_create_sync_opaque_twin_sync() -> SyncReturn<RustOpaque<NonSendHideData>> {
//     SyncReturn(RustOpaque::new(NonSendHideData::new()))
// }

#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_run_twin_sync(data: [RustOpaque<HideData>; 2]) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_twin_sync() -> Vec<RustOpaque<HideData>> {
    vec![
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_run_twin_sync(data: Vec<RustOpaque<HideData>>) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_nested_opaque_twin_sync() -> OpaqueNestedTwinSync {
    OpaqueNestedTwinSync {
        first: RustOpaque::new(HideData::new()),
        second: RustOpaque::new(HideData::new()),
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn run_nested_opaque_twin_sync(opaque: OpaqueNestedTwinSync) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

#[flutter_rust_bridge::frb(sync)]
pub fn unwrap_rust_opaque_twin_sync(opaque: RustOpaque<HideData>) -> Result<String> {
    let data: HideData = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(sync)]
pub fn frb_generator_test_twin_sync() -> RustOpaque<FrbOpaqueReturn> {
    panic!("dummy code");
}
