use flutter_rust_bridge::{frb, DartFnFuture, DartOpaque};
pub use std::panic::{RefUnwindSafe, UnwindSafe};

#[frb(serialize)]
pub async fn minimal_adder(a: i32, b: i32) -> i32 {
    panic!("hello this is deliberate panic")
}

pub async fn rust_call_dart_simple(callback: impl Fn() -> DartFnFuture<()> + UnwindSafe) {
    println!("rust_call_dart_simple before");
    callback().await;
    println!("rust_call_dart_simple after");
}
