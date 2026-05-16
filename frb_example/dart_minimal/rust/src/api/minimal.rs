use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    // Keep stacktraces, but do not install a default logger, since that would
    // prevent users from enabling the FRB Rust-to-Dart logging bridge.
    flutter_rust_bridge::setup_backtrace();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}
