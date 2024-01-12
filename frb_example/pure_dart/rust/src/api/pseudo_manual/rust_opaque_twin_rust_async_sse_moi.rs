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
pub trait DartDebugTwinRustAsyncSseMoi: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinRustAsyncSseMoi for T {}

pub enum EnumOpaqueTwinRustAsyncSseMoi {
    Struct(RustOpaqueMoi<HideData>),
    Primitive(RustOpaqueMoi<i32>),
    TraitObj(RustOpaqueMoi<Box<dyn DartDebugTwinRustAsyncSseMoi>>),
    Mutex(RustOpaqueMoi<Mutex<HideData>>),
    RwLock(RustOpaqueMoi<RwLock<HideData>>),
}

/// [`HideData`] has private fields.
pub struct OpaqueNestedTwinRustAsyncSseMoi {
    pub first: RustOpaqueMoi<HideData>,
    pub second: RustOpaqueMoi<HideData>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn create_opaque_twin_rust_async_sse_moi() -> RustOpaqueMoi<HideData> {
    RustOpaque::new(HideData::new())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn create_option_opaque_twin_rust_async_sse_moi(
    opaque: Option<RustOpaqueMoi<HideData>>,
) -> Option<RustOpaqueMoi<HideData>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn sync_create_opaque_twin_rust_async_sse_moi() -> SyncReturn<RustOpaqueMoi<HideData>> {
//     SyncReturn(RustOpaque::new(HideData::new()))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn create_array_opaque_enum_twin_rust_async_sse_moi() -> [EnumOpaqueTwinRustAsyncSseMoi; 5] {
    [
        EnumOpaqueTwinRustAsyncSseMoi::Struct(RustOpaque::new(HideData::new())),
        EnumOpaqueTwinRustAsyncSseMoi::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinRustAsyncSseMoi::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinRustAsyncSseMoi::Mutex(RustOpaque::new(Mutex::new(HideData::new()))),
        EnumOpaqueTwinRustAsyncSseMoi::RwLock(RustOpaque::new(RwLock::new(HideData::new()))),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn run_enum_opaque_twin_rust_async_sse_moi(opaque: EnumOpaqueTwinRustAsyncSseMoi) -> String {
    match opaque {
        EnumOpaqueTwinRustAsyncSseMoi::Struct(s) => s.hide_data(),
        EnumOpaqueTwinRustAsyncSseMoi::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinRustAsyncSseMoi::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinRustAsyncSseMoi::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaqueTwinRustAsyncSseMoi::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn run_opaque_twin_rust_async_sse_moi(opaque: RustOpaqueMoi<HideData>) -> String {
    opaque.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn run_opaque_with_delay_twin_rust_async_sse_moi(opaque: RustOpaqueMoi<HideData>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn opaque_array_twin_rust_async_sse_moi() -> [RustOpaqueMoi<HideData>; 2] {
    [
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn sync_create_non_clone_twin_rust_async_sse_moi() -> SyncReturn<RustOpaqueMoi<NonCloneData>> {
//     SyncReturn(RustOpaque::new(NonCloneData::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn run_non_clone_twin_rust_async_sse_moi(clone: RustOpaqueMoi<NonCloneData>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn create_sync_opaque_twin_rust_async_sse_moi() -> RustOpaqueMoi<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn sync_create_sync_opaque_twin_rust_async_sse_moi() -> SyncReturn<RustOpaqueMoi<NonSendHideData>> {
//     SyncReturn(RustOpaque::new(NonSendHideData::new()))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn opaque_array_run_twin_rust_async_sse_moi(data: [RustOpaqueMoi<HideData>; 2]) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn opaque_vec_twin_rust_async_sse_moi() -> Vec<RustOpaqueMoi<HideData>> {
    vec![
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn opaque_vec_run_twin_rust_async_sse_moi(data: Vec<RustOpaqueMoi<HideData>>) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn create_nested_opaque_twin_rust_async_sse_moi() -> OpaqueNestedTwinRustAsyncSseMoi {
    OpaqueNestedTwinRustAsyncSseMoi {
        first: RustOpaque::new(HideData::new()),
        second: RustOpaque::new(HideData::new()),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn run_nested_opaque_twin_rust_async_sse_moi(opaque: OpaqueNestedTwinRustAsyncSseMoi) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn unwrap_rust_opaque_twin_rust_async_sse_moi(opaque: RustOpaqueMoi<HideData>) -> Result<String> {
    let data: HideData = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn frb_generator_test_twin_rust_async_sse_moi() -> RustOpaqueMoi<FrbOpaqueReturn> {
    panic!("dummy code");
}
