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

/// The simple handler uses a simple thread pool to execute tasks.
pub struct SimpleHandler<E: Executor, H: ErrorHandler> {
    executor: E,
    error_handler: H,
}

impl<E: Executor, H: ErrorHandler> SimpleHandler<E, H> {
    pub fn new(executor: E, error_handler: H) -> Self {
        SimpleHandler {
            executor,
            error_handler,
        }
    }
}

impl Default for SimpleHandler<ThreadPoolExecutor, ReportDartErrorHandler> {
    fn default() -> Self {
        Self::new(
            ThreadPoolExecutor::new(4, ReportDartErrorHandler),
            ReportDartErrorHandler,
        )
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
            if let Err(error) = panic::catch_unwind(move || {
                let task = prepare();
                self.executor.execute(task);
            }) {
                self.error_handler
                    .handle_error(port, ErrorType::Panic, error);
            }
        });
    }
}

pub trait Executor {
    fn execute<TaskFn, TaskRet>(&self, port: i64, task: TaskFn)
    where
        TaskFn: FnOnce() -> Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart;
}

pub struct ThreadPoolExecutor<'a> {
    thread_pool: Mutex<ThreadPool>,
    error_handler: &'a ErrorHandler,
}

impl<'a> ThreadPoolExecutor<'a> {
    pub fn new(num_threads: usize, error_handler: &'a ErrorHandler) -> Self {
        ThreadPoolExecutor {
            thread_pool: Mutex::new(ThreadPool::new(num_threads)),
            error_handler,
        }
    }
}

impl<'a> Executor for ThreadPoolExecutor<'a> {
    fn execute<TaskFn, TaskRet>(&self, port: i64, task: TaskFn)
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
                        self.error_handler
                            .handle_error(port, ErrorType::ResultError, error);
                    }
                };
            });

            if let Err(error) = thread_result {
                self.error_handler
                    .handle_error(port, ErrorType::Panic, error);
            }
        });
    }
}

pub enum ErrorType {
    ResultError,
    Panic,
}

pub trait ErrorHandler {
    fn handle_error<E>(&self, port: i64, typ: ErrorType, error: E);
}

pub struct ReportDartErrorHandler;

impl ErrorHandler for ReportDartErrorHandler {
    fn handle_error<E>(&self, port: i64, typ: ErrorType, error: E) {
        let error_code = match typ {
            ErrorType::ResultError => "RESULT_ERROR",
            ErrorType::Panic => "PANIC_ERROR",
        };
        Rust2Dart::new(port).error(error_code.to_string(), format!("{:?}", err));
    }
}
