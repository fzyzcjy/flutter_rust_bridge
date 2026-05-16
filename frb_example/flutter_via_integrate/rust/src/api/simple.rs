#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Keep stacktraces, but do not install a default logger, since that would
    // prevent users from enabling the FRB Rust-to-Dart logging bridge.
    flutter_rust_bridge::setup_backtrace();
}
