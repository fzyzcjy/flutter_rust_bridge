//! Wrappers and executors for Rust functions.

use std::any::Any;
use std::panic;
use std::panic::{RefUnwindSafe, UnwindSafe};

use crate::ffi::{IntoDart, MessagePort};

use crate::rust2dart::{BoxIntoDart, IntoIntoDart, Rust2Dart, TaskCallback};
use crate::support::WireSyncReturn;
use crate::{spawn, DartAbi, SyncReturn};

/// The types of return values for a particular Rust function.
#[derive(Copy, Clone)]
pub enum FfiCallMode {
    /// The default mode, returns a Dart `Future<T>`.
    Normal,
    /// Used by `SyncReturn<T>` to skip spawning workers.
    Sync,
    /// Returns a Dart `Stream<T>`.
    Stream,
}

/// Supporting information to identify a function's operating mode.
#[derive(Clone)]
pub struct WrapInfo {
    /// A Dart `SendPort`. [None] if the mode is [FfiCallMode::Sync].
    pub port: Option<MessagePort>,
    /// Usually the name of the function.
    pub debug_name: &'static str,
    /// The call mode of this function.
    pub mode: FfiCallMode,
}
/// Provide your own handler to customize how to execute your function calls, etc.
pub trait Handler {
    /// Prepares the arguments, executes a Rust function and sets up its return value.
    ///
    /// Why separate `PrepareFn` and `TaskFn`: because some things cannot be [`Send`] (e.g. raw
    /// pointers), so those can be done in `PrepareFn`, while the real work is done in `TaskFn` with [`Send`].
    ///
    /// The generated code depends on the fact that `PrepareFn` is synchronous to maintain
    /// correctness, therefore implementors of [`Handler`] must also uphold this property.
    ///
    /// If a Rust function returns [`SyncReturn`], it must be called with
    /// [`wrap_sync`](Handler::wrap_sync) instead.
    fn wrap<PrepareFn, TaskFn, TaskRet, D, Er>(&self, wrap_info: WrapInfo, prepare: PrepareFn)
    where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet, Er> + Send + UnwindSafe + 'static,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart + 'static;

    /// Same as [`wrap`][Handler::wrap], but the Rust function must return a [SyncReturn] and
    /// need not implement [Send].
    fn wrap_sync<SyncTaskFn, TaskRet, D, Er>(
        &self,
        wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> WireSyncReturn
    where
        SyncTaskFn: FnOnce() -> Result<SyncReturn<TaskRet>, Er> + UnwindSafe,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart + 'static;
}

/// The simple handler uses a simple thread pool to execute tasks.
pub struct SimpleHandler<E: Executor, EH: ErrorHandler> {
    executor: E,
    error_handler: EH,
}

impl<E: Executor, H: ErrorHandler> SimpleHandler<E, H> {
    /// Create a new default handler.
    pub fn new(executor: E, error_handler: H) -> Self {
        SimpleHandler {
            executor,
            error_handler,
        }
    }
}

/// The default handler used by the generated code.
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
    fn wrap<PrepareFn, TaskFn, TaskRet, D, Er>(&self, wrap_info: WrapInfo, prepare: PrepareFn)
    where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet, Er> + Send + UnwindSafe + 'static,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart + 'static,
    {
        // NOTE This extra [catch_unwind] **SHOULD** be put outside **ALL** code!
        // Why do this: As nomicon says, unwind across languages is undefined behavior (UB).
        // Therefore, we should wrap a [catch_unwind] outside of *each and every* line of code
        // that can cause panic. Otherwise we may touch UB.
        // Why do not report error or something like that if this outer [catch_unwind] really
        // catches something: Because if we report error, that line of code itself can cause panic
        // as well. Then that new panic will go across language boundary and cause UB.
        // ref https://doc.rust-lang.org/nomicon/unwinding.html
        let _ = panic::catch_unwind(move || {
            let wrap_info2 = wrap_info.clone();
            if let Err(error) = panic::catch_unwind(move || {
                let task = prepare();
                self.executor.execute(wrap_info2, task);
            }) {
                self.error_handler
                    .handle_error(wrap_info.port.unwrap(), Error::Panic(error));
            }
        });
    }

    fn wrap_sync<SyncTaskFn, TaskRet, D, Er>(
        &self,
        wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> WireSyncReturn
    where
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        SyncTaskFn: FnOnce() -> Result<SyncReturn<TaskRet>, Er> + UnwindSafe,
        Er: IntoDart + 'static,
    {
        // NOTE This extra [catch_unwind] **SHOULD** be put outside **ALL** code!
        // For reason, see comments in [wrap]
        panic::catch_unwind(move || {
            let catch_unwind_result = panic::catch_unwind(move || {
                match self.executor.execute_sync(wrap_info, sync_task) {
                    Ok(data) => wire_sync_from_data(data.0.into_into_dart(), true),
                    Err(err) => self
                        .error_handler
                        .handle_error_sync(Error::CustomError(Box::new(err))),
                }
            });
            catch_unwind_result
                .unwrap_or_else(|error| self.error_handler.handle_error_sync(Error::Panic(error)))
        })
        .unwrap_or_else(|_| wire_sync_from_data(None::<()>, false))
    }
}

/// An executor model for Rust functions.
///
/// For example, the default model is [ThreadPoolExecutor]
/// which runs each function in a separate thread.
pub trait Executor: RefUnwindSafe {
    /// Executes a Rust function and transforms its return value into a Dart-compatible
    /// value, i.e. types that implement [`IntoDart`].
    fn execute<TaskFn, TaskRet, D, Er>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet, Er> + Send + UnwindSafe + 'static,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart + 'static;

    /// Executes a Rust function that returns a [SyncReturn].
    fn execute_sync<SyncTaskFn, TaskRet, D, Er>(
        &self,
        wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> Result<SyncReturn<TaskRet>, Er>
    where
        SyncTaskFn: FnOnce() -> Result<SyncReturn<TaskRet>, Er> + UnwindSafe,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart + 'static;
}

/// The default executor used.
/// It creates an internal thread pool, and each call to a Rust function is
/// handled by a different thread.
pub struct ThreadPoolExecutor<EH: ErrorHandler> {
    error_handler: EH,
}

impl<EH: ErrorHandler> ThreadPoolExecutor<EH> {
    /// Create a new executor backed by a thread pool.
    pub fn new(error_handler: EH) -> Self {
        ThreadPoolExecutor { error_handler }
    }
}

impl<EH: ErrorHandler> Executor for ThreadPoolExecutor<EH> {
    fn execute<TaskFn, TaskRet, D, Er>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet, Er> + Send + UnwindSafe + 'static,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart + 'static,
    {
        let eh = self.error_handler;
        let eh2 = self.error_handler;

        let WrapInfo { port, mode, .. } = wrap_info;

        spawn!(|port: Option<MessagePort>| {
            let port2 = port.as_ref().cloned();
            let thread_result = panic::catch_unwind(move || {
                let port2 = port2.expect("(worker) thread");
                #[allow(clippy::clone_on_copy)]
                let rust2dart = Rust2Dart::new(port2.clone());

                let ret = task(TaskCallback::new(rust2dart.clone()))
                    .map(|e| e.into_into_dart().into_dart());

                match ret {
                    Ok(result) => {
                        match mode {
                            FfiCallMode::Normal => {
                                rust2dart.success(result);
                            }
                            FfiCallMode::Stream => {
                                // nothing - ignore the return value of a Stream-typed function
                            }
                            FfiCallMode::Sync => {
                                panic!("FfiCallMode::Sync should not call execute, please call execute_sync instead")
                            }
                        }
                    }
                    Err(error) => {
                        eh2.handle_error(port2, Error::CustomError(Box::new(error)));
                    }
                };
            });

            if let Err(error) = thread_result {
                eh.handle_error(port.expect("(worker) eh"), Error::Panic(error));
            }
        });
    }

    fn execute_sync<SyncTaskFn, TaskRet, D, Er>(
        &self,
        _wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> Result<SyncReturn<TaskRet>, Er>
    where
        SyncTaskFn: FnOnce() -> Result<SyncReturn<TaskRet>, Er> + UnwindSafe,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart,
    {
        sync_task()
    }
}

/// Errors that occur from normal code execution.
pub enum Error {
    /// Errors that implement [IntoDart].
    CustomError(Box<dyn BoxIntoDart>),
    /// Exceptional errors from panicking.
    Panic(Box<dyn Any + Send>),
}

fn error_to_string(panic_err: &Box<dyn Any + Send>) -> String {
    match panic_err.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match panic_err.downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<dyn Any>",
        },
    }
    .to_string()
}

impl Error {
    /// The message of the error.
    pub fn message(&self) -> String {
        match self {
            Error::CustomError(_e) => "Box<dyn BoxIntoDart>".to_string(),
            Error::Panic(panic_err) => error_to_string(panic_err),
        }
    }
}

impl IntoDart for Error {
    fn into_dart(self) -> DartAbi {
        match self {
            Error::CustomError(e) => e.box_into_dart(),
            Error::Panic(panic_err) => error_to_string(&panic_err).into_dart(),
        }
    }
}

/// A handler model that sends back the error to a Dart `SendPort`.
///
/// For example, instead of using the default [`ReportDartErrorHandler`],
/// you could implement your own handler that logs each error to stderr,
/// or to an external logging service.
pub trait ErrorHandler: UnwindSafe + RefUnwindSafe + Copy + Send + 'static {
    /// The default error handler.
    fn handle_error(&self, port: MessagePort, error: Error);

    /// Special handler only used for synchronous code.
    fn handle_error_sync(&self, error: Error) -> WireSyncReturn;
}

/// The default error handler used by generated code.
#[derive(Clone, Copy)]
pub struct ReportDartErrorHandler;

impl ErrorHandler for ReportDartErrorHandler {
    fn handle_error(&self, port: MessagePort, error: Error) {
        match error {
            e @ Error::CustomError(_) => Rust2Dart::new(port).error(e),
            e @ Error::Panic(_) => Rust2Dart::new(port).panic(e),
        };
    }

    fn handle_error_sync(&self, error: Error) -> WireSyncReturn {
        match error {
            Error::CustomError(err) => {
                let data = err.box_into_dart();
                let sync_return = vec![data, false.into_dart()].into_dart();

                #[cfg(not(wasm))]
                return crate::support::new_leak_box_ptr(sync_return);

                #[cfg(wasm)]
                return sync_return;
            }
            Error::Panic(err) => wire_sync_from_data(error_to_string(&err), false),
        }
    }
}

fn wire_sync_from_data<T: IntoDart>(data: T, success: bool) -> WireSyncReturn {
    let sync_return = vec![data.into_dart(), success.into_dart()].into_dart();

    #[cfg(not(wasm))]
    return crate::support::new_leak_box_ptr(sync_return);

    #[cfg(wasm)]
    return sync_return;
}
