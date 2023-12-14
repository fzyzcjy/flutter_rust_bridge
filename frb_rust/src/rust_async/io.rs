use std::future::Future;
use std::panic::{AssertUnwindSafe, RefUnwindSafe};
pub use tokio::spawn;
pub use tokio::task::JoinHandle;
pub use tokio::task::{spawn_blocking, spawn_local};

pub trait BaseAsyncRuntime: RefUnwindSafe {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}

// Why AssertUnwindSafe: https://github.com/tokio-rs/tokio/issues/6188
#[derive(Debug)]
pub struct SimpleAsyncRuntime(pub AssertUnwindSafe<tokio::runtime::Runtime>);

impl Default for SimpleAsyncRuntime {
    fn default() -> Self {
        Self(AssertUnwindSafe(tokio::runtime::Runtime::new().unwrap()))
    }
}

impl BaseAsyncRuntime for SimpleAsyncRuntime {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.0.spawn(future)
    }
}
