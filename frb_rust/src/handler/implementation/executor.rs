use crate::codec::BaseCodec;
use crate::generalized_isolate::{Channel, IntoDart};
use crate::handler::error::Error;
use crate::handler::error_handler::ErrorHandler;
use crate::handler::executor::Executor;
use crate::handler::handler::{FfiCallMode, TaskContext, TaskInfo, TaskRetFutTrait};
use crate::misc::into_into_dart::IntoIntoDart;
use crate::platform_types::{DartAbi, MessagePort};
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::context::TaskRust2DartContext;
use crate::rust2dart::sender::Rust2DartSender;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnWrapperTrait;
use crate::rust_async::BaseAsyncRuntime;
use crate::thread_pool::BaseThreadPool;
use crate::{rust_async, transfer};
use allo_isolate::ffi::DartCObject;
use futures::FutureExt;
use parking_lot::Mutex;
use std::future::Future;
use std::panic;
use std::panic::{AssertUnwindSafe, UnwindSafe};

/// The default executor used.
/// It creates an internal thread pool, and each call to a Rust function is
/// handled by a different thread.
pub struct SimpleExecutor<EH: ErrorHandler, TP: BaseThreadPool, AR: BaseAsyncRuntime> {
    error_handler: EH,
    thread_pool: TP,
    async_runtime: AR,
}

impl<EH: ErrorHandler, TP: BaseThreadPool, AR: BaseAsyncRuntime> SimpleExecutor<EH, TP, AR> {
    /// Create a new executor backed by a thread pool.
    pub fn new(error_handler: EH, thread_pool: TP, async_runtime: AR) -> Self {
        SimpleExecutor {
            error_handler,
            thread_pool,
            async_runtime,
        }
    }

    pub fn thread_pool(&self) -> &TP {
        &self.thread_pool
    }
}

impl<EH: ErrorHandler + Sync, TP: BaseThreadPool, AR: BaseAsyncRuntime> Executor
    for SimpleExecutor<EH, TP, AR>
{
    fn execute_normal<Rust2DartCodec, TaskFn>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> Result<DartCObject, DartCObject>
            + Send
            + UnwindSafe
            + 'static,
        Rust2DartCodec: BaseCodec,
    {
        let eh = self.error_handler;
        let eh2 = self.error_handler;

        let TaskInfo { port, mode, .. } = task_info;
        let port = port.unwrap();

        self.thread_pool.execute(transfer!(|port: MessagePort| {
            let port2 = port.clone();
            let thread_result = panic::catch_unwind(|| {
                #[allow(clippy::clone_on_copy)]
                let sender = Rust2DartSender::new(Channel::new(port2.clone()));
                let task_context = TaskContext::new(TaskRust2DartContext::new(sender.clone()));

                let ret = task(task_context);

                ExecuteNormalOrAsyncUtils::handle_result::<Rust2DartCodec, _>(
                    ret, mode, sender, eh2, port2,
                );
            });

            if let Err(error) = thread_result {
                eh.handle_error::<Rust2DartCodec>(port, Error::Panic(error));
            }
        }));
    }

    fn execute_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        _task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> Result<Rust2DartCodec::WireSyncReturnWrapper, Rust2DartCodec::WireSyncReturnWrapper>
    where
        SyncTaskFn: FnOnce() -> Result<
                Rust2DartCodec::WireSyncReturnWrapper,
                Rust2DartCodec::WireSyncReturnWrapper,
            > + UnwindSafe,
        Rust2DartCodec: BaseCodec,
    {
        sync_task()
    }

    #[cfg(feature = "rust-async")]
    fn execute_async<Rust2DartCodec, TaskFn, TaskRetFut>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut:
            Future<Output = Result<DartCObject, DartCObject>> + TaskRetFutTrait + UnwindSafe,
        Rust2DartCodec: BaseCodec,
    {
        let eh = self.error_handler;
        let eh2 = self.error_handler;

        self.async_runtime.spawn(async move {
            let TaskInfo { port, mode, .. } = task_info;
            let port = port.unwrap();
            let port2 = port.clone();

            let async_result = async {
                #[allow(clippy::clone_on_copy)]
                let sender = Rust2DartSender::new(Channel::new(port2.clone()));
                let task_context = TaskContext::new(TaskRust2DartContext::new(sender.clone()));

                let ret = task(task_context).await;

                ExecuteNormalOrAsyncUtils::handle_result::<Rust2DartCodec, _>(
                    ret, mode, sender, eh2, port2,
                );
            }
            .catch_unwind()
            .await;

            if let Err(error) = async_result {
                eh.handle_error::<Rust2DartCodec>(port, Error::Panic(error));
            }
        });
    }
}

struct ExecuteNormalOrAsyncUtils;

impl ExecuteNormalOrAsyncUtils {
    fn handle_result<Rust2DartCodec, EH>(
        ret: Result<DartCObject, DartCObject>,
        mode: FfiCallMode,
        sender: Rust2DartSender,
        eh: EH,
        port: MessagePort,
    ) where
        EH: ErrorHandler + Sync,
        Rust2DartCodec: BaseCodec,
    {
        match ret {
            Ok(result) => {
                match mode {
                    FfiCallMode::Normal => {
                        // TODO wrong, the user should have already encoded?
                        sender.send(Rust2DartCodec::encode(
                            result.into_into_dart(),
                            Rust2DartAction::Success,
                        ));
                    }
                    FfiCallMode::Stream => {
                        // nothing - ignore the return value of a Stream-typed function
                    }
                    _ => unreachable!(),
                }
            }
            Err(error) => {
                eh.handle_error::<Rust2DartCodec>(port, Error::CustomError(Box::new(error)));
            }
        };
    }
}
