use crate::platform_types::DartAbi;
use futures::future::BoxFuture;
use std::ops::Deref;

/// Roughly speaking, just BoxFuture + UnwindSafe.
pub type DartFnFuture<T> = Pin<Box<dyn Future<Output = T> + Send + UnwindSafe + 'static>>;

pub fn dart_fn_invoke<Ret>(closure_and_args: Vec<DartAbi>) -> DartFnFuture<Ret> {
    todo!()
}
