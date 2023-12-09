// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_fn.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "syncSse", "rustAsyncSse"]}

use flutter_rust_bridge::DartFnFuture;
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

// TODO
// pub async fn rust_call_dart_one_arg(callback: DartFn<fn(String) -> BoxFuture<'static, ()>>) {
//     callback("a".to_owned()).await;
// }
//
// pub async fn rust_call_dart_two_args(
//     callback: DartFn<fn(String, DemoStructForRustCallDart) -> BoxFuture<'static, ()>>,
// ) {
//     callback(
//         "a".to_owned(),
//         DemoStructForRustCallDart {
//             name: "b".to_owned(),
//         },
//     )
//     .await;
// }
//
// pub async fn rust_call_dart_return(callback: DartFn<fn() -> BoxFuture<'static, String>>) {
//     let result = callback().await;
//     assert_eq!(&result, "a");
// }
//
// pub async fn rust_call_dart_loopback(
//     callback: DartFn<
//         fn(DemoStructForRustCallDart) -> BoxFuture<'static, DemoStructForRustCallDart>,
//     >,
// ) {
//     let result = callback(DemoStructForRustCallDart { name: "a".into() }).await;
//     assert_eq!(&result.name, "a");
// }
//
// pub async fn rust_call_dart_with_dart_opaque_arg(
//     input: DartOpaque,
//     callback: DartFn<fn(DartOpaque) -> BoxFuture<'static, ()>>,
// ) {
//     callback(input).await
// }
//
// pub async fn rust_call_dart_with_dart_opaque_result(
//     callback: DartFn<fn() -> BoxFuture<'static, DartOpaque>>,
// ) -> DartOpaque {
//     callback().await
// }
//
// pub async fn TODO_dart_closure_be_async(){}
