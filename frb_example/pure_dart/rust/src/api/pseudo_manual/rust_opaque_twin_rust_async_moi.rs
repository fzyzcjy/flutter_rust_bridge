// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

pub use crate::auxiliary::sample_types::{
    FrbOpaqueReturn, HideData, NonCloneData, NonSendHideData,
};
use anyhow::Result;
use flutter_rust_bridge::{opaque_dyn, RustOpaque};
use std::fmt::Debug;
use std::ops::Deref;
pub use std::sync::{Mutex, RwLock};

/// Opaque types
pub trait DartDebugTwinRustAsyncMoi: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinRustAsyncMoi for T {}

pub enum EnumOpaqueTwinRustAsyncMoi {
    Struct(RustOpaque<HideData>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinRustAsyncMoi>>),
    Mutex(RustOpaque<Mutex<HideData>>),
    RwLock(RustOpaque<RwLock<HideData>>),
}

/// [`HideData`] has private fields.
pub struct OpaqueNestedTwinRustAsyncMoi {
    pub first: RustOpaque<HideData>,
    pub second: RustOpaque<HideData>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_opaque_twin_rust_async_moi() -> RustOpaque<HideData> {
    RustOpaque::new(HideData::new())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_option_opaque_twin_rust_async_moi(
    opaque: Option<RustOpaque<HideData>>,
) -> Option<RustOpaque<HideData>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn sync_create_opaque_twin_rust_async_moi() -> SyncReturn<RustOpaque<HideData>> {
//     SyncReturn(RustOpaque::new(HideData::new()))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_array_opaque_enum_twin_rust_async_moi() -> [EnumOpaqueTwinRustAsyncMoi; 5] {
    [
        EnumOpaqueTwinRustAsyncMoi::Struct(RustOpaque::new(HideData::new())),
        EnumOpaqueTwinRustAsyncMoi::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinRustAsyncMoi::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinRustAsyncMoi::Mutex(RustOpaque::new(Mutex::new(HideData::new()))),
        EnumOpaqueTwinRustAsyncMoi::RwLock(RustOpaque::new(RwLock::new(HideData::new()))),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_enum_opaque_twin_rust_async_moi(opaque: EnumOpaqueTwinRustAsyncMoi) -> String {
    match opaque {
        EnumOpaqueTwinRustAsyncMoi::Struct(s) => s.hide_data(),
        EnumOpaqueTwinRustAsyncMoi::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinRustAsyncMoi::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinRustAsyncMoi::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaqueTwinRustAsyncMoi::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_opaque_twin_rust_async_moi(opaque: RustOpaque<HideData>) -> String {
    opaque.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_opaque_with_delay_twin_rust_async_moi(opaque: RustOpaque<HideData>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_array_twin_rust_async_moi() -> [RustOpaque<HideData>; 2] {
    [
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn sync_create_non_clone_twin_rust_async_moi() -> SyncReturn<RustOpaque<NonCloneData>> {
//     SyncReturn(RustOpaque::new(NonCloneData::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_non_clone_twin_rust_async_moi(clone: RustOpaque<NonCloneData>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_sync_opaque_twin_rust_async_moi() -> RustOpaque<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn sync_create_sync_opaque_twin_rust_async_moi() -> SyncReturn<RustOpaque<NonSendHideData>> {
//     SyncReturn(RustOpaque::new(NonSendHideData::new()))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_array_run_twin_rust_async_moi(data: [RustOpaque<HideData>; 2]) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_vec_twin_rust_async_moi() -> Vec<RustOpaque<HideData>> {
    vec![
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_vec_run_twin_rust_async_moi(data: Vec<RustOpaque<HideData>>) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_nested_opaque_twin_rust_async_moi() -> OpaqueNestedTwinRustAsyncMoi {
    OpaqueNestedTwinRustAsyncMoi {
        first: RustOpaque::new(HideData::new()),
        second: RustOpaque::new(HideData::new()),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_nested_opaque_twin_rust_async_moi(opaque: OpaqueNestedTwinRustAsyncMoi) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn unwrap_rust_opaque_twin_rust_async_moi(
    opaque: RustOpaque<HideData>,
) -> Result<String> {
    let data: HideData = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn frb_generator_test_twin_rust_async_moi() -> RustOpaque<FrbOpaqueReturn> {
    panic!("dummy code");
}
