use crate::thread_pool::BaseThreadPool;
use crate::transfer;
use futures::channel::oneshot;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
pub use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait BaseAsyncRuntime {
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleAsyncRuntime;

impl BaseAsyncRuntime for SimpleAsyncRuntime {
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(future)
    }
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    // wasm_bindgen runs everything on the same thread and only has spawn_local
    spawn_local(future)
}

pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    let (sender, handle) = JoinHandle::create_pair();
    wasm_bindgen_futures::spawn_local(async {
        let output = future.await;
        (sender.send(output)).unwrap_or_else(|_| panic!("Fail to send output in spawn_local"));
    });
    handle
}

#[cfg(feature = "thread-pool")]
pub fn spawn_blocking_with<F, R>(f: F, thread_pool: &impl BaseThreadPool) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let (sender, handle) = JoinHandle::create_pair();
    thread_pool.execute(transfer!(|| {
        let output = f();
        (sender.send(output)).unwrap_or_else(|_| panic!("Fail to send output in spawn_local"));
    }));
    handle
}

// ref: async-std's implementation
// https://github.com/async-rs/async-std/blob/8fea0500990c9d8977cbeef55bc9003cca39abc8/src/task/join_handle.rs#L23
pub struct JoinHandle<T>(oneshot::Receiver<T>);

impl<T> JoinHandle<T> {
    fn create_pair() -> (oneshot::Sender<T>, Self) {
        let (sender, receiver) = oneshot::channel::<T>();
        (sender, Self(receiver))
    }
}

impl<T> Future for JoinHandle<T> {
    // tokio uses `super::Result<T>`
    type Output = Result<T, oneshot::Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}
