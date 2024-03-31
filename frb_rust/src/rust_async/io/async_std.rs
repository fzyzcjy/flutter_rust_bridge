use crate::rust_async::BaseAsyncRuntime;
pub use async_std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use async_std::task::JoinHandle;
use std::future::Future;

#[derive(Debug)]
pub struct SimpleAsyncRuntime();

impl Default for SimpleAsyncRuntime {
    fn default() -> Self {
        return Self();
    }
}

impl BaseAsyncRuntime for SimpleAsyncRuntime {
    type JoinHandle<O> = async_std::task::JoinHandle<O>;

    fn spawn<F>(&self, future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        async_std::task::spawn(future)
    }

    fn spawn_blocking<F>(&self, func: F) -> Self::JoinHandle<F::Output>
    where
        F: FnOnce() + Send + 'static,
        F::Output: Send + 'static,
    {
        async_std::task::spawn_blocking(func)
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        async_std::task::block_on(future)
    }
}
