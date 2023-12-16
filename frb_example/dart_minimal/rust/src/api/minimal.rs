use flutter_rust_bridge::DartFnFuture;
pub use std::panic::{RefUnwindSafe, UnwindSafe};

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO temp
pub async fn hi_1(callback: impl Fn() -> DartFnFuture<()> + UnwindSafe) {
    println!("rust_call_dart_simple before");
    callback().await;
    println!("rust_call_dart_simple after");
}
pub fn hi_2(opaque: flutter_rust_bridge::DartOpaque) -> flutter_rust_bridge::DartOpaque {
    opaque
}
