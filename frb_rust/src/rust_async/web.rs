use std::future::Future;
use std::thread::JoinHandle;
use wasm_bindgen_futures::spawn_local;

pub(crate) fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    spawn_local(future)
}
