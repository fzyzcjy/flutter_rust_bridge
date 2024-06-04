pub use std::any::Any;
pub use std::error::Error;
use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[frb(opaque)]
pub struct DummyStruct;

pub fn f(a: DummyStruct) { let _ = a; }
