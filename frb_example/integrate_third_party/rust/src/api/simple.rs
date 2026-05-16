use flutter_rust_bridge::frb;
pub use std::any::Any;
pub use std::error::Error;

#[frb(init)]
pub fn init_app() {
    // Keep stacktraces, but do not install a default logger, since that would
    // prevent users from enabling the FRB Rust-to-Dart logging bridge.
    flutter_rust_bridge::setup_backtrace();
}

#[frb(opaque)]
pub struct DummyStruct;

pub fn f(a: DummyStruct) {
    let _ = a;
}
