#[cfg(not(wasm))]
fn get_thread_count() -> usize {
    #[allow(unused_mut, unused_assignments)]
    let mut thread_count: usize = 4;
    #[cfg(feature = "thread-count-single")]
    {
        worker_count = 1;
    }
    #[cfg(feature = "thread-count-max")]
    {
        thread_count = std::thread::available_parallelism().unwrap().get();
    }
    thread_count
}

#[cfg(wasm)]
fn get_worker_count() -> usize {
    #[allow(unused_mut, unused_assignments)]
    let mut worker_count: usize = 4;
    #[cfg(feature = "worker-count-single")]
    {
        worker_count = 1;
    }
    #[cfg(feature = "worker-count-max")]
    {
        worker_count = 16;
    }
    worker_count
}

#[cfg(not(wasm))]
mod io {
    use super::*;
    use lazy_static::lazy_static;
    use parking_lot::Mutex;
    use threadpool::ThreadPool;

    lazy_static! {
        pub static ref THREAD_POOL: Mutex<ThreadPool> = Mutex::new(ThreadPool::with_name(
            "frb_workerpool".into(),
            get_thread_count()
        ));
    }
}

#[cfg(not(wasm))]
pub use io::THREAD_POOL;

#[cfg(wasm)]
mod web {
    use crate::{script_path, wasm_bindgen_src::pool::WorkerPool};

    use super::*;

    thread_local! {
        pub static WORKER_POOL: Option<WorkerPool> = WorkerPool::new(
            get_worker_count(), script_path().unwrap())
                .map_err(|err| crate::console_error!("Failed to spawn worker: {:?}", err)).ok()
    }
}

#[cfg(wasm)]
pub use web::WORKER_POOL;
