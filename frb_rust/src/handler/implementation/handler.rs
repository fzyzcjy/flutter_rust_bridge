use crate::dart_fn::DartFnFuture;
use crate::dart_opaque::{DartOpaque, DartOpaqueWireType};
use crate::generalized_isolate::IntoDart;
use crate::handler::error::Error;
use crate::handler::error_handler::ErrorHandler;
use crate::handler::executor::Executor;
use crate::handler::handler::{Handler, TaskContext, TaskInfo, TaskRetFutTrait};
use crate::handler::implementation::error_handler::ReportDartErrorHandler;
use crate::handler::implementation::executor::SimpleExecutor;
use crate::misc::into_into_dart::IntoIntoDart;
use crate::platform_types::message_port_to_handle;
use crate::platform_types::DartAbi;
use crate::platform_types::MessagePort;
use crate::platform_types::SendableMessagePortHandle;
use crate::platform_types::WireSyncReturn;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnSrc;
use crate::rust_async::{BaseAsyncRuntime, SimpleAsyncRuntime};
use crate::thread_pool::BaseThreadPool;
use log::warn;
use std::future::Future;
use std::panic;
use std::panic::UnwindSafe;
use std::sync::Mutex;

/// The default handler used by the generated code.
pub type DefaultHandler<TP> = SimpleHandler<
    SimpleExecutor<ReportDartErrorHandler, TP, SimpleAsyncRuntime>,
    ReportDartErrorHandler,
>;

impl<TP: BaseThreadPool> DefaultHandler<TP> {
    pub fn new_simple(thread_pool: TP) -> Self {
        Self::new(
            SimpleExecutor::new(ReportDartErrorHandler, thread_pool, Default::default()),
            ReportDartErrorHandler,
        )
    }

    pub fn thread_pool(&self) -> &TP {
        self.executor.thread_pool()
    }
}

/// The simple handler uses a simple thread pool to execute tasks.
pub struct SimpleHandler<E: Executor, EH: ErrorHandler> {
    executor: E,
    error_handler: EH,
    config: Mutex<Option<SimpleHandlerConfig>>,
}

struct SimpleHandlerConfig {
    dart_opaque_drop_port: SendableMessagePortHandle,
    dart_fn_invoke_port: SendableMessagePortHandle,
}

impl<E: Executor, H: ErrorHandler> SimpleHandler<E, H> {
    /// Create a new default handler.
    pub fn new(executor: E, error_handler: H) -> Self {
        SimpleHandler {
            executor,
            error_handler,
            config: Mutex::new(None),
        }
    }
}

impl<E: Executor, EH: ErrorHandler> Handler for SimpleHandler<E, EH> {
    fn initialize(&self, dart_opaque_drop_port: MessagePort, dart_fn_invoke_port: MessagePort) {
        // Again, as mentioned below, ensure panics never cross FFI boundary
        let _ = panic::catch_unwind(|| {
            if let Ok(mut config) = self.config.lock() {
                if config.is_some() {
                    let msg = "SimpleHandler.initialize is called multiple times.
* If you are hot-restarting Dart (Flutter) while reusing the same Rust, it is usually normal.
* However, if you are running two live FRB Dart instances while one FRB Rust instance, it is usually problematic.
* If you see this in unit tests, try `dart test --concurrency=1`";
                    warn!("{}", msg);
                    println!("{}", msg); // when users do not enable log
                }

                *config = Some(SimpleHandlerConfig {
                    dart_opaque_drop_port: message_port_to_handle(&dart_opaque_drop_port),
                    dart_fn_invoke_port: message_port_to_handle(&dart_fn_invoke_port),
                });
            }
        });
    }

    fn wrap_normal<PrepareFn, TaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext) -> Result<TaskRetDirect, Er> + Send + UnwindSafe + 'static,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static,
    {
        self.wrap_normal_or_async(task_info, prepare, |task_info, task| {
            self.executor.execute_normal(task_info, task)
        })
    }

    fn wrap_sync<SyncTaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> WireSyncReturn
    where
        SyncTaskFn: FnOnce() -> Result<TaskRetDirect, Er> + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static,
    {
        // NOTE This extra [catch_unwind] **SHOULD** be put outside **ALL** code!
        // For reason, see comments in [wrap]
        panic::catch_unwind(move || {
            let catch_unwind_result = panic::catch_unwind(move || {
                match self.executor.execute_sync(task_info, sync_task) {
                    Ok(data) => data,
                    Err(err) => self
                        .error_handler
                        .handle_error_sync(Error::CustomError(Box::new(err))),
                }
            });
            catch_unwind_result
                .unwrap_or_else(|error| self.error_handler.handle_error_sync(Error::Panic(error)))
                .leak()
        })
        // Deliberately construct simplest possible WireSyncReturn object
        // instead of more realistic things like `WireSyncReturnSrc::new(Panic, ...)`.
        // See comments in [wrap] for why.
        .unwrap_or_else(|_| WireSyncReturnSrc::new_raw(().into_dart()).leak())
    }

    #[cfg(feature = "rust-async")]
    fn wrap_async<PrepareFn, TaskFn, TaskRetFut, TaskRetDirect, TaskRetData, Er>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut: Future<Output = Result<TaskRetDirect, Er>> + TaskRetFutTrait + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static,
    {
        self.wrap_normal_or_async(task_info, prepare, |task_info, task| {
            self.executor.execute_async(task_info, task)
        })
    }

    unsafe fn wire2api_dart_opaque(&self, raw: DartOpaqueWireType) -> DartOpaque {
        let drop_port = (self.config.lock().expect("cannot get config lock"))
            .as_ref()
            .expect("no handler config")
            .dart_opaque_drop_port
            .to_owned();
        crate::dart_opaque::wire2api_dart_opaque(raw, drop_port)
    }

    fn dart_fn_invoke<Ret>(&self, dart_fn_and_args: Vec<DartAbi>) -> DartFnFuture<Ret> {
        todo!()
    }
}

impl<E: Executor, EH: ErrorHandler> SimpleHandler<E, EH> {
    fn wrap_normal_or_async<PrepareFn, TaskFn, TaskFnRet, ExecuteFn>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
        execute: ExecuteFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext) -> TaskFnRet,
        ExecuteFn: FnOnce(TaskInfo, TaskFn) + UnwindSafe,
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
            let task_info2 = task_info.clone();
            if let Err(error) = panic::catch_unwind(move || {
                let task = prepare();
                execute(task_info2, task);
            }) {
                self.error_handler
                    .handle_error(task_info.port.unwrap(), Error::Panic(error));
            }
        });
    }
}
