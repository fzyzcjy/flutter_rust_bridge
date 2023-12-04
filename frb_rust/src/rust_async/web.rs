use std::future::Future;
use wasm_bindgen_futures::spawn_local;

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    spawn_local(future)
}

pub trait BaseAsyncRuntime {
    fn spawn<F>(future: F)
    where
        F: Future<Output = ()> + 'static;
}
