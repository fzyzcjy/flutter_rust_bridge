use flutter_rust_bridge::DartFnFuture;
use std::panic::UnwindSafe;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn async_func() {
    log::warn!("hi rust async_func executing");
}

pub async fn async_greet_with_callback(
    name: String,
    logger: impl Fn(String) -> DartFnFuture<()> + UnwindSafe,
) {
    log::warn!("hi rust async_greet_with_callback start");
    logger(format!("this is arg of rust->dart call")).await;
    log::warn!("hi rust async_greet_with_callback after await");
}
