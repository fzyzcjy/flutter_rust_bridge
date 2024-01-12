// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

pub use crate::auxiliary::sample_types::{HideDataRaw, NonCloneDataRaw};
use anyhow::Result;
use flutter_rust_bridge::{opaque_dyn, RustOpaque};
use std::fmt::Debug;
use std::ops::Deref;
pub use std::sync::{Mutex, RwLock};

pub struct HideDataTwinRustAsyncMoi(HideDataRaw);
pub struct NonCloneDataTwinRustAsyncMoi(NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinRustAsyncMoi;

/// Opaque types
pub trait DartDebugTwinRustAsyncMoi: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinRustAsyncMoi for T {}

pub enum EnumOpaqueTwinRustAsyncMoi {
    Struct(RustOpaque<HideDataTwinRustAsyncMoi>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinRustAsyncMoi>>),
    Mutex(RustOpaque<Mutex<HideDataTwinRustAsyncMoi>>),
    RwLock(RustOpaque<RwLock<HideDataTwinRustAsyncMoi>>),
}

/// [`HideDataTwinRustAsyncMoi`] has private fields.
pub struct OpaqueNestedTwinRustAsyncMoi {
    pub first: RustOpaque<HideDataTwinRustAsyncMoi>,
    pub second: RustOpaque<HideDataTwinRustAsyncMoi>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_opaque_twin_rust_async_moi() -> RustOpaque<HideDataTwinRustAsyncMoi> {
    RustOpaque::new(HideDataTwinRustAsyncMoi(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_option_opaque_twin_rust_async_moi(
    opaque: Option<RustOpaque<HideDataTwinRustAsyncMoi>>,
) -> Option<RustOpaque<HideDataTwinRustAsyncMoi>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn sync_create_opaque_twin_rust_async_moi() -> SyncReturn<RustOpaque<HideDataTwinRustAsyncMoi>> {
//     SyncReturn(RustOpaque::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_array_opaque_enum_twin_rust_async_moi() -> [EnumOpaqueTwinRustAsyncMoi; 5] {
    [
        EnumOpaqueTwinRustAsyncMoi::Struct(RustOpaque::new(HideDataTwinRustAsyncMoi(
            HideDataRaw::new(),
        ))),
        EnumOpaqueTwinRustAsyncMoi::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinRustAsyncMoi::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinRustAsyncMoi::Mutex(RustOpaque::new(Mutex::new(HideDataTwinRustAsyncMoi(
            HideDataRaw::new(),
        )))),
        EnumOpaqueTwinRustAsyncMoi::RwLock(RustOpaque::new(RwLock::new(HideDataTwinRustAsyncMoi(
            HideDataRaw::new(),
        )))),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_enum_opaque_twin_rust_async_moi(opaque: EnumOpaqueTwinRustAsyncMoi) -> String {
    match opaque {
        EnumOpaqueTwinRustAsyncMoi::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinRustAsyncMoi::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinRustAsyncMoi::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinRustAsyncMoi::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinRustAsyncMoi::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_opaque_twin_rust_async_moi(
    opaque: RustOpaque<HideDataTwinRustAsyncMoi>,
) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_opaque_with_delay_twin_rust_async_moi(
    opaque: RustOpaque<HideDataTwinRustAsyncMoi>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_array_twin_rust_async_moi() -> [RustOpaque<HideDataTwinRustAsyncMoi>; 2] {
    [
        RustOpaque::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn sync_create_non_clone_twin_rust_async_moi() -> SyncReturn<RustOpaque<NonCloneDataTwinRustAsyncMoi>> {
//     SyncReturn(RustOpaque::new(NonCloneDataTwinRustAsyncMoi::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_non_clone_twin_rust_async_moi(
    clone: RustOpaque<NonCloneDataTwinRustAsyncMoi>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_array_run_twin_rust_async_moi(data: [RustOpaque<HideDataTwinRustAsyncMoi>; 2]) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_vec_twin_rust_async_moi() -> Vec<RustOpaque<HideDataTwinRustAsyncMoi>> {
    vec![
        RustOpaque::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_vec_run_twin_rust_async_moi(data: Vec<RustOpaque<HideDataTwinRustAsyncMoi>>) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_nested_opaque_twin_rust_async_moi() -> OpaqueNestedTwinRustAsyncMoi {
    OpaqueNestedTwinRustAsyncMoi {
        first: RustOpaque::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
        second: RustOpaque::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_nested_opaque_twin_rust_async_moi(opaque: OpaqueNestedTwinRustAsyncMoi) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn unwrap_rust_opaque_twin_rust_async_moi(
    opaque: RustOpaque<HideDataTwinRustAsyncMoi>,
) -> Result<String> {
    let data: HideDataTwinRustAsyncMoi = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn frb_generator_test_twin_rust_async_moi() -> RustOpaque<FrbOpaqueReturnTwinRustAsyncMoi>
{
    panic!("dummy code");
}
