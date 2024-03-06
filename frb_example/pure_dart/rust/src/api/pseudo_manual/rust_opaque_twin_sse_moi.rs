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

pub struct HideDataTwinSseMoi(pub HideDataRaw);
pub struct NonCloneDataTwinSseMoi(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinSseMoi;

/// Opaque types
pub trait DartDebugTwinSseMoi: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinSseMoi for T {}

pub enum EnumOpaqueTwinSseMoi {
    Struct(crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>),
    Primitive(crate::frb_generated::RustOpaqueMoi<i16>),
    TraitObj(crate::frb_generated::RustOpaqueMoi<Box<dyn DartDebugTwinSseMoi>>),
    Mutex(crate::frb_generated::RustOpaqueMoi<Mutex<HideDataTwinSseMoi>>),
    RwLock(crate::frb_generated::RustOpaqueMoi<RwLock<HideDataTwinSseMoi>>),
    Nothing,
}

/// [`HideDataTwinSseMoi`] has private fields.
pub struct OpaqueNestedTwinSseMoi {
    pub first: crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn create_opaque_twin_sse_moi() -> crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi> {
    crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSseMoi(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn create_option_opaque_twin_sse_moi(
    opaque: Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>>,
) -> Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_opaque_twin_sse_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSseMoi(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn create_array_opaque_enum_twin_sse_moi() -> [EnumOpaqueTwinSseMoi; 5] {
    [
        EnumOpaqueTwinSseMoi::Struct(crate::frb_generated::RustOpaqueMoi::new(
            HideDataTwinSseMoi(HideDataRaw::new()),
        )),
        EnumOpaqueTwinSseMoi::Primitive(crate::frb_generated::RustOpaqueMoi::new(42)),
        EnumOpaqueTwinSseMoi::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSseMoi::Mutex(crate::frb_generated::RustOpaqueMoi::new(Mutex::new(
            HideDataTwinSseMoi(HideDataRaw::new()),
        ))),
        EnumOpaqueTwinSseMoi::RwLock(crate::frb_generated::RustOpaqueMoi::new(RwLock::new(
            HideDataTwinSseMoi(HideDataRaw::new()),
        ))),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn run_enum_opaque_twin_sse_moi(opaque: EnumOpaqueTwinSseMoi) -> String {
    match opaque {
        EnumOpaqueTwinSseMoi::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinSseMoi::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSseMoi::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSseMoi::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinSseMoi::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
        _ => "nothing".to_owned(),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn run_opaque_twin_sse_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>,
) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn run_opaque_with_delay_twin_sse_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_array_twin_sse_moi() -> [crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>; 2] {
    [
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSseMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSseMoi(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_non_clone_twin_sse_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinSseMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(NonCloneDataTwinSseMoi::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn run_non_clone_twin_sse_moi(
    clone: crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinSseMoi>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_array_run_twin_sse_moi(
    data: [crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>; 2],
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_vec_twin_sse_moi() -> Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>> {
    vec![
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSseMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSseMoi(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_vec_run_twin_sse_moi(
    data: Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>>,
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn create_nested_opaque_twin_sse_moi() -> OpaqueNestedTwinSseMoi {
    OpaqueNestedTwinSseMoi {
        first: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSseMoi(HideDataRaw::new())),
        second: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSseMoi(HideDataRaw::new())),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn run_nested_opaque_twin_sse_moi(opaque: OpaqueNestedTwinSseMoi) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn unwrap_rust_opaque_twin_sse_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSseMoi>,
) -> Result<String> {
    let data: HideDataTwinSseMoi = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub fn frb_generator_test_twin_sse_moi(
) -> crate::frb_generated::RustOpaqueMoi<FrbOpaqueReturnTwinSseMoi> {
    panic!("dummy code");
}
