#[wasm_bindgen::prelude::wasm_bindgen]
fn get_logical_core_count() {
    #[cfg(not(wasm))]
    {
        std::thread::available_parallelism().unwrap().get()
    }
    #[cfg(wasm)]
    {
        let script = r#"
                function get_logical_cores() {
                    return navigator.hardwareConcurrency || 4;
                }
            "#;
        let js_value = js_sys::eval(script).unwrap();
        js_value.as_f64().unwrap() as usize
    }
}

fn get_worker_count() -> usize {
    #[cfg(all(feature = "worker-max", feature = "worker-single"))]
    {
        compile_error!(r#"Cannot use "worker-max" and "worker-single" features together"#);
    }
    #[cfg(not(any(feature = "worker-max", feature = "worker-single")))]
    {
        4 // Default
    }
    #[cfg(feature = "worker-single")]
    {
        1 // One
    }
    #[cfg(feature = "worker-max")]
    {
        get_logical_core_count() // Logical cores
    }
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
            get_worker_count()
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
