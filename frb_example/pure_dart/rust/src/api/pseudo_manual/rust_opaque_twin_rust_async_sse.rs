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

pub struct HideDataTwinRustAsyncSse(pub HideDataRaw);
pub struct NonCloneDataTwinRustAsyncSse(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinRustAsyncSse;

/// Opaque types
pub trait DartDebugTwinRustAsyncSse: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinRustAsyncSse for T {}

pub enum EnumOpaqueTwinRustAsyncSse {
    Struct(RustOpaque<HideDataTwinRustAsyncSse>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinRustAsyncSse>>),
    Mutex(RustOpaque<Mutex<HideDataTwinRustAsyncSse>>),
    RwLock(RustOpaque<RwLock<HideDataTwinRustAsyncSse>>),
    Nothing,
}

/// [`HideDataTwinRustAsyncSse`] has private fields.
pub struct OpaqueNestedTwinRustAsyncSse {
    pub first: RustOpaque<HideDataTwinRustAsyncSse>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: RustOpaqueNom<HideDataTwinRustAsyncSse>,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn create_opaque_twin_rust_async_sse() -> RustOpaque<HideDataTwinRustAsyncSse> {
    RustOpaque::new(HideDataTwinRustAsyncSse(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn create_option_opaque_twin_rust_async_sse(
    opaque: Option<RustOpaque<HideDataTwinRustAsyncSse>>,
) -> Option<RustOpaque<HideDataTwinRustAsyncSse>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub async fn sync_create_opaque_twin_rust_async_sse() -> SyncReturn<RustOpaque<HideDataTwinRustAsyncSse>> {
//     SyncReturn(RustOpaque::new(HideDataTwinRustAsyncSse(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(serialize)]
pub async fn create_array_opaque_enum_twin_rust_async_sse() -> [EnumOpaqueTwinRustAsyncSse; 5] {
    [
        EnumOpaqueTwinRustAsyncSse::Struct(RustOpaque::new(HideDataTwinRustAsyncSse(
            HideDataRaw::new(),
        ))),
        EnumOpaqueTwinRustAsyncSse::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinRustAsyncSse::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinRustAsyncSse::Mutex(RustOpaque::new(Mutex::new(HideDataTwinRustAsyncSse(
            HideDataRaw::new(),
        )))),
        EnumOpaqueTwinRustAsyncSse::RwLock(RustOpaque::new(RwLock::new(HideDataTwinRustAsyncSse(
            HideDataRaw::new(),
        )))),
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
        _ => "nothing".to_owned(),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn run_opaque_twin_rust_async_sse(
    opaque: RustOpaque<HideDataTwinRustAsyncSse>,
) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn run_opaque_with_delay_twin_rust_async_sse(
    opaque: RustOpaque<HideDataTwinRustAsyncSse>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_array_twin_rust_async_sse() -> [RustOpaque<HideDataTwinRustAsyncSse>; 2] {
    [
        RustOpaque::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub async fn sync_create_non_clone_twin_rust_async_sse() -> SyncReturn<RustOpaque<NonCloneDataTwinRustAsyncSse>> {
//     SyncReturn(RustOpaque::new(NonCloneDataTwinRustAsyncSse::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn run_non_clone_twin_rust_async_sse(
    clone: RustOpaque<NonCloneDataTwinRustAsyncSse>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_array_run_twin_rust_async_sse(data: [RustOpaque<HideDataTwinRustAsyncSse>; 2]) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_vec_twin_rust_async_sse() -> Vec<RustOpaque<HideDataTwinRustAsyncSse>> {
    vec![
        RustOpaque::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn opaque_vec_run_twin_rust_async_sse(data: Vec<RustOpaque<HideDataTwinRustAsyncSse>>) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn create_nested_opaque_twin_rust_async_sse() -> OpaqueNestedTwinRustAsyncSse {
    OpaqueNestedTwinRustAsyncSse {
        first: RustOpaque::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
        second: RustOpaque::new(HideDataTwinRustAsyncSse(HideDataRaw::new())),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn run_nested_opaque_twin_rust_async_sse(opaque: OpaqueNestedTwinRustAsyncSse) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn unwrap_rust_opaque_twin_rust_async_sse(
    opaque: RustOpaque<HideDataTwinRustAsyncSse>,
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
pub async fn frb_generator_test_twin_rust_async_sse() -> RustOpaque<FrbOpaqueReturnTwinRustAsyncSse>
{
    panic!("dummy code");
}
