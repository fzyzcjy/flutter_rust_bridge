use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::future::Future;
use std::panic::RefUnwindSafe;
use tokio::task::JoinHandle;

pub use tokio::runtime::Runtime as SimpleAsyncRuntime;

pub trait BaseAsyncRuntime: RefUnwindSafe {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}

impl BaseAsyncRuntime for SimpleAsyncRuntime {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.spawn(future)
    }
}
