use std::future::Future;

// ref: futures `boxed()`
#[cfg(feature = "dart-opaque")]
pub fn convert_into_dart_fn_future<T: Future<Output = O> + Send + 'static, O>(
    raw: T,
) -> crate::DartFnFuture<O> {
    Box::pin(raw)
}
