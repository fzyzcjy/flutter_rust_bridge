// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync"]}

use futures::future::BoxFuture;
use std::future::Future;

// TODO
struct DartFn<F> {
    inner: F,
}

// https://github.com/rust-lang/rust/issues/29625#issuecomment-1692602873
impl<F> std::ops::Deref for DartFn<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

struct DemoStructForRustCallDart {
    pub name: String,
}

pub async fn rust_call_dart_simple(callback: DartFn<fn() -> BoxFuture<()>>) {
    println!("rust_call_dart_simple before");
    callback().await;
    println!("rust_call_dart_simple after");
}

pub async fn rust_call_dart_one_arg(callback: DartFn<fn(String) -> BoxFuture<()>>) {
    callback("a".to_owned()).await;
}

pub async fn rust_call_dart_two_args(
    callback: DartFn<fn(String, DemoStructForRustCallDart) -> BoxFuture<()>>,
) {
    callback(
        "a".to_owned(),
        DemoStructForRustCallDart {
            name: "b".to_owned(),
        },
    )
    .await;
}

pub async fn rust_call_dart_return(callback: DartFn<fn() -> BoxFuture<String>>) {
    let result = callback().await;
    assert_eq!(&result, "a");
}

pub async fn rust_call_dart_loopback(
    callback: DartFn<fn(DemoStructForRustCallDart) -> BoxFuture<DemoStructForRustCallDart>>,
) {
    let result = callback(DemoStructForRustCallDart { name: "a".into() }).await;
    assert_eq!(&result.name, "a");
}
