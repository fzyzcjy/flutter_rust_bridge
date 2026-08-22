use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::generalized_isolate::Channel;
use crate::handler::error::Error;
use crate::handler::error_listener::ErrorListener;
use crate::handler::executor::Executor;
use crate::handler::handler::{TaskContext, TaskInfo, TaskRetFutTrait};
use crate::handler::implementation::error_listener::handle_non_sync_panic_error;
use crate::misc::panic_backtrace::{CatchUnwindWithBacktrace, PanicBacktrace};
use crate::platform_types::MessagePort;
use crate::rust2dart::sender::Rust2DartSender;
use crate::rust_async::BaseAsyncRuntime;
use crate::thread_pool::BaseThreadPool;
use crate::transfer;
#[cfg(feature = "rust-async")]
use futures::FutureExt;
use std::future::Future;
use std::panic::AssertUnwindSafe;

/// The default executor used.
/// It creates an internal thread pool, and each call to a Rust function is
/// handled by a different thread.
pub struct SimpleExecutor<EL: ErrorListener, TP: BaseThreadPool, AR: BaseAsyncRuntime> {
    error_listener: EL,
    thread_pool: TP,
    async_runtime: AR,
}

impl<EL: ErrorListener, TP: BaseThreadPool, AR: BaseAsyncRuntime> SimpleExecutor<EL, TP, AR> {
    /// Create a new executor backed by a thread pool.
    pub fn new(error_listener: EL, thread_pool: TP, async_runtime: AR) -> Self {
        SimpleExecutor {
            error_listener,
            thread_pool,
            async_runtime,
        }
    }

    pub fn thread_pool(&self) -> &TP {
        &self.thread_pool
    }

    pub fn async_runtime(&self) -> &AR {
        &self.async_runtime
    }
}

impl<EL: ErrorListener + Sync, TP: BaseThreadPool, AR: BaseAsyncRuntime> Executor
    for SimpleExecutor<EL, TP, AR>
{
    #[cfg(feature = "thread-pool")]
    fn execute_normal<Rust2DartCodec, TaskFn>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext) -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>
            + Send
            + 'static,
        Rust2DartCodec: BaseCodec,
    {
        let el = self.error_listener;
        let el2 = self.error_listener;

        let TaskInfo { port, .. } = task_info;
        let port: MessagePort = port.unwrap();

        self.thread_pool
            .execute(transfer!(|port: crate::platform_types::MessagePort| {
                #[allow(clippy::clone_on_copy)]
                let port2 = port.clone();
                let thread_result = PanicBacktrace::catch_unwind(AssertUnwindSafe(|| {
                    #[allow(clippy::clone_on_copy)]
                    let sender = Rust2DartSender::new(Channel::new(port2.clone()));
                    let task_context = TaskContext::new();

                    let ret = task(task_context);

                    ExecuteNormalOrAsyncUtils::handle_result::<Rust2DartCodec, _>(ret, sender, el2);
                }));

                if let Err(error) = thread_result {
                    handle_non_sync_panic_error::<Rust2DartCodec>(el, port, error);
                }
            }));
    }

    fn execute_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        _task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> Rust2DartCodec::Message
    where
        SyncTaskFn: FnOnce() -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>,
        Rust2DartCodec: BaseCodec,
    {
        match sync_task() {
            Ok(data) => data,
            Err(err) => {
                self.error_listener.on_error(Error::CustomError);
                err
            }
        }
    }

    #[cfg(feature = "rust-async")]
    fn execute_async<Rust2DartCodec, TaskFn, TaskRetFut>(&self, task_info: TaskInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskContext) -> TaskRetFut + Send + 'static,
        TaskRetFut: Future<Output = Result<Rust2DartCodec::Message, Rust2DartCodec::Message>>
            + TaskRetFutTrait,
        Rust2DartCodec: BaseCodec,
    {
        let el = self.error_listener;
        let el2 = self.error_listener;

        self.async_runtime.spawn(async move {
            let TaskInfo { port, .. } = task_info;
            let port = port.unwrap();
            #[allow(clippy::clone_on_copy)]
            let port2 = port.clone();

            let async_result = AssertUnwindSafe(async {
                #[allow(clippy::clone_on_copy)]
                let sender = Rust2DartSender::new(Channel::new(port2.clone()));
                let task_context = TaskContext::new();

                let ret = task(task_context).await;

                ExecuteNormalOrAsyncUtils::handle_result::<Rust2DartCodec, _>(ret, sender, el2);
            })
            .catch_unwind()
            .await;

            if let Err(err) = async_result {
                let err = CatchUnwindWithBacktrace::new(err, PanicBacktrace::take_last());
                handle_non_sync_panic_error::<Rust2DartCodec>(el, port, err);
            }
        });
    }
}

struct ExecuteNormalOrAsyncUtils;

impl ExecuteNormalOrAsyncUtils {
    fn handle_result<Rust2DartCodec, EL>(
        ret: Result<Rust2DartCodec::Message, Rust2DartCodec::Message>,
        sender: Rust2DartSender,
        el: EL,
    ) where
        EL: ErrorListener + Sync,
        Rust2DartCodec: BaseCodec,
    {
        match ret {
            Ok(result) => {
                sender.send_or_warn(result.into_dart_abi());
            }
            Err(error) => {
                el.on_error(Error::CustomError);
                sender.send_or_warn(error.into_dart_abi());
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleExecutor;
    use crate::codec::{BaseCodec, Rust2DartMessageTrait};
    use crate::handler::error::Error;
    use crate::handler::error_listener::ErrorListener;
    use crate::handler::executor::Executor;
    use crate::handler::handler::{FfiCallMode, TaskInfo};
    use crate::platform_types::DartAbi;
    use crate::rust_async::SimpleAsyncRuntime;
    use crate::thread_pool::SimpleThreadPool;
    use std::any::Any;
    use std::backtrace::Backtrace;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    static CUSTOM_ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[derive(Clone, Copy)]
    struct TestCodec;

    struct TestMessage(i32);

    impl Rust2DartMessageTrait for TestMessage {
        type WireSyncRust2DartType = i32;

        fn simplest() -> Self {
            Self(-1)
        }

        fn into_dart_abi(self) -> DartAbi {
            unreachable!()
        }

        unsafe fn from_raw_wire_sync(raw: Self::WireSyncRust2DartType) -> Self {
            Self(raw)
        }

        fn into_raw_wire_sync(self) -> Self::WireSyncRust2DartType {
            self.0
        }
    }

    impl BaseCodec for TestCodec {
        type Message = TestMessage;

        fn encode_panic(_: &Box<dyn Any + Send>, _: &Option<Backtrace>) -> Self::Message {
            TestMessage(99)
        }

        fn encode_close_stream() -> Self::Message {
            TestMessage(0)
        }
    }

    #[derive(Clone, Copy)]
    struct RecordingErrorListener;

    impl ErrorListener for RecordingErrorListener {
        fn on_error(&self, error: Error) {
            if matches!(error, Error::CustomError) {
                CUSTOM_ERROR_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    fn task_info() -> TaskInfo {
        TaskInfo {
            port: None,
            debug_name: "test",
            mode: FfiCallMode::Sync,
        }
    }

    /// Returns successful messages without notifying the error listener.
    #[test]
    fn test_execute_sync_returns_success_without_error_notification() {
        let _guard = TEST_LOCK.lock().unwrap();
        CUSTOM_ERROR_COUNT.store(0, Ordering::SeqCst);
        let executor = SimpleExecutor::new(
            RecordingErrorListener,
            SimpleThreadPool::default(),
            SimpleAsyncRuntime::default(),
        );

        let message = executor.execute_sync::<TestCodec, _>(task_info(), || Ok(TestMessage(7)));

        assert_eq!(message.0, 7);
        assert_eq!(CUSTOM_ERROR_COUNT.load(Ordering::SeqCst), 0);
    }

    /// Returns failed messages while notifying the error listener exactly once.
    #[test]
    fn test_execute_sync_returns_error_with_error_notification() {
        let _guard = TEST_LOCK.lock().unwrap();
        CUSTOM_ERROR_COUNT.store(0, Ordering::SeqCst);
        let executor = SimpleExecutor::new(
            RecordingErrorListener,
            SimpleThreadPool::default(),
            SimpleAsyncRuntime::default(),
        );

        let message = executor.execute_sync::<TestCodec, _>(task_info(), || Err(TestMessage(8)));

        assert_eq!(message.0, 8);
        assert_eq!(CUSTOM_ERROR_COUNT.load(Ordering::SeqCst), 1);
    }
}
