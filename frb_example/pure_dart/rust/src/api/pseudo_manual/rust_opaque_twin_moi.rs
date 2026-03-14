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

pub struct HideDataTwinMoi(pub HideDataRaw);
pub struct NonCloneDataTwinMoi(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinMoi;

/// Opaque types
pub trait DartDebugTwinMoi: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinMoi for T {}

pub enum EnumOpaqueTwinMoi {
    Struct(crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>),
    Primitive(crate::frb_generated::RustOpaqueMoi<i16>),
    TraitObj(crate::frb_generated::RustOpaqueMoi<Box<dyn DartDebugTwinMoi>>),
    Mutex(crate::frb_generated::RustOpaqueMoi<Mutex<HideDataTwinMoi>>),
    RwLock(crate::frb_generated::RustOpaqueMoi<RwLock<HideDataTwinMoi>>),
    Nothing,
}

/// [`HideDataTwinMoi`] has private fields.
pub struct OpaqueNestedTwinMoi {
    pub first: crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn create_opaque_twin_moi() -> crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi> {
    crate::frb_generated::RustOpaqueMoi::new(HideDataTwinMoi(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn create_option_opaque_twin_moi(
    opaque: Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>>,
) -> Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub fn sync_create_opaque_twin_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(HideDataTwinMoi(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn create_array_opaque_enum_twin_moi() -> [EnumOpaqueTwinMoi; 5] {
    [
        EnumOpaqueTwinMoi::Struct(crate::frb_generated::RustOpaqueMoi::new(HideDataTwinMoi(
            HideDataRaw::new(),
        ))),
        EnumOpaqueTwinMoi::Primitive(crate::frb_generated::RustOpaqueMoi::new(42)),
        EnumOpaqueTwinMoi::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinMoi::Mutex(crate::frb_generated::RustOpaqueMoi::new(Mutex::new(
            HideDataTwinMoi(HideDataRaw::new()),
        ))),
        EnumOpaqueTwinMoi::RwLock(crate::frb_generated::RustOpaqueMoi::new(RwLock::new(
            HideDataTwinMoi(HideDataRaw::new()),
        ))),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn run_enum_opaque_twin_moi(opaque: EnumOpaqueTwinMoi) -> String {
    match opaque {
        EnumOpaqueTwinMoi::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinMoi::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinMoi::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinMoi::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinMoi::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
        _ => "nothing".to_owned(),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn run_opaque_twin_moi(opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn run_opaque_with_delay_twin_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn opaque_array_twin_moi() -> [crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>; 2] {
    [
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinMoi(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub fn sync_create_non_clone_twin_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(NonCloneDataTwinMoi::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn run_non_clone_twin_moi(
    clone: crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinMoi>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn opaque_array_run_twin_moi(data: [crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>; 2]) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn opaque_vec_twin_moi() -> Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>> {
    vec![
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinMoi(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn opaque_vec_run_twin_moi(data: Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>>) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn create_nested_opaque_twin_moi() -> OpaqueNestedTwinMoi {
    OpaqueNestedTwinMoi {
        first: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinMoi(HideDataRaw::new())),
        second: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinMoi(HideDataRaw::new())),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn run_nested_opaque_twin_moi(opaque: OpaqueNestedTwinMoi) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn unwrap_rust_opaque_twin_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinMoi>,
) -> Result<String> {
    let data: HideDataTwinMoi = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn frb_generator_test_twin_moi() -> crate::frb_generated::RustOpaqueMoi<FrbOpaqueReturnTwinMoi>
{
    panic!("dummy code");
}
