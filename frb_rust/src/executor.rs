use std::panic;
use std::panic::UnwindSafe;

use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use threadpool::ThreadPool;

use crate::rust2dart::Rust2Dart;
use crate::support::DartCObject;

/// Provide your own executor to customize how to execute your function calls
pub trait Executor {
    fn wrap<PrepareFn, TaskFn, TaskRet>(&self, debug_name: &str, port: i64, prepare: PrepareFn)
    where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce() -> Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart;
}

/// The simple executor uses a simple thread pool to execute tasks.
pub struct SimpleExecutor;

impl SimpleExecutor {
    pub fn new() -> Self {
        SimpleExecutor {}
    }
}

impl Default for SimpleExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor for SimpleExecutor {
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
                            rust2dart.error("GENERAL_ERROR".to_string(), format!("{:?}", error))
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
