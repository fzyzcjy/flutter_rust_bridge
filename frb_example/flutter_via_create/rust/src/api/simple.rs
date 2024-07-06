use flutter_rust_bridge::DartFnFuture;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn rust_function(
    dart_callback: impl Fn(String) -> DartFnFuture<String>,
) {
    dart_callback("Tom".to_owned()).await; // Will get `Hello, Tom!`
}
