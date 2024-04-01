use crate::rust_async::BaseAsyncRuntime;
pub use async_std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use async_std::task::JoinHandle as AsyncJoinHandle;
use futures::never::Never;
use futures::FutureExt;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

pub struct JoinHandle<O>(async_std::task::JoinHandle<O>);

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, Never>;

    #[cfg(not(target_os = "unknown"))]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.0.poll_unpin(cx) {
            Poll::Ready(res) => Poll::Ready(Ok(res)),
            Poll::Pending => Poll::Pending,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.0.poll(cx) {
            Poll::Ready(res) => Poll::Ready(Ok(res)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> Deref for JoinHandle<T> {
    type Target = async_std::task::JoinHandle<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
pub struct SimpleAsyncRuntime();

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

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    JoinHandle(async_std::task::spawn(future))
}

pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    JoinHandle(async_std::task::spawn_local(future))
}

pub fn spawn_blocking<F>(func: F) -> JoinHandle<F::Output>
where
    F: FnOnce() + Send + 'static,
    F::Output: Send + 'static,
{
    JoinHandle(async_std::task::spawn_blocking(func))
}

pub fn spawn_blocking_with<F, R, TP>(f: F, _thread_pool_on_web: TP) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    JoinHandle(async_std::task::spawn_blocking(f))
}
