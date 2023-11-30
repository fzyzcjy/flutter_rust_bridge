use lazy_static::lazy_static;
use parking_lot::Mutex;
use tokio::runtime::Runtime;

lazy_static! {
    static ref ASYNC_RUNTIME: Mutex<Runtime> = Mutex::new(create_runtime());
}

pub(crate) fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let runtime = ASYNC_RUNTIME.lock();
    runtime.spawn(future)
}
