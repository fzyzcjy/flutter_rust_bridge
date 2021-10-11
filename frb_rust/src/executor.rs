use std::panic;
use std::panic::UnwindSafe;

use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use threadpool::ThreadPool;

use crate::rust2dart::Rust2Dart;
use crate::support::DartCObject;

pub type ExecutorTask = Box<dyn FnOnce() -> Result<DartCObject> + Send + UnwindSafe + 'static>;

/// Provide your own executor to customize how to execute your function calls
pub trait Executor {
    fn execute(&self, debug_name: &str, port: i64, f: ExecutorTask);
}

/// The default executor uses a simple thread pool to execute tasks.
pub struct DefaultExecutor;

impl DefaultExecutor {
    pub fn new() -> Self {
        DefaultExecutor {}
    }
}

impl Default for DefaultExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor for DefaultExecutor {
    fn execute(&self, _debug_name: &str, port: i64, f: ExecutorTask) {
        const DEFAULT_WORKER_THREAD_POOL_NUM_WORKERS: usize = 4;

        lazy_static! {
            static ref DEFAULT_WORKER_THREAD_POOL: Mutex<ThreadPool> =
                Mutex::new(ThreadPool::new(DEFAULT_WORKER_THREAD_POOL_NUM_WORKERS));
        }

        let result = panic::catch_unwind(move || {
            DEFAULT_WORKER_THREAD_POOL.lock().execute(move || {
                let thread_result = panic::catch_unwind(move || {
                    let rust2dart = Rust2Dart::new(port);

                    let ret = f();

                    match ret {
                        Ok(result) => rust2dart.success(result),
                        Err(error) => {
                            rust2dart.error("GENERAL_ERROR".to_string(), error.to_string())
                        }
                    };
                });

                if let Err(err) = thread_result {
                    Rust2Dart::new(port).error("PANIC_ERROR".to_string(), format!("{:?}", err));
                }
            });
        });

        if let Err(err) = result {
            Rust2Dart::new(port).error("PANIC_ERROR".to_string(), format!("{:?}", err));
        }
    }
}
