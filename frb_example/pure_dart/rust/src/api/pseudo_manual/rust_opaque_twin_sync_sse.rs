// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

pub use crate::auxiliary::sample_types::{HideDataRaw, NonCloneDataRaw, NonSendHideDataRaw};
use anyhow::Result;
use flutter_rust_bridge::{opaque_dyn, RustOpaque};
use std::fmt::Debug;
use std::ops::Deref;
pub use std::sync::{Mutex, RwLock};

pub struct HideDataTwinSyncSse(HideDataRaw);
pub struct NonCloneDataTwinSyncSse(NonCloneDataRaw);
pub struct NonSendHideDataTwinSyncSse(NonSendHideDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinSyncSse;

/// Opaque types
pub trait DartDebugTwinSyncSse: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinSyncSse for T {}

pub enum EnumOpaqueTwinSyncSse {
    Struct(crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>),
    Primitive(crate::frb_generated::RustOpaqueMoi<i32>),
    TraitObj(crate::frb_generated::RustOpaqueMoi<Box<dyn DartDebugTwinSyncSse>>),
    Mutex(crate::frb_generated::RustOpaqueMoi<Mutex<HideDataTwinSyncSse>>),
    RwLock(crate::frb_generated::RustOpaqueMoi<RwLock<HideDataTwinSyncSse>>),
}

/// [`HideDataTwinSyncSse`] has private fields.
pub struct OpaqueNestedTwinSyncSse {
    pub first: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>,
    pub second: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_opaque_twin_sync_sse() -> crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse> {
    RustOpaque::new(HideDataTwinSyncSse::new())
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_option_opaque_twin_sync_sse(
    opaque: Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>>,
) -> Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_opaque_twin_sync_sse() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>> {
//     SyncReturn(RustOpaque::new(HideDataTwinSyncSse::new()))
// }

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_array_opaque_enum_twin_sync_sse() -> [EnumOpaqueTwinSyncSse; 5] {
    [
        EnumOpaqueTwinSyncSse::Struct(RustOpaque::new(HideDataTwinSyncSse::new())),
        EnumOpaqueTwinSyncSse::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinSyncSse::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSyncSse::Mutex(RustOpaque::new(Mutex::new(HideDataTwinSyncSse::new()))),
        EnumOpaqueTwinSyncSse::RwLock(RustOpaque::new(RwLock::new(HideDataTwinSyncSse::new()))),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_enum_opaque_twin_sync_sse(opaque: EnumOpaqueTwinSyncSse) -> String {
    match opaque {
        EnumOpaqueTwinSyncSse::Struct(s) => s.hide_data(),
        EnumOpaqueTwinSyncSse::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSyncSse::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSyncSse::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaqueTwinSyncSse::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_twin_sync_sse(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>,
) -> String {
    opaque.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_with_delay_twin_sync_sse(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_twin_sync_sse() -> [crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>; 2]
{
    [
        RustOpaque::new(HideDataTwinSyncSse::new()),
        RustOpaque::new(HideDataTwinSyncSse::new()),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_non_clone_twin_sync_sse() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinSyncSse>> {
//     SyncReturn(RustOpaque::new(NonCloneDataTwinSyncSse::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_non_clone_twin_sync_sse(
    clone: crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinSyncSse>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_sync_opaque_twin_sync_sse(
) -> crate::frb_generated::RustOpaqueMoi<NonSendHideDataTwinSyncSse> {
    RustOpaque::new(NonSendHideDataTwinSyncSse::new())
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_sync_opaque_twin_sync_sse() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonSendHideDataTwinSyncSse>> {
//     SyncReturn(RustOpaque::new(NonSendHideDataTwinSyncSse::new()))
// }

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_run_twin_sync_sse(
    data: [crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>; 2],
) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_twin_sync_sse() -> Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>> {
    vec![
        RustOpaque::new(HideDataTwinSyncSse::new()),
        RustOpaque::new(HideDataTwinSyncSse::new()),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_run_twin_sync_sse(
    data: Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>>,
) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_nested_opaque_twin_sync_sse() -> OpaqueNestedTwinSyncSse {
    OpaqueNestedTwinSyncSse {
        first: RustOpaque::new(HideDataTwinSyncSse::new()),
        second: RustOpaque::new(HideDataTwinSyncSse::new()),
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_nested_opaque_twin_sync_sse(opaque: OpaqueNestedTwinSyncSse) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn unwrap_rust_opaque_twin_sync_sse(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSyncSse>,
) -> Result<String> {
    let data: HideDataTwinSyncSse = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn frb_generator_test_twin_sync_sse(
) -> crate::frb_generated::RustOpaqueMoi<FrbOpaqueReturnTwinSyncSse> {
    panic!("dummy code");
}
