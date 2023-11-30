use std::future::Future;
use wasm_bindgen_futures::spawn_local;

pub(crate) fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    spawn_local(future)
}
