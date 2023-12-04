use std::future::Future;
use std::panic::RefUnwindSafe;
use wasm_bindgen_futures::spawn_local;

pub trait BaseAsyncRuntime: RefUnwindSafe {
    fn spawn<F>(future: F)
    where
        F: Future<Output = ()> + 'static;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleAsyncRuntime;

impl BaseAsyncRuntime for SimpleAsyncRuntime {
    fn spawn<F>(future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        spawn_local(future)
    }
}
