// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "syncSse", "rustAsyncSse"]}

use flutter_rust_bridge::{DartFnFuture, DartOpaque};
use std::future::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;

pub struct DemoStructForRustCallDart {
    pub name: String,
}

pub async fn rust_call_dart_simple(callback: impl Fn() -> DartFnFuture<()> + UnwindSafe) {
    println!("rust_call_dart_simple before");
    callback().await;
    println!("rust_call_dart_simple after");
}

pub async fn rust_call_dart_one_arg(callback: impl Fn(String) -> DartFnFuture<()> + UnwindSafe) {
    callback("a".to_owned()).await;
}

pub async fn rust_call_dart_two_args(
    callback: impl Fn(String, DemoStructForRustCallDart) -> DartFnFuture<()> + UnwindSafe,
) {
    callback(
        "a".to_owned(),
        DemoStructForRustCallDart {
            name: "b".to_owned(),
        },
    )
    .await;
}

pub async fn rust_call_dart_return(callback: impl Fn() -> DartFnFuture<String> + UnwindSafe) {
    let result = callback().await;
    assert_eq!(&result, "a");
}

pub async fn rust_call_dart_loopback(
    callback: impl Fn(DemoStructForRustCallDart) -> DartFnFuture<DemoStructForRustCallDart> + UnwindSafe,
) {
    let result = callback(DemoStructForRustCallDart { name: "a".into() }).await;
    assert_eq!(&result.name, "a");
}

pub async fn rust_call_dart_with_dart_opaque_arg(
    input: DartOpaque,
    callback: impl Fn(DartOpaque) -> DartFnFuture<()> + UnwindSafe,
) {
    callback(input).await
}

pub async fn rust_call_dart_with_dart_opaque_result(
    callback: impl Fn() -> DartFnFuture<DartOpaque> + UnwindSafe,
) -> DartOpaque {
    callback().await
}

pub async fn TODO_dart_closure_be_async() {}
