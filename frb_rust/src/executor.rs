use std::panic;
use std::panic::UnwindSafe;

use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use threadpool::ThreadPool;

use crate::rust2dart::Rust2Dart;
use crate::support::DartCObject;

/// Provide your own handler to customize how to execute your function calls, etc
pub trait Handler {
    // Why separate [PrepareFn] and [TaskFn]: because some things cannot be [Send] (e.g. raw
    // pointers), so those can be done in [PrepareFn], while the real work is done in [TaskFn] with [Send].
    fn wrap<PrepareFn, TaskFn, TaskRet>(&self, debug_name: &str, port: i64, prepare: PrepareFn)
    where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce() -> Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart;
}

pub trait Executor {
    fn execute<TaskFn, TaskRet>(&self, task: TaskFn)
    where
        TaskFn: FnOnce() -> Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart;
}

/// The simple handler uses a simple thread pool to execute tasks.
pub struct SimpleHandler<E: Executor> {
    pub executor: E,
}

impl<E: Executor> SimpleHandler<E> {
    pub fn new(executor: E) -> Self {
        SimpleHandler { executor }
    }
}

impl Default for SimpleHandler {
    fn default() -> Self {
        Self::new(ThreadPoolExecutor::new(4))
    }
}

impl<E: Executor> Handler for SimpleHandler<E> {
    fn wrap<PrepareFn, TaskFn, TaskRet>(&self, debug_name: &str, port: i64, prepare: PrepareFn)
    where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce() -> Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart,
    {
        // NOTE This [catch_unwind] **SHOULD** be put outside **ALL** code!
        // Why do this: As nomicon says, unwind across languages is undefined behavior (UB).
        // Therefore, we should wrap a [catch_unwind] outside of *each and every* line of code
        // that can cause panic. Otherwise we may touch UB.
        // Why do not report error or something like that if this outer [catch_unwind] really
        // catches something: Because if we report error, that line of code itself can cause panic
        // as well. Then that new panic will go across language boundry and cause UB.
        // ref https://doc.rust-lang.org/nomicon/unwinding.html
        panic::catch_unwind(move || {
            if let Err(err) = panic::catch_unwind(move || {
                let task = prepare();
                self.executor.execute(task);
            }) {
                Rust2Dart::new(port).error("PANIC_ERROR".to_string(), format!("{:?}", err));
            }
        });
    }
}

pub struct ThreadPoolExecutor {
    thread_pool: Mutex<ThreadPool>,
}

impl ThreadPoolExecutor {
    pub fn new(num_threads: usize) -> Self {
        ThreadPoolExecutor {
            thread_pool: Mutex::new(ThreadPool::new(num_threads)),
        }
    }
}

impl Executor for ThreadPoolExecutor {
    fn execute<TaskFn, TaskRet>(&self, task: TaskFn)
    where
        TaskFn: FnOnce() -> Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart,
    {
        self.thread_pool.lock().execute(move || {
            let thread_result = panic::catch_unwind(move || {
                let rust2dart = Rust2Dart::new(port);

                let ret = task().map(|ret| ret.into_dart());

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
    }
}
