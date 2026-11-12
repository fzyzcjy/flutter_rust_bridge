use flutter_rust_bridge::{frb, DartFnFuture};

#[frb(init)]
pub fn init_app() {
    // This example also shares a runtime with pure_dart, which installs its own logger.
    flutter_rust_bridge::setup_backtrace();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(sync)]
pub fn minimal_sync_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn minimal_callback(callback: impl Fn() -> DartFnFuture<i32>) -> i32 {
    callback().await
}
