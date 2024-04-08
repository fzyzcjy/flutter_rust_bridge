// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `external_impl.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;
pub use frb_example_pure_dart_example_external_lib::{
    SimpleOpaqueExternalStructWithMethod, SimpleTranslatableExternalStructWithMethod,
};

#[frb(mirror(SimpleTranslatableExternalStructWithMethod))]
pub struct _SimpleTranslatableExternalStructWithMethod {
    pub a: String,
}

#[frb(external)]
impl SimpleTranslatableExternalStructWithMethod {
    #[flutter_rust_bridge::frb(sync)]
    pub fn simple_external_method_twin_sync(&self) -> String {}
}

#[frb(external)]
impl SimpleOpaqueExternalStructWithMethod {
    #[flutter_rust_bridge::frb(sync)]
    pub fn simple_external_method_twin_sync(&self) -> String {}
}
