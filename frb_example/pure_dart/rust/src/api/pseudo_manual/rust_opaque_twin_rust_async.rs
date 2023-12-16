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
pub trait DartDebugTwinRustAsync: DartSafe + Debug + Send + Sync {}
impl<T: DartSafe + Debug + Send + Sync> DartDebugTwinRustAsync for T {}

pub enum EnumOpaqueTwinRustAsync {
    Struct(RustOpaque<HideData>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinRustAsync>>),
    Mutex(RustOpaque<Mutex<HideData>>),
    RwLock(RustOpaque<RwLock<HideData>>),
}

/// [`HideData`] has private fields.
pub struct OpaqueNestedTwinRustAsync {
    pub first: RustOpaque<HideData>,
    pub second: RustOpaque<HideData>,
}

pub async fn create_opaque_twin_rust_async() -> RustOpaque<HideData> {
    RustOpaque::new(HideData::new())
}

pub async fn create_option_opaque_twin_rust_async(
    opaque: Option<RustOpaque<HideData>>,
) -> Option<RustOpaque<HideData>> {
    opaque
}

// TODO about sync
// pub async fn sync_create_opaque_twin_rust_async() -> SyncReturn<RustOpaque<HideData>> {
//     SyncReturn(RustOpaque::new(HideData::new()))
// }

pub async fn create_array_opaque_enum_twin_rust_async() -> [EnumOpaqueTwinRustAsync; 5] {
    [
        EnumOpaqueTwinRustAsync::Struct(RustOpaque::new(HideData::new())),
        EnumOpaqueTwinRustAsync::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinRustAsync::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinRustAsync::Mutex(RustOpaque::new(Mutex::new(HideData::new()))),
        EnumOpaqueTwinRustAsync::RwLock(RustOpaque::new(RwLock::new(HideData::new()))),
    ]
}

pub async fn run_enum_opaque_twin_rust_async(opaque: EnumOpaqueTwinRustAsync) -> String {
    match opaque {
        EnumOpaqueTwinRustAsync::Struct(s) => s.hide_data(),
        EnumOpaqueTwinRustAsync::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinRustAsync::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinRustAsync::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaqueTwinRustAsync::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

pub async fn run_opaque_twin_rust_async(opaque: RustOpaque<HideData>) -> String {
    opaque.hide_data()
}

pub async fn run_opaque_with_delay_twin_rust_async(opaque: RustOpaque<HideData>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    sleep(Duration::from_millis(1000));

    opaque.hide_data()
}

pub async fn opaque_array_twin_rust_async() -> [RustOpaque<HideData>; 2] {
    [
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

// TODO about sync
// pub async fn sync_create_non_clone_twin_rust_async() -> SyncReturn<RustOpaque<NonCloneData>> {
//     SyncReturn(RustOpaque::new(NonCloneData::new()))
// }

#[allow(clippy::redundant_clone)]
pub async fn run_non_clone_twin_rust_async(clone: RustOpaque<NonCloneData>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

pub async fn create_sync_opaque_twin_rust_async() -> RustOpaque<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

// TODO about sync
// pub async fn sync_create_sync_opaque_twin_rust_async() -> SyncReturn<RustOpaque<NonSendHideData>> {
//     SyncReturn(RustOpaque::new(NonSendHideData::new()))
// }

pub async fn opaque_array_run_twin_rust_async(data: [RustOpaque<HideData>; 2]) {
    for i in data {
        i.hide_data();
    }
}

pub async fn opaque_vec_twin_rust_async() -> Vec<RustOpaque<HideData>> {
    vec![
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

pub async fn opaque_vec_run_twin_rust_async(data: Vec<RustOpaque<HideData>>) {
    for i in data {
        i.hide_data();
    }
}

pub async fn create_nested_opaque_twin_rust_async() -> OpaqueNestedTwinRustAsync {
    OpaqueNestedTwinRustAsync {
        first: RustOpaque::new(HideData::new()),
        second: RustOpaque::new(HideData::new()),
    }
}

pub async fn run_nested_opaque_twin_rust_async(opaque: OpaqueNestedTwinRustAsync) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

pub async fn unwrap_rust_opaque_twin_rust_async(opaque: RustOpaque<HideData>) -> Result<String> {
    let data: HideData = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub async fn frb_generator_test_twin_rust_async() -> RustOpaque<FrbOpaqueReturn> {
    panic!("dummy code");
}
