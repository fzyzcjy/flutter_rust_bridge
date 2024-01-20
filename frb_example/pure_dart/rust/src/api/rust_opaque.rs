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

pub struct HideDataTwinNormal(pub HideDataRaw);
pub struct NonCloneDataTwinNormal(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinNormal;

/// Opaque types
pub trait DartDebugTwinNormal: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinNormal for T {}

pub enum EnumOpaqueTwinNormal {
    Struct(RustOpaque<HideDataTwinNormal>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinNormal>>),
    Mutex(RustOpaque<Mutex<HideDataTwinNormal>>),
    RwLock(RustOpaque<RwLock<HideDataTwinNormal>>),
    Nothing,
}

/// [`HideDataTwinNormal`] has private fields.
pub struct OpaqueNestedTwinNormal {
    pub first: RustOpaque<HideDataTwinNormal>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: RustOpaqueNom<HideDataTwinNormal>,
}

pub fn create_opaque_twin_normal() -> RustOpaque<HideDataTwinNormal> {
    RustOpaque::new(HideDataTwinNormal(HideDataRaw::new()))
}

pub fn create_option_opaque_twin_normal(
    opaque: Option<RustOpaque<HideDataTwinNormal>>,
) -> Option<RustOpaque<HideDataTwinNormal>> {
    opaque
}

// TODO about sync
// pub fn sync_create_opaque_twin_normal() -> SyncReturn<RustOpaque<HideDataTwinNormal>> {
//     SyncReturn(RustOpaque::new(HideDataTwinNormal(HideDataRaw::new())))
// }

pub fn create_array_opaque_enum_twin_normal() -> [EnumOpaqueTwinNormal; 5] {
    [
        EnumOpaqueTwinNormal::Struct(RustOpaque::new(HideDataTwinNormal(HideDataRaw::new()))),
        EnumOpaqueTwinNormal::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinNormal::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinNormal::Mutex(RustOpaque::new(Mutex::new(HideDataTwinNormal(
            HideDataRaw::new(),
        )))),
        EnumOpaqueTwinNormal::RwLock(RustOpaque::new(RwLock::new(HideDataTwinNormal(
            HideDataRaw::new(),
        )))),
    ]
}

pub fn run_enum_opaque_twin_normal(opaque: EnumOpaqueTwinNormal) -> String {
    match opaque {
        EnumOpaqueTwinNormal::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinNormal::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinNormal::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinNormal::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinNormal::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
        _ => "nothing".to_owned(),
    }
}

pub fn run_opaque_twin_normal(opaque: RustOpaque<HideDataTwinNormal>) -> String {
    opaque.0.hide_data()
}

pub fn run_opaque_with_delay_twin_normal(opaque: RustOpaque<HideDataTwinNormal>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

pub fn opaque_array_twin_normal() -> [RustOpaque<HideDataTwinNormal>; 2] {
    [
        RustOpaque::new(HideDataTwinNormal(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinNormal(HideDataRaw::new())),
    ]
}

// TODO about sync
// pub fn sync_create_non_clone_twin_normal() -> SyncReturn<RustOpaque<NonCloneDataTwinNormal>> {
//     SyncReturn(RustOpaque::new(NonCloneDataTwinNormal::new()))
// }

#[allow(clippy::redundant_clone)]
pub fn run_non_clone_twin_normal(clone: RustOpaque<NonCloneDataTwinNormal>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

pub fn opaque_array_run_twin_normal(data: [RustOpaque<HideDataTwinNormal>; 2]) {
    for i in data {
        i.0.hide_data();
    }
}

pub fn opaque_vec_twin_normal() -> Vec<RustOpaque<HideDataTwinNormal>> {
    vec![
        RustOpaque::new(HideDataTwinNormal(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinNormal(HideDataRaw::new())),
    ]
}

pub fn opaque_vec_run_twin_normal(data: Vec<RustOpaque<HideDataTwinNormal>>) {
    for i in data {
        i.0.hide_data();
    }
}

pub fn create_nested_opaque_twin_normal() -> OpaqueNestedTwinNormal {
    OpaqueNestedTwinNormal {
        first: RustOpaque::new(HideDataTwinNormal(HideDataRaw::new())),
        second: RustOpaque::new(HideDataTwinNormal(HideDataRaw::new())),
    }
}

pub fn run_nested_opaque_twin_normal(opaque: OpaqueNestedTwinNormal) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

pub fn unwrap_rust_opaque_twin_normal(opaque: RustOpaque<HideDataTwinNormal>) -> Result<String> {
    let data: HideDataTwinNormal = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub fn frb_generator_test_twin_normal() -> RustOpaque<FrbOpaqueReturnTwinNormal> {
    panic!("dummy code");
}
