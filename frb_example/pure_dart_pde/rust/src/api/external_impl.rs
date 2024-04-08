// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

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
    pub fn simple_external_method(&self) -> String {}
}
