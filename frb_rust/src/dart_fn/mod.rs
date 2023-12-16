pub(crate) mod handler;

use crate::platform_types::SendableMessagePortHandle;
use crate::DartOpaque;
use std::future::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;

/// Roughly speaking, just BoxFuture + UnwindSafe.
pub type DartFnFuture<T> = Pin<Box<dyn Future<Output = T> + Send + UnwindSafe + 'static>>;

#[derive(Debug, Clone)]
pub struct DartFn {
    closure: DartOpaque,
    invoke_port: SendableMessagePortHandle,
}
