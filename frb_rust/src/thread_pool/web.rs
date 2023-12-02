use thread_local::ThreadLocal;

pub use crate::third_party::wasm_bindgen::worker_pool::WorkerPool as ThreadPool;

pub(crate) struct SingleThreadAccessorThreadPool(ThreadLocal<ThreadPool>);
