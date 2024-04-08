// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

#![allow(unused_variables)]

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
    pub fn simple_external_method(&self) -> String {}
}

#[frb(external)]
impl SimpleOpaqueExternalStructWithMethod {
    #[frb(sync)]
    pub fn new(a: String) -> Self {}

    pub fn simple_external_method(&self) -> String {}
}
