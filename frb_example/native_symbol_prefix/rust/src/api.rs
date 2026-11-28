use flutter_rust_bridge::{frb, DartFnFuture};

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(sync)]
pub fn add_sync(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn call_dart(callback: impl Fn() -> DartFnFuture<i32>) -> i32 {
    callback().await
}
