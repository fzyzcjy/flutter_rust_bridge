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

pub struct HideDataTwinRustAsyncSse(HideDataRaw);
pub struct NonCloneDataTwinRustAsyncSse(NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinRustAsyncSse;

/// Opaque types
pub trait DartDebugTwinRustAsyncSse: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinRustAsyncSse for T {}

pub enum EnumOpaqueTwinRustAsyncSse {
    Struct(crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>),
    Primitive(crate::frb_generated::RustOpaqueMoi<i16>),
    TraitObj(crate::frb_generated::RustOpaqueMoi<Box<dyn DartDebugTwinRustAsyncSse>>),
    Mutex(crate::frb_generated::RustOpaqueMoi<Mutex<HideDataTwinRustAsyncSse>>),
    RwLock(crate::frb_generated::RustOpaqueMoi<RwLock<HideDataTwinRustAsyncSse>>),
}

/// [`HideDataTwinRustAsyncSse`] has private fields.
pub struct OpaqueNestedTwinRustAsyncSse {
    pub first: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>,
    pub second: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn create_opaque_twin_rust_async_sse(
) -> crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse> {
    crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSse(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn create_option_opaque_twin_rust_async_sse(
    opaque: Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>>,
) -> Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub async fn sync_create_opaque_twin_rust_async_sse() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSse(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(serialize)]
pub async fn create_array_opaque_enum_twin_rust_async_sse() -> [EnumOpaqueTwinRustAsyncSse; 5] {
    [
        EnumOpaqueTwinRustAsyncSse::Struct(crate::frb_generated::RustOpaqueMoi::new(
            HideDataTwinRustAsyncSse(HideDataRaw::new()),
        )),
        EnumOpaqueTwinRustAsyncSse::Primitive(crate::frb_generated::RustOpaqueMoi::new(42)),
        EnumOpaqueTwinRustAsyncSse::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinRustAsyncSse::Mutex(crate::frb_generated::RustOpaqueMoi::new(Mutex::new(
            HideDataTwinRustAsyncSse(HideDataRaw::new()),
        ))),
        EnumOpaqueTwinRustAsyncSse::RwLock(crate::frb_generated::RustOpaqueMoi::new(RwLock::new(
            HideDataTwinRustAsyncSse(HideDataRaw::new()),
        ))),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn run_enum_opaque_twin_rust_async_sse(opaque: EnumOpaqueTwinRustAsyncSse) -> String {
    match opaque {
        EnumOpaqueTwinRustAsyncSse::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinRustAsyncSse::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinRustAsyncSse::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinRustAsyncSse::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinRustAsyncSse::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn run_opaque_twin_rust_async_sse(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>,
) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn run_opaque_with_delay_twin_rust_async_sse(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_array_twin_rust_async_sse(
) -> [crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>; 2] {
    [
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub async fn sync_create_non_clone_twin_rust_async_sse() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinRustAsyncSse>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(NonCloneDataTwinRustAsyncSse::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn run_non_clone_twin_rust_async_sse(
    clone: crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinRustAsyncSse>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_array_run_twin_rust_async_sse(
    data: [crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>; 2],
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_vec_twin_rust_async_sse(
) -> Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>> {
    vec![
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_vec_run_twin_rust_async_sse(
    data: Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>>,
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn create_nested_opaque_twin_rust_async_sse() -> OpaqueNestedTwinRustAsyncSse {
    OpaqueNestedTwinRustAsyncSse {
        first: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSse(
            HideDataRaw::new(),
        )),
        second: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncSse(
            HideDataRaw::new(),
        )),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn run_nested_opaque_twin_rust_async_sse(opaque: OpaqueNestedTwinRustAsyncSse) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn unwrap_rust_opaque_twin_rust_async_sse(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncSse>,
) -> Result<String> {
    let data: HideDataTwinRustAsyncSse = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(serialize)]
pub async fn frb_generator_test_twin_rust_async_sse(
) -> crate::frb_generated::RustOpaqueMoi<FrbOpaqueReturnTwinRustAsyncSse> {
    panic!("dummy code");
}
