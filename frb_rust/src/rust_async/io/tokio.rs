use crate::rust_async::BaseAsyncRuntime;
use std::future::Future;
use std::panic::AssertUnwindSafe;
pub use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use tokio::task::JoinHandle;

// Why AssertUnwindSafe: https://github.com/tokio-rs/tokio/issues/6188
#[derive(Debug)]
pub struct SimpleAsyncRuntime(pub AssertUnwindSafe<tokio::runtime::Runtime>);

impl Default for SimpleAsyncRuntime {
    fn default() -> Self {
        Self(AssertUnwindSafe(tokio::runtime::Runtime::new().unwrap()))
    }
}

impl BaseAsyncRuntime for SimpleAsyncRuntime {
    type JoinHandle<O> = tokio::task::JoinHandle<O>;

    fn spawn<F>(&self, future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.0.spawn(future)
    }

    fn spawn_blocking<F>(&self, func: F) -> Self::JoinHandle<F::Output>
    where
        F: FnOnce() + Send + 'static,
        F::Output: Send + 'static,
    {
        self.0.spawn_blocking(func)
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.0.block_on(future)
    }
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    tokio::spawn(future)
}

pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    tokio::task::spawn_local(future)
}

pub fn spawn_blocking<F>(func: F) -> JoinHandle<F::Output>
where
    F: FnOnce() + Send + 'static,
    F::Output: Send + 'static,
{
    tokio::task::spawn_blocking(func)
}

pub fn spawn_blocking_with<F, R, TP>(f: F, _thread_pool_on_web: TP) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(f)
}
