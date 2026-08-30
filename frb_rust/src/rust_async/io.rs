use std::future::Future;
use std::panic::AssertUnwindSafe;
#[cfg(any(test, frb_sanitize_runtime_shutdown))]
use std::sync::Mutex;
pub use tokio::spawn;
pub use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use tokio::task::spawn_local;
pub use tokio::task::JoinHandle;

pub trait BaseAsyncRuntime {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}

// Why AssertUnwindSafe: https://github.com/tokio-rs/tokio/issues/6188
#[derive(Debug)]
#[cfg(not(frb_sanitize_runtime_shutdown))]
pub struct SimpleAsyncRuntime(pub AssertUnwindSafe<tokio::runtime::Runtime>);

#[derive(Debug)]
#[cfg(frb_sanitize_runtime_shutdown)]
pub struct SimpleAsyncRuntime(pub Mutex<Option<AssertUnwindSafe<tokio::runtime::Runtime>>>);

#[cfg(not(frb_sanitize_runtime_shutdown))]
impl Default for SimpleAsyncRuntime {
    fn default() -> Self {
        Self(AssertUnwindSafe(tokio::runtime::Runtime::new().unwrap()))
    }
}

#[cfg(frb_sanitize_runtime_shutdown)]
impl Default for SimpleAsyncRuntime {
    fn default() -> Self {
        Self(Mutex::new(Some(AssertUnwindSafe(
            tokio::runtime::Runtime::new().unwrap(),
        ))))
    }
}

#[cfg(frb_sanitize_runtime_shutdown)]
impl SimpleAsyncRuntime {
    pub fn shutdown(&self) {
        shutdown_runtime(&self.0);
    }
}

#[cfg(frb_sanitize_runtime_shutdown)]
fn shutdown_runtime(runtime: &Mutex<Option<AssertUnwindSafe<tokio::runtime::Runtime>>>) {
    shutdown_runtime_with_probe(runtime, |_| {});
}

#[cfg(any(test, frb_sanitize_runtime_shutdown))]
fn shutdown_runtime_with_probe<F>(
    runtime_storage: &Mutex<Option<AssertUnwindSafe<tokio::runtime::Runtime>>>,
    before_drop: F,
) where
    F: FnOnce(&Mutex<Option<AssertUnwindSafe<tokio::runtime::Runtime>>>),
{
    let runtime = runtime_storage.lock().unwrap().take();
    before_drop(runtime_storage);
    drop(runtime);
}

#[cfg(not(frb_sanitize_runtime_shutdown))]
impl BaseAsyncRuntime for SimpleAsyncRuntime {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.0.spawn(future)
    }
}

#[cfg(frb_sanitize_runtime_shutdown)]
impl BaseAsyncRuntime for SimpleAsyncRuntime {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.0.lock().unwrap().as_ref().unwrap().0.spawn(future)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Shutdown unlocks runtime storage before dropping the owned runtime.
    #[test]
    fn shutdown_runtime_unlocks_storage_before_drop() {
        let runtime = Mutex::new(Some(AssertUnwindSafe(
            tokio::runtime::Runtime::new().unwrap(),
        )));
        shutdown_runtime_with_probe(&runtime, |runtime_storage| {
            assert!(runtime_storage.try_lock().unwrap().is_none());
        });

        assert!(runtime.try_lock().unwrap().is_none());
    }

    /// Runs spawned futures on the dedicated Tokio runtime.
    #[test]
    #[cfg(not(frb_sanitize_runtime_shutdown))]
    fn test_simple_async_runtime_spawns_futures() {
        let runtime = SimpleAsyncRuntime::default();
        let output = runtime.0.block_on(runtime.spawn(async { 7 }));

        assert_eq!(output.unwrap(), 7);
    }

    /// Runs blocking work on Tokio's blocking executor.
    #[test]
    #[cfg(not(frb_sanitize_runtime_shutdown))]
    fn test_spawn_blocking_with_returns_the_closure_result() {
        let runtime = SimpleAsyncRuntime::default();
        let output = runtime
            .0
            .block_on(async { spawn_blocking_with(|| 11, ()).await });

        assert_eq!(output.unwrap(), 11);
    }
}

/// Similar to tokio's `spawn_blocking`, except that you need to provide a second argumnet.
///
/// If you are using flutter_rust_bridge, the second argument can be easily provided:
/// Just use `FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()`.
///
/// More formally, the second argument is defined as:
///
/// * When on web: The thread pool you want to use.
/// * When on non-web: Unused, can be anything (since we use Tokio's built-in pool).
pub fn spawn_blocking_with<F, R, TP>(f: F, _thread_pool_on_web: TP) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(f)
}
