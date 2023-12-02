use std::future::Future;
use std::panic::UnwindSafe;
use crate::generalized_isolate::IntoDart;
use futures::FutureExt;
use crate::thread_pool::ThreadPool;
use crate::handler::error::Error;
use crate::handler::error_handler::ErrorHandler;
use crate::handler::executor::Executor;
use crate::handler::handler::{FfiCallMode, TaskContext, TaskRetFutTrait, TaskInfo};
use crate::misc::into_into_dart::IntoIntoDart;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::context::TaskRust2DartContext;
use crate::rust2dart::sender::Rust2DartSender;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnSrc;
use crate::{rust_async, transfer};
use std::panic;

/// The default executor used.
/// It creates an internal thread pool, and each call to a Rust function is
/// handled by a different thread.
pub struct SimpleExecutor<EH: ErrorHandler> {
    error_handler: EH,
    thread_pool: ThreadPool,
}

impl<EH: ErrorHandler> SimpleExecutor<EH> {
    /// Create a new executor backed by a thread pool.
    pub fn new(error_handler: EH) -> Self {
        SimpleExecutor { error_handler }
    }
}

impl<EH: ErrorHandler + Sync> Executor for SimpleExecutor<EH> {
    fn execute<TaskFn, TaskRetDirect, TaskRetData, Er>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext) -> Result<TaskRetDirect, Er> + Send + UnwindSafe + 'static,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static,
    {
        let eh = self.error_handler;
        let eh2 = self.error_handler;

        let TaskInfo { port, mode, .. } = task_info;

        self.thread_pool.execute(transfer!(|port: Option<MessagePort>| {
            let port2 = port.as_ref().cloned();
            let thread_result = panic::catch_unwind(move || {
                let port2 = port2.expect("(worker) thread");
                #[allow(clippy::clone_on_copy)]
                    let sender = Rust2DartSender::new(port2.clone());

                let task_context = TaskContext::new(TaskRust2DartContext::new(sender));
                let ret = task(task_context)
                    .map(|e| e.into_into_dart().into_dart());

                match ret {
                    Ok(result) => {
                        match mode {
                            FfiCallMode::Normal => {
                                sender.success(result);
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
        }));
    }

    fn execute_sync<SyncTaskFn, TaskRetDirect, TaskRetData, Er>(
        &self,
        _task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> Result<WireSyncReturnSrc, Er>
    where
        SyncTaskFn: FnOnce() -> Result<TaskRetDirect, Er> + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart,
    {
        sync_task()
            .map(|value| WireSyncReturnSrc::new_from_data(value, Rust2DartAction::Success))
    }

    #[cfg(feature = "rust-async")]
    fn execute_async<TaskFn, TaskRetFut, TaskRetDirect, TaskRetData, Er>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut: Future<Output = Result<TaskRetDirect, Er>> + TaskRetFutTrait + UnwindSafe,
        TaskRetDirect: IntoIntoDart<TaskRetData>,
        TaskRetData: IntoDart,
        Er: IntoDart + 'static,
    {
        // TODO merge with `execute` case later
        // TODO avoid lock later

        let eh = self.error_handler;
        let eh2 = self.error_handler;

        rust_async::spawn(async move {
            let TaskInfo { port, mode, .. } = task_info;
            let port2 = port.as_ref().cloned();

            // TODO rename variable (not "thread" anymore)
            let thread_result = async {
                let port2 = port2.expect("(worker) thread");
                #[allow(clippy::clone_on_copy)]
                    let sender = Rust2DartSender::new(port2.clone());

                let task_context = TaskContext::new(TaskRust2DartContext::new(sender));
                let ret = task(task_context)
                    .await
                    .map(|e| e.into_into_dart().into_dart());

                match ret {
                    Ok(result) => {
                        match mode {
                            FfiCallMode::Normal => {
                                sender.success(result);
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
            }.catch_unwind().await;

            if let Err(error) = thread_result {
                eh.handle_error(port.expect("(worker) eh"), Error::Panic(error));
            }
        });
    }
}
