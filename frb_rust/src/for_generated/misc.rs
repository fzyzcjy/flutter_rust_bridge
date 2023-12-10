use crate::DartFnFuture;
use std::future::Future;
use std::panic::UnwindSafe;

// ref: futures `boxed()`
pub fn convert_into_dart_fn_future<T: Future<Output = String> + Send + UnwindSafe + 'static>(
    raw: T,
) -> DartFnFuture<String> {
    Box::pin(raw)
}
