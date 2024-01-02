pub use crate::auxiliary::sample_types::{
    FrbOpaqueReturn, HideData, NonCloneData, NonSendHideData,
};
use anyhow::Result;
use flutter_rust_bridge::{opaque_dyn, RustOpaque};
use std::fmt::Debug;
use std::ops::Deref;
pub use std::sync::{Mutex, RwLock};

/// Opaque types
pub trait DartDebugTwinNormal: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinNormal for T {}

pub enum EnumOpaqueTwinNormal {
    Struct(RustOpaque<HideData>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinNormal>>),
    Mutex(RustOpaque<Mutex<HideData>>),
    RwLock(RustOpaque<RwLock<HideData>>),
}

/// [`HideData`] has private fields.
pub struct OpaqueNestedTwinNormal {
    pub first: RustOpaque<HideData>,
    pub second: RustOpaque<HideData>,
}

pub fn create_opaque_twin_normal() -> RustOpaque<HideData> {
    RustOpaque::new(HideData::new())
}

pub fn create_option_opaque_twin_normal(
    opaque: Option<RustOpaque<HideData>>,
) -> Option<RustOpaque<HideData>> {
    opaque
}

// TODO about sync
// pub fn sync_create_opaque_twin_normal() -> SyncReturn<RustOpaque<HideData>> {
//     SyncReturn(RustOpaque::new(HideData::new()))
// }

pub fn create_array_opaque_enum_twin_normal() -> [EnumOpaqueTwinNormal; 5] {
    [
        EnumOpaqueTwinNormal::Struct(RustOpaque::new(HideData::new())),
        EnumOpaqueTwinNormal::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinNormal::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinNormal::Mutex(RustOpaque::new(Mutex::new(HideData::new()))),
        EnumOpaqueTwinNormal::RwLock(RustOpaque::new(RwLock::new(HideData::new()))),
    ]
}

pub fn run_enum_opaque_twin_normal(opaque: EnumOpaqueTwinNormal) -> String {
    match opaque {
        EnumOpaqueTwinNormal::Struct(s) => s.hide_data(),
        EnumOpaqueTwinNormal::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinNormal::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinNormal::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaqueTwinNormal::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

pub fn run_opaque_twin_normal(opaque: RustOpaque<HideData>) -> String {
    opaque.hide_data()
}

pub fn run_opaque_with_delay_twin_normal(opaque: RustOpaque<HideData>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.hide_data()
}

pub fn opaque_array_twin_normal() -> [RustOpaque<HideData>; 2] {
    [
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

// TODO about sync
// pub fn sync_create_non_clone_twin_normal() -> SyncReturn<RustOpaque<NonCloneData>> {
//     SyncReturn(RustOpaque::new(NonCloneData::new()))
// }

#[allow(clippy::redundant_clone)]
pub fn run_non_clone_twin_normal(clone: RustOpaque<NonCloneData>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

pub fn create_sync_opaque_twin_normal() -> RustOpaque<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

// TODO about sync
// pub fn sync_create_sync_opaque_twin_normal() -> SyncReturn<RustOpaque<NonSendHideData>> {
//     SyncReturn(RustOpaque::new(NonSendHideData::new()))
// }

pub fn opaque_array_run_twin_normal(data: [RustOpaque<HideData>; 2]) {
    for i in data {
        i.hide_data();
    }
}

pub fn opaque_vec_twin_normal() -> Vec<RustOpaque<HideData>> {
    vec![
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

pub fn opaque_vec_run_twin_normal(data: Vec<RustOpaque<HideData>>) {
    for i in data {
        i.hide_data();
    }
}

pub fn create_nested_opaque_twin_normal() -> OpaqueNestedTwinNormal {
    OpaqueNestedTwinNormal {
        first: RustOpaque::new(HideData::new()),
        second: RustOpaque::new(HideData::new()),
    }
}

pub fn run_nested_opaque_twin_normal(opaque: OpaqueNestedTwinNormal) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

pub fn unwrap_rust_opaque_twin_normal(opaque: RustOpaque<HideData>) -> Result<String> {
    let data: HideData = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub fn frb_generator_test_twin_normal() -> RustOpaque<FrbOpaqueReturn> {
    panic!("dummy code");
}
