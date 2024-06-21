#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    greet_inner(name);
}

#[no_mangle]
#[inline(never)]
pub fn greet_inner(name: String) -> String {
    let a = name.len();
    let b = a * a + a;
    format!("Hello, {name}! a={a} b={b}")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
