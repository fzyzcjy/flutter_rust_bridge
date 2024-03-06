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

pub struct HideDataTwinRustAsync(pub HideDataRaw);
pub struct NonCloneDataTwinRustAsync(pub NonCloneDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinRustAsync;

/// Opaque types
pub trait DartDebugTwinRustAsync: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinRustAsync for T {}

pub enum EnumOpaqueTwinRustAsync {
    Struct(RustOpaque<HideDataTwinRustAsync>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinRustAsync>>),
    Mutex(RustOpaque<Mutex<HideDataTwinRustAsync>>),
    RwLock(RustOpaque<RwLock<HideDataTwinRustAsync>>),
    Nothing,
}

/// [`HideDataTwinRustAsync`] has private fields.
pub struct OpaqueNestedTwinRustAsync {
    pub first: RustOpaque<HideDataTwinRustAsync>,
    // Randomly use postfix here once, in order to test they are equivalent (just type alias)
    pub second: RustOpaqueNom<HideDataTwinRustAsync>,
}

pub async fn create_opaque_twin_rust_async() -> RustOpaque<HideDataTwinRustAsync> {
    RustOpaque::new(HideDataTwinRustAsync(HideDataRaw::new()))
}

pub async fn create_option_opaque_twin_rust_async(
    opaque: Option<RustOpaque<HideDataTwinRustAsync>>,
) -> Option<RustOpaque<HideDataTwinRustAsync>> {
    opaque
}

// TODO about sync
// pub async fn sync_create_opaque_twin_rust_async() -> SyncReturn<RustOpaque<HideDataTwinRustAsync>> {
//     SyncReturn(RustOpaque::new(HideDataTwinRustAsync(HideDataRaw::new())))
// }

pub async fn create_array_opaque_enum_twin_rust_async() -> [EnumOpaqueTwinRustAsync; 5] {
    [
        EnumOpaqueTwinRustAsync::Struct(RustOpaque::new(HideDataTwinRustAsync(HideDataRaw::new()))),
        EnumOpaqueTwinRustAsync::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinRustAsync::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinRustAsync::Mutex(RustOpaque::new(Mutex::new(HideDataTwinRustAsync(
            HideDataRaw::new(),
        )))),
        EnumOpaqueTwinRustAsync::RwLock(RustOpaque::new(RwLock::new(HideDataTwinRustAsync(
            HideDataRaw::new(),
        )))),
    ]
}

pub async fn run_enum_opaque_twin_rust_async(opaque: EnumOpaqueTwinRustAsync) -> String {
    match opaque {
        EnumOpaqueTwinRustAsync::Struct(s) => s.0.hide_data(),
        EnumOpaqueTwinRustAsync::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinRustAsync::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinRustAsync::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().0.hide_data())
        }
        EnumOpaqueTwinRustAsync::RwLock(r) => {
            format!("{:?}", r.read().unwrap().0.hide_data())
        }
        _ => "nothing".to_owned(),
    }
}

pub async fn run_opaque_twin_rust_async(opaque: RustOpaque<HideDataTwinRustAsync>) -> String {
    opaque.0.hide_data()
}

pub async fn run_opaque_with_delay_twin_rust_async(
    opaque: RustOpaque<HideDataTwinRustAsync>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.0.hide_data()
}

pub async fn opaque_array_twin_rust_async() -> [RustOpaque<HideDataTwinRustAsync>; 2] {
    [
        RustOpaque::new(HideDataTwinRustAsync(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinRustAsync(HideDataRaw::new())),
    ]
}

// TODO about sync
// pub async fn sync_create_non_clone_twin_rust_async() -> SyncReturn<RustOpaque<NonCloneDataTwinRustAsync>> {
//     SyncReturn(RustOpaque::new(NonCloneDataTwinRustAsync::new()))
// }

#[allow(clippy::redundant_clone)]
pub async fn run_non_clone_twin_rust_async(clone: RustOpaque<NonCloneDataTwinRustAsync>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().0.hide_data()
}

pub async fn opaque_array_run_twin_rust_async(data: [RustOpaque<HideDataTwinRustAsync>; 2]) {
    for i in data {
        i.0.hide_data();
    }
}

pub async fn opaque_vec_twin_rust_async() -> Vec<RustOpaque<HideDataTwinRustAsync>> {
    vec![
        RustOpaque::new(HideDataTwinRustAsync(HideDataRaw::new())),
        RustOpaque::new(HideDataTwinRustAsync(HideDataRaw::new())),
    ]
}

pub async fn opaque_vec_run_twin_rust_async(data: Vec<RustOpaque<HideDataTwinRustAsync>>) {
    for i in data {
        i.0.hide_data();
    }
}

pub async fn create_nested_opaque_twin_rust_async() -> OpaqueNestedTwinRustAsync {
    OpaqueNestedTwinRustAsync {
        first: RustOpaque::new(HideDataTwinRustAsync(HideDataRaw::new())),
        second: RustOpaque::new(HideDataTwinRustAsync(HideDataRaw::new())),
    }
}

pub async fn run_nested_opaque_twin_rust_async(opaque: OpaqueNestedTwinRustAsync) {
    opaque.first.0.hide_data();
    opaque.second.0.hide_data();
}

pub async fn unwrap_rust_opaque_twin_rust_async(
    opaque: RustOpaque<HideDataTwinRustAsync>,
) -> Result<String> {
    let data: HideDataTwinRustAsync = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.0.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub async fn frb_generator_test_twin_rust_async() -> RustOpaque<FrbOpaqueReturnTwinRustAsync> {
    panic!("dummy code");
}
