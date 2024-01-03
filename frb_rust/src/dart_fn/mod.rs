pub(crate) mod handler;

use std::future::Future;
use std::pin::Pin;

/// Roughly speaking, just BoxFuture + UnwindSafe.
pub type DartFnFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
