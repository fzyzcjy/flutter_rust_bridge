use flutter_rust_bridge::frb;
pub use std::any::Any;
pub use std::error::Error;

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[frb(opaque)]
pub struct DummyStruct;

pub fn f(a: DummyStruct) {
    let _ = a;
}
