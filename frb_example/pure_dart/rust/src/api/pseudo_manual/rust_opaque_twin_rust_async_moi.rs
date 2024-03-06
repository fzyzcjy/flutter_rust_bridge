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

pub struct HideDataTwinRustAsyncMoi(pub HideDataRaw);
pub struct NonCloneDataTwinRustAsyncMoi(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinRustAsyncMoi;

/// Opaque types
pub trait DartDebugTwinRustAsyncMoi: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinRustAsyncMoi for T {}

pub enum EnumOpaqueTwinRustAsyncMoi {
    Struct(crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>),
    Primitive(crate::frb_generated::RustOpaqueMoi<i16>),
    TraitObj(crate::frb_generated::RustOpaqueMoi<Box<dyn DartDebugTwinRustAsyncMoi>>),
    Mutex(crate::frb_generated::RustOpaqueMoi<Mutex<HideDataTwinRustAsyncMoi>>),
    RwLock(crate::frb_generated::RustOpaqueMoi<RwLock<HideDataTwinRustAsyncMoi>>),
    Nothing,
}

/// [`HideDataTwinRustAsyncMoi`] has private fields.
pub struct OpaqueNestedTwinRustAsyncMoi {
    pub first: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_opaque_twin_rust_async_moi(
) -> crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi> {
    crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncMoi(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_option_opaque_twin_rust_async_moi(
    opaque: Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>>,
) -> Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn sync_create_opaque_twin_rust_async_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_array_opaque_enum_twin_rust_async_moi() -> [EnumOpaqueTwinRustAsyncMoi; 5] {
    [
        EnumOpaqueTwinRustAsyncMoi::Struct(crate::frb_generated::RustOpaqueMoi::new(
            HideDataTwinRustAsyncMoi(HideDataRaw::new()),
        )),
        EnumOpaqueTwinRustAsyncMoi::Primitive(crate::frb_generated::RustOpaqueMoi::new(42)),
        EnumOpaqueTwinRustAsyncMoi::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinRustAsyncMoi::Mutex(crate::frb_generated::RustOpaqueMoi::new(Mutex::new(
            HideDataTwinRustAsyncMoi(HideDataRaw::new()),
        ))),
        EnumOpaqueTwinRustAsyncMoi::RwLock(crate::frb_generated::RustOpaqueMoi::new(RwLock::new(
            HideDataTwinRustAsyncMoi(HideDataRaw::new()),
        ))),
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
        _ => "nothing".to_owned(),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_opaque_twin_rust_async_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>,
) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_opaque_with_delay_twin_rust_async_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_array_twin_rust_async_moi(
) -> [crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>; 2] {
    [
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn sync_create_non_clone_twin_rust_async_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinRustAsyncMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(NonCloneDataTwinRustAsyncMoi::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_non_clone_twin_rust_async_moi(
    clone: crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinRustAsyncMoi>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_array_run_twin_rust_async_moi(
    data: [crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>; 2],
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_vec_twin_rust_async_moi(
) -> Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>> {
    vec![
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncMoi(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn opaque_vec_run_twin_rust_async_moi(
    data: Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>>,
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn create_nested_opaque_twin_rust_async_moi() -> OpaqueNestedTwinRustAsyncMoi {
    OpaqueNestedTwinRustAsyncMoi {
        first: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncMoi(
            HideDataRaw::new(),
        )),
        second: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinRustAsyncMoi(
            HideDataRaw::new(),
        )),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn run_nested_opaque_twin_rust_async_moi(opaque: OpaqueNestedTwinRustAsyncMoi) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub async fn unwrap_rust_opaque_twin_rust_async_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinRustAsyncMoi>,
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
pub async fn frb_generator_test_twin_rust_async_moi(
) -> crate::frb_generated::RustOpaqueMoi<FrbOpaqueReturnTwinRustAsyncMoi> {
    panic!("dummy code");
}
