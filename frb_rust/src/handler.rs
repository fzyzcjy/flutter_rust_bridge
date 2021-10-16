use std::any::Any;
use std::panic;
use std::panic::{RefUnwindSafe, UnwindSafe};

use allo_isolate::IntoDart;
use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use threadpool::ThreadPool;

use crate::rust2dart::{Rust2Dart, TaskCallback};

#[derive(Copy, Clone)]
pub enum FfiCallMode {
    Normal,
    Stream,
}

#[derive(Clone)]
pub struct WrapInfo {
    pub debug_name: &'static str,
    pub port: i64,
    pub mode: FfiCallMode,
}

/// Provide your own handler to customize how to execute your function calls, etc
pub trait Handler {
    // Why separate [PrepareFn] and [TaskFn]: because some things cannot be [Send] (e.g. raw
    // pointers), so those can be done in [PrepareFn], while the real work is done in [TaskFn] with [Send].
    fn wrap<PrepareFn, TaskFn, TaskRet>(&self, wrap_info: WrapInfo, prepare: PrepareFn)
    where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart;
}

/// The simple handler uses a simple thread pool to execute tasks.
pub struct SimpleHandler<E: Executor, EH: ErrorHandler> {
    executor: E,
    error_handler: EH,
}

impl<E: Executor, H: ErrorHandler> SimpleHandler<E, H> {
    pub fn new(executor: E, error_handler: H) -> Self {
        SimpleHandler {
            executor,
            error_handler,
        }
    }
}

pub type DefaultHandler =
    SimpleHandler<ThreadPoolExecutor<ReportDartErrorHandler>, ReportDartErrorHandler>;

impl Default for DefaultHandler {
    fn default() -> Self {
        Self::new(
            ThreadPoolExecutor::new(ReportDartErrorHandler),
            ReportDartErrorHandler,
        )
    }
}

impl<E: Executor, EH: ErrorHandler> Handler for SimpleHandler<E, EH> {
    fn wrap<PrepareFn, TaskFn, TaskRet>(&self, wrap_info: WrapInfo, prepare: PrepareFn)
    where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet> + Send + UnwindSafe + 'static,
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
        let _ = panic::catch_unwind(move || {
            let wrap_info2 = wrap_info.clone();
            if let Err(error) = panic::catch_unwind(move || {
                let task = prepare();
                self.executor.execute(wrap_info2, task);
            }) {
                self.error_handler
                    .handle_error(wrap_info.port, Error::Panic(error));
            }
        });
    }
}

pub trait Executor: RefUnwindSafe {
    fn execute<TaskFn, TaskRet>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart;
}

pub struct ThreadPoolExecutor<EH: ErrorHandler> {
    error_handler: EH,
}

impl<EH: ErrorHandler> ThreadPoolExecutor<EH> {
    pub fn new(error_handler: EH) -> Self {
        ThreadPoolExecutor { error_handler }
    }
}

impl<EH: ErrorHandler> Executor for ThreadPoolExecutor<EH> {
    fn execute<TaskFn, TaskRet>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart,
    {
        const NUM_WORKERS: usize = 4;
        lazy_static! {
            static ref THREAD_POOL: Mutex<ThreadPool> = Mutex::new(ThreadPool::new(NUM_WORKERS));
        }

        let eh = self.error_handler;
        let eh2 = self.error_handler;
        THREAD_POOL.lock().execute(move || {
            let wrap_info2 = wrap_info.clone();
            let thread_result = panic::catch_unwind(move || {
                let rust2dart = Rust2Dart::new(wrap_info2.port);

                let ret = task(TaskCallback::new(rust2dart)).map(|ret| ret.into_dart());

                match ret {
                    Ok(result) => {
                        match wrap_info2.mode {
                            FfiCallMode::Normal => {
                                rust2dart.success(result);
                            }
                            FfiCallMode::Stream => {
                                // nothing - ignore the return value of a Stream-typed function
                            }
                        }
                    }
                    Err(error) => {
                        eh2.handle_error(wrap_info2.port, Error::ResultError(error));
                    }
                };
            });

            if let Err(error) = thread_result {
                eh.handle_error(wrap_info.port, Error::Panic(error));
            }
        });
    }
}

#[derive(Debug)]
pub enum Error {
    ResultError(anyhow::Error),
    Panic(Box<dyn Any + Send>),
}

impl Error {
    pub fn code(&self) -> &'static str {
        match self {
            Error::ResultError(_) => "RESULT_ERROR",
            Error::Panic(_) => "PANIC_ERROR",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Error::ResultError(e) => format!("{:?}", e),
            Error::Panic(panic_err) => match panic_err.downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match panic_err.downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<dyn Any>",
                },
            }
            .to_string(),
        }
    }
}

pub trait ErrorHandler: UnwindSafe + RefUnwindSafe + Copy + Send + 'static {
    fn handle_error(&self, port: i64, error: Error);
}

#[derive(Clone, Copy)]
pub struct ReportDartErrorHandler;

impl ErrorHandler for ReportDartErrorHandler {
    fn handle_error(&self, port: i64, error: Error) {
        Rust2Dart::new(port).error(error.code().to_string(), error.message());
    }
}
