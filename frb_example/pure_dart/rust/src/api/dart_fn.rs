// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "syncSse"]}

use flutter_rust_bridge::{DartFnFuture, DartOpaque};
use std::future::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;

pub struct DemoStructForRustCallDartTwinNormal {
    pub name: String,
}

pub async fn rust_call_dart_simple_twin_normal(
    callback: impl Fn() -> DartFnFuture<()> + UnwindSafe,
) {
    println!("rust_call_dart_simple before");
    callback().await;
    println!("rust_call_dart_simple after");
}

pub async fn rust_call_dart_one_arg_twin_normal(
    callback: impl Fn(String) -> DartFnFuture<()> + UnwindSafe,
) {
    callback("a".to_owned()).await;
}

pub async fn rust_call_dart_two_args_twin_normal(
    callback: impl Fn(String, DemoStructForRustCallDartTwinNormal) -> DartFnFuture<()> + UnwindSafe,
) {
    callback(
        "a".to_owned(),
        DemoStructForRustCallDartTwinNormal {
            name: "b".to_owned(),
        },
    )
    .await;
}

pub async fn rust_call_dart_return_twin_normal(
    callback: impl Fn() -> DartFnFuture<String> + UnwindSafe,
) {
    let result = callback().await;
    assert_eq!(&result, "a");
}

pub async fn rust_call_dart_loopback_twin_normal(
    callback: impl Fn(DemoStructForRustCallDartTwinNormal) -> DartFnFuture<DemoStructForRustCallDartTwinNormal>
        + UnwindSafe,
) {
    let result = callback(DemoStructForRustCallDartTwinNormal { name: "a".into() }).await;
    assert_eq!(&result.name, "a");
}

pub async fn rust_call_dart_with_dart_opaque_arg_twin_normal(
    input: DartOpaque,
    callback: impl Fn(DartOpaque) -> DartFnFuture<()> + UnwindSafe,
) {
    callback(input).await
}

pub async fn rust_call_dart_with_dart_opaque_result_twin_normal(
    callback: impl Fn() -> DartFnFuture<DartOpaque> + UnwindSafe,
) -> DartOpaque {
    callback().await
}

// TODO this has no code on rust side
// pub async fn TODO_dart_closure_be_async() {}

// TODO make dart side stateful
pub async fn rust_call_dart_multi_times_twin_normal(
    callback: impl Fn() -> DartFnFuture<()> + UnwindSafe,
    num_times: i32,
) {
    for i in 0..num_times {
        callback().await;
    }
}
