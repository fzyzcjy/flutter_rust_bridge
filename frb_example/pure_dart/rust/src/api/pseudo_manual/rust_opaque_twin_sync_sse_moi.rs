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

pub struct HideDataTwinSyncSseMoi(pub HideDataRaw);
pub struct NonCloneDataTwinSyncSseMoi(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinSyncSseMoi;

/// Opaque types
pub trait DartDebugTwinSyncSseMoi: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinSyncSseMoi for T {}

pub enum EnumOpaqueTwinSyncSseMoi {
    Struct(crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>),
    Primitive(crate::frb_generated::RustOpaqueMoi<i16>),
    TraitObj(crate::frb_generated::RustOpaqueMoi<Box<dyn DartDebugTwinSyncSseMoi>>),
    Mutex(crate::frb_generated::RustOpaqueMoi<Mutex<HideDataTwinSyncSseMoi>>),
    RwLock(crate::frb_generated::RustOpaqueMoi<RwLock<HideDataTwinSyncSseMoi>>),
    Nothing,
}

/// [`HideDataTwinSyncSseMoi`] has private fields.
pub struct OpaqueNestedTwinSyncSseMoi {
    pub first: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_opaque_twin_sync_sse_moi(
) -> crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi> {
    crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSyncSseMoi(HideDataRaw::new()))
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_option_opaque_twin_sync_sse_moi(
    opaque: Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>>,
) -> Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_opaque_twin_sync_sse_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSyncSseMoi(HideDataRaw::new())))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_array_opaque_enum_twin_sync_sse_moi() -> [EnumOpaqueTwinSyncSseMoi; 5] {
    [
        EnumOpaqueTwinSyncSseMoi::Struct(crate::frb_generated::RustOpaqueMoi::new(
            HideDataTwinSyncSseMoi(HideDataRaw::new()),
        )),
        EnumOpaqueTwinSyncSseMoi::Primitive(crate::frb_generated::RustOpaqueMoi::new(42)),
        EnumOpaqueTwinSyncSseMoi::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSyncSseMoi::Mutex(crate::frb_generated::RustOpaqueMoi::new(Mutex::new(
            HideDataTwinSyncSseMoi(HideDataRaw::new()),
        ))),
        EnumOpaqueTwinSyncSseMoi::RwLock(crate::frb_generated::RustOpaqueMoi::new(RwLock::new(
            HideDataTwinSyncSseMoi(HideDataRaw::new()),
        ))),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_enum_opaque_twin_sync_sse_moi(opaque: EnumOpaqueTwinSyncSseMoi) -> String {
    match opaque {
        EnumOpaqueTwinSyncSseMoi::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinSyncSseMoi::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSyncSseMoi::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSyncSseMoi::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinSyncSseMoi::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
        _ => "nothing".to_owned(),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_twin_sync_sse_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>,
) -> String {
    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_with_delay_twin_sync_sse_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_twin_sync_sse_moi(
) -> [crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>; 2] {
    [
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSyncSseMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSyncSseMoi(HideDataRaw::new())),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_non_clone_twin_sync_sse_moi() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinSyncSseMoi>> {
//     SyncReturn(crate::frb_generated::RustOpaqueMoi::new(NonCloneDataTwinSyncSseMoi::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_non_clone_twin_sync_sse_moi(
    clone: crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinSyncSseMoi>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_run_twin_sync_sse_moi(
    data: [crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>; 2],
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_twin_sync_sse_moi(
) -> Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>> {
    vec![
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSyncSseMoi(HideDataRaw::new())),
        crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSyncSseMoi(HideDataRaw::new())),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_run_twin_sync_sse_moi(
    data: Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>>,
) {
    for i in data {
        i.0.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_nested_opaque_twin_sync_sse_moi() -> OpaqueNestedTwinSyncSseMoi {
    OpaqueNestedTwinSyncSseMoi {
        first: crate::frb_generated::RustOpaqueMoi::new(HideDataTwinSyncSseMoi(HideDataRaw::new())),
        second: crate::frb_generated::RustOpaqueMoi::new(
            HideDataTwinSyncSseMoi(HideDataRaw::new()),
        ),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_nested_opaque_twin_sync_sse_moi(opaque: OpaqueNestedTwinSyncSseMoi) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn unwrap_rust_opaque_twin_sync_sse_moi(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSseMoi>,
) -> Result<String> {
    let data: HideDataTwinSyncSseMoi = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn frb_generator_test_twin_sync_sse_moi(
) -> crate::frb_generated::RustOpaqueMoi<FrbOpaqueReturnTwinSyncSseMoi> {
    panic!("dummy code");
}
