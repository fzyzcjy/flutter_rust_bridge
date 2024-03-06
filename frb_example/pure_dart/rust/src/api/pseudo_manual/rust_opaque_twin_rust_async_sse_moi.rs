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

pub struct HideDataTwinRustAsyncSseMoi(pub HideDataRaw);
pub struct NonCloneDataTwinRustAsyncSseMoi(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinRustAsyncSseMoi;

/// Opaque types
pub trait DartDebugTwinRustAsyncSseMoi: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinRustAsyncSseMoi for T {}

pub enum EnumOpaqueTwinRustAsyncSseMoi {
    Struct(crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>),
    Primitive(crate::frb_generated::RustOpaqueMoi<i16>),
    TraitObj(crate::frb_generated::RustOpaqueMoi<Box<dyn DartDebugTwinRustAsyncSseMoi>>),
    Mutex(crate::frb_generated::RustOpaqueMoi<Mutex<HideDataTwinRustAsyncSseMoi>>),
    RwLock(crate::frb_generated::RustOpaqueMoi<RwLock<HideDataTwinRustAsyncSseMoi>>),
    Nothing,
}

/// [`HideDataTwinRustAsyncSseMoi`] has private fields.
pub struct OpaqueNestedTwinRustAsyncSseMoi {
    pub first: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn create_opaque_twin_rust_async_sse_moi(
) -> crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi> {
    crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSseMoi(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn create_option_opaque_twin_rust_async_sse_moi(
    opaque: Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>>,
) -> Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn sync_create_opaque_twin_rust_async_sse_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSseMoi(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn create_array_opaque_enum_twin_rust_async_sse_moi() -> [EnumOpaqueTwinRustAsyncSseMoi; 5]
{
    [
        EnumOpaqueTwinRustAsyncSseMoi::Struct(crate::frb_generated::RustOpaqueMoi::new(
            HideDataTwinRustAsyncSseMoi(HideDataRaw::new()),
        )),
        EnumOpaqueTwinRustAsyncSseMoi::Primitive(crate::frb_generated::RustOpaqueMoi::new(42)),
        EnumOpaqueTwinRustAsyncSseMoi::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinRustAsyncSseMoi::Mutex(crate::frb_generated::RustOpaqueMoi::new(Mutex::new(
            HideDataTwinRustAsyncSseMoi(HideDataRaw::new()),
        ))),
        EnumOpaqueTwinRustAsyncSseMoi::RwLock(crate::frb_generated::RustOpaqueMoi::new(
            RwLock::new(HideDataTwinRustAsyncSseMoi(HideDataRaw::new())),
        )),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn run_enum_opaque_twin_rust_async_sse_moi(
    opaque: EnumOpaqueTwinRustAsyncSseMoi,
) -> String {
    match opaque {
        EnumOpaqueTwinRustAsyncSseMoi::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinRustAsyncSseMoi::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinRustAsyncSseMoi::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinRustAsyncSseMoi::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinRustAsyncSseMoi::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
        _ => "nothing".to_owned(),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn run_opaque_twin_rust_async_sse_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>,
) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn run_opaque_with_delay_twin_rust_async_sse_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_array_twin_rust_async_sse_moi(
) -> [crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>; 2] {
    [
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSseMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSseMoi(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn sync_create_non_clone_twin_rust_async_sse_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinRustAsyncSseMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(NonCloneDataTwinRustAsyncSseMoi::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn run_non_clone_twin_rust_async_sse_moi(
    clone: crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinRustAsyncSseMoi>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_array_run_twin_rust_async_sse_moi(
    data: [crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>; 2],
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_vec_twin_rust_async_sse_moi(
) -> Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>> {
    vec![
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSseMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSseMoi(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_vec_run_twin_rust_async_sse_moi(
    data: Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>>,
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn create_nested_opaque_twin_rust_async_sse_moi() -> OpaqueNestedTwinRustAsyncSseMoi {
    OpaqueNestedTwinRustAsyncSseMoi {
        first: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSseMoi(
            HideDataRaw::new(),
        )),
        second: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSseMoi(
            HideDataRaw::new(),
        )),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn run_nested_opaque_twin_rust_async_sse_moi(opaque: OpaqueNestedTwinRustAsyncSseMoi) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn unwrap_rust_opaque_twin_rust_async_sse_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSseMoi>,
) -> Result<String> {
    let data: HideDataTwinRustAsyncSseMoi = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn frb_generator_test_twin_rust_async_sse_moi(
) -> crate::frb_generated::RustOpaqueMoi<FrbOpaqueReturnTwinRustAsyncSseMoi> {
    panic!("dummy code");
}
