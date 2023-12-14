use futures::channel::oneshot;
use std::future::Future;
use std::panic::RefUnwindSafe;
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait BaseAsyncRuntime: RefUnwindSafe {
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
    let (sender, receiver) = oneshot::channel::<F::Output>();
    wasm_bindgen_futures::spawn_local(async {
        let output = future.await;
        sender.send(output);
    });
    JoinHandle(receiver)
}

pub fn spawn_blocking<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    todo!()
}

// ref: async-std uses naive futures_channel::oneshot::Receiver
// in the JoinHandle
// https://github.com/async-rs/async-std/blob/8fea0500990c9d8977cbeef55bc9003cca39abc8/src/task/join_handle.rs#L23
pub struct JoinHandle<T>(oneshot::Receiver<T>);

impl<T> Future for JoinHandle<T> {
    // tokio uses `super::Result<T>`
    type Output = anyhow::Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.poll(cx)
    }
}
