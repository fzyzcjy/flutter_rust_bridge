pub use frb_example_pure_dart_example_external_lib::{
    SimpleOpaqueExternalStructWithMethod, SimpleTranslatableExternalStructWithMethod,
};

use flutter_rust_bridge::frb;

#[frb(mirror(SimpleTranslatableExternalStructWithMethod))]
pub struct _SimpleTranslatableExternalStructWithMethod {
    pub a: String,
}

#[frb(stub)]
impl SimpleTranslatableExternalStructWithMethod {
    pub fn simple_external_method(&self) -> String {}
}

#[frb(stub)]
impl SimpleOpaqueExternalStructWithMethod {
    pub fn simple_external_method(&self) -> String {}
}
