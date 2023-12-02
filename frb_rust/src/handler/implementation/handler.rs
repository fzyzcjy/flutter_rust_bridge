use std::future::Future;
use std::panic;
use std::panic::UnwindSafe;
use allo_isolate::IntoDart;
use crate::handler::error_handler::ErrorHandler;
use crate::handler::executor::Executor;
use crate::handler::handler::{Handler, TaskRetFutTrait};
use crate::handler::implementation::error_handler::ReportDartErrorHandler;
use crate::handler::implementation::executor::ThreadPoolExecutor;

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

impl<E: Executor, EH: ErrorHandler> Handler for SimpleHandler<E, EH> {
    // TODO rename all these series (e.g. wrap -> wrap_normal)
    fn wrap<PrepareFn, TaskFn, TaskRetDirect, TaskRetData, Er>(&self, wrap_info: WrapInfo, prepare: PrepareFn)
        where
            PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
            TaskFn: FnOnce(TaskCallback) -> Result<TaskRetDirect, Er> + Send + UnwindSafe + 'static,
            TaskRetDirect: IntoIntoDart<TaskRetData>,
            TaskRetData: IntoDart,
            Er: IntoDart + 'static
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

    fn wrap_sync<SyncTaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> WireSyncReturn
        where
            SyncTaskFn: FnOnce() -> Result<TaskRetDirect, Er> + UnwindSafe,
            TaskRetDirect: IntoIntoDart<TaskRetData>,
            TaskRetData: IntoDart,
            Er: IntoDart + 'static
    {
        // NOTE This extra [catch_unwind] **SHOULD** be put outside **ALL** code!
        // For reason, see comments in [wrap]
        panic::catch_unwind(move || {
            let catch_unwind_result = panic::catch_unwind(move || {
                match self.executor.execute_sync(wrap_info, sync_task) {
                    Ok(data) => {
                        wire_sync_from_data(data.into_into_dart(), Rust2DartAction::Success)
                    }
                    Err(err) => self
                        .error_handler
                        .handle_error_sync(Error::CustomError(Box::new(err))),
                }
            });
            catch_unwind_result
                .unwrap_or_else(|error| self.error_handler.handle_error_sync(Error::Panic(error)))
        })
            .unwrap_or_else(|_| wire_sync_from_data(None::<()>, Rust2DartAction::Panic))
    }

    #[cfg(feature = "rust-async")]
    fn wrap_async<PrepareFn, TaskFn, TaskRetFut, TaskRetDirect, TaskRetData, Er>(
        &self,
        wrap_info: WrapInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskCallback) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut: Future<Output = Result<TaskRetDirect, Er>> + TaskRetFutTrait + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static
    {
        // TODO temporary copy-and-paste, should merge with case above later
        let _ = panic::catch_unwind(move || {
            let wrap_info2 = wrap_info.clone();
            if let Err(error) = panic::catch_unwind(move || {
                let task = prepare();
                self.executor.execute_async(wrap_info2, task);
            }) {
                self.error_handler
                    .handle_error(wrap_info.port.unwrap(), Error::Panic(error));
            }
        });
    }
}
