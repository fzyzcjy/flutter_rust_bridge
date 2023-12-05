use crate::platform_types::DartAbi;
use futures::future::BoxFuture;
use std::future::Future;
use std::ops::Deref;
use std::panic::UnwindSafe;
use std::pin::Pin;

/// Roughly speaking, just BoxFuture + UnwindSafe.
pub type DartFnFuture<T> = Pin<Box<dyn Future<Output = T> + Send + UnwindSafe + 'static>>;

pub fn dart_fn_invoke<Ret>(dart_fn_and_args: Vec<DartAbi>) -> DartFnFuture<Ret> {
    todo!()
}
