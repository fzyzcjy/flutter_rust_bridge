use crate::codec::sse::Dart2RustMessageSse;
use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::handler::error::Error;
use crate::handler::error_listener::ErrorListener;
use crate::handler::executor::Executor;
use crate::handler::handler::{Handler, TaskContext, TaskInfo, TaskRetFutTrait};
use crate::handler::implementation::error_listener::{
    handle_non_sync_panic_error, NoOpErrorListener,
};
use crate::handler::implementation::executor::SimpleExecutor;
use crate::misc::panic_backtrace::PanicBacktrace;
use crate::platform_types::DartAbi;
use crate::rust_async::SimpleAsyncRuntime;
use crate::thread_pool::BaseThreadPool;
use std::future::Future;
use std::panic;
use std::panic::AssertUnwindSafe;

/// The default handler used by the generated code.
pub type DefaultHandler<TP> =
    SimpleHandler<SimpleExecutor<NoOpErrorListener, TP, SimpleAsyncRuntime>, NoOpErrorListener>;

impl<TP: BaseThreadPool> DefaultHandler<TP> {
    pub fn new_simple(thread_pool: TP) -> Self {
        Self::new(
            SimpleExecutor::new(NoOpErrorListener, thread_pool, Default::default()),
            NoOpErrorListener,
        )
    }

    pub fn thread_pool(&self) -> &TP {
        self.executor.thread_pool()
    }

    pub fn async_runtime(&self) -> &SimpleAsyncRuntime {
        self.executor.async_runtime()
    }
}

/// The simple handler uses a simple thread pool to execute tasks.
pub struct SimpleHandler<E: Executor, EL: ErrorListener> {
    executor: E,
    error_listener: EL,
    #[cfg(all(feature = "rust-async", feature = "dart-opaque"))]
    dart_fn_handler: crate::dart_fn::handler::DartFnHandler,
}

impl<E: Executor, H: ErrorListener> SimpleHandler<E, H> {
    /// Create a new default handler.
    pub fn new(executor: E, error_listener: H) -> Self {
        SimpleHandler {
            executor,
            error_listener,
            #[cfg(all(feature = "rust-async", feature = "dart-opaque"))]
            dart_fn_handler: crate::dart_fn::handler::DartFnHandler::new(),
        }
    }
}

impl<E: Executor, EL: ErrorListener> Handler for SimpleHandler<E, EL> {
    #[cfg(feature = "thread-pool")]
    fn wrap_normal<Rust2DartCodec, PrepareFn, TaskFn>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce(TaskContext) -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>
            + Send
            + 'static,
        Rust2DartCodec: BaseCodec,
    {
        self.wrap_normal_or_async::<Rust2DartCodec, _, _, _, _>(
            task_info,
            prepare,
            |task_info, task| {
                self.executor
                    .execute_normal::<Rust2DartCodec, _>(task_info, task)
            },
        )
    }

    fn wrap_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> <Rust2DartCodec::Message as Rust2DartMessageTrait>::WireSyncRust2DartType
    where
        SyncTaskFn: FnOnce() -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>,
        Rust2DartCodec: BaseCodec,
    {
        // NOTE This extra [catch_unwind] **SHOULD** be put outside **ALL** code!
        // For reason, see comments in [wrap]
        panic::catch_unwind(AssertUnwindSafe(move || {
            let catch_unwind_result = PanicBacktrace::catch_unwind(AssertUnwindSafe(move || {
                (self.executor).execute_sync::<Rust2DartCodec, _>(task_info, sync_task)
            }));
            catch_unwind_result
                .unwrap_or_else(|error| {
                    let message = Rust2DartCodec::encode_panic(&error.err, &error.backtrace);
                    self.error_listener.on_error(Error::Panic(error.err));
                    message
                })
                .into_raw_wire_sync()
        }))
        // Deliberately construct simplest possible WireSyncRust2Dart object
        // instead of more realistic things like `WireSyncRust2DartSrc::new(Panic, ...)`.
        // See comments in [wrap] for why.
        .unwrap_or_else(|_| Rust2DartCodec::Message::simplest().into_raw_wire_sync())
    }

    #[cfg(feature = "rust-async")]
    fn wrap_async<Rust2DartCodec, PrepareFn, TaskFn, TaskRetFut>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce(TaskContext) -> TaskRetFut + Send + 'static,
        TaskRetFut: Future<Output = Result<Rust2DartCodec::Message, Rust2DartCodec::Message>>
            + TaskRetFutTrait,
        Rust2DartCodec: BaseCodec,
    {
        self.wrap_normal_or_async::<Rust2DartCodec, _, _, _, _>(
            task_info,
            prepare,
            |task_info, task| {
                self.executor
                    .execute_async::<Rust2DartCodec, _, _>(task_info, task)
            },
        )
    }

    #[cfg(all(feature = "rust-async", feature = "dart-opaque"))]
    fn dart_fn_invoke(
        &self,
        dart_fn: crate::dart_opaque::DartOpaque,
        args: Vec<DartAbi>,
    ) -> crate::dart_fn::DartFnFuture<Dart2RustMessageSse> {
        self.dart_fn_handler.invoke(dart_fn, args)
    }

    #[cfg(all(feature = "rust-async", feature = "dart-opaque"))]
    fn dart_fn_handle_output(&self, call_id: i32, message: Dart2RustMessageSse) {
        self.dart_fn_handler.handle_output(call_id, message)
    }
}

impl<E: Executor, EL: ErrorListener> SimpleHandler<E, EL> {
    fn wrap_normal_or_async<Rust2DartCodec, PrepareFn, TaskFn, TaskFnRet, ExecuteFn>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
        execute: ExecuteFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn,
        TaskFn: FnOnce(TaskContext) -> TaskFnRet,
        ExecuteFn: FnOnce(TaskInfo, TaskFn),
        Rust2DartCodec: BaseCodec,
    {
        // NOTE This extra [catch_unwind] **SHOULD** be put outside **ALL** code!
        // Why do this: As nomicon says, unwind across languages is undefined behavior (UB).
        // Therefore, we should wrap a [catch_unwind] outside of *each and every* line of code
        // that can cause panic. Otherwise we may touch UB.
        // Why do not report error or something like that if this outer [catch_unwind] really
        // catches something: Because if we report error, that line of code itself can cause panic
        // as well. Then that new panic will go across language boundary and cause UB.
        // ref https://doc.rust-lang.org/nomicon/unwinding.html
        let _ = panic::catch_unwind(AssertUnwindSafe(move || {
            let task_info2 = task_info.clone();
            if let Err(error) = PanicBacktrace::catch_unwind(AssertUnwindSafe(move || {
                let task = prepare();
                execute(task_info2, task);
            })) {
                handle_non_sync_panic_error::<Rust2DartCodec>(
                    self.error_listener,
                    task_info.port.unwrap(),
                    error,
                );
            }
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleHandler;
    use crate::codec::{BaseCodec, Rust2DartMessageTrait};
    use crate::handler::error::Error;
    use crate::handler::error_listener::ErrorListener;
    use crate::handler::executor::Executor;
    use crate::handler::handler::{FfiCallMode, Handler, TaskContext, TaskInfo, TaskRetFutTrait};
    use crate::platform_types::DartAbi;
    use std::any::Any;
    use std::backtrace::Backtrace;
    use std::future::Future;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    static EXECUTE_SYNC_COUNT: AtomicUsize = AtomicUsize::new(0);
    static PANIC_ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
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

    struct RecordingExecutor;

    impl Executor for RecordingExecutor {
        #[cfg(feature = "thread-pool")]
        fn execute_normal<Rust2DartCodec, TaskFn>(&self, _: TaskInfo, _: TaskFn)
        where
            TaskFn: FnOnce(TaskContext) -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>
                + Send
                + 'static,
            Rust2DartCodec: BaseCodec,
        {
            unreachable!()
        }

        fn execute_sync<Rust2DartCodec, SyncTaskFn>(
            &self,
            _: TaskInfo,
            sync_task: SyncTaskFn,
        ) -> Rust2DartCodec::Message
        where
            SyncTaskFn: FnOnce() -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message>,
            Rust2DartCodec: BaseCodec,
        {
            EXECUTE_SYNC_COUNT.fetch_add(1, Ordering::SeqCst);
            sync_task().unwrap_or_else(|error| error)
        }

        #[cfg(feature = "rust-async")]
        fn execute_async<Rust2DartCodec, TaskFn, TaskRetFut>(&self, _: TaskInfo, _: TaskFn)
        where
            TaskFn: FnOnce(TaskContext) -> TaskRetFut + Send + 'static,
            TaskRetFut: Future<Output = Result<Rust2DartCodec::Message, Rust2DartCodec::Message>>
                + TaskRetFutTrait,
            Rust2DartCodec: BaseCodec,
        {
            unreachable!()
        }
    }

    #[derive(Clone, Copy)]
    struct RecordingErrorListener;

    impl ErrorListener for RecordingErrorListener {
        fn on_error(&self, error: Error) {
            if matches!(error, Error::Panic(_)) {
                PANIC_ERROR_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    #[derive(Clone, Copy)]
    struct PanickingErrorListener;

    impl ErrorListener for PanickingErrorListener {
        fn on_error(&self, _: Error) {
            panic!("listener panic")
        }
    }

    fn task_info() -> TaskInfo {
        TaskInfo {
            port: None,
            debug_name: "test",
            mode: FfiCallMode::Sync,
        }
    }

    /// Forwards sync success through the executor and retains its wire value.
    #[test]
    fn test_wrap_sync_forwards_successful_task_result() {
        let _guard = TEST_LOCK.lock().unwrap();
        EXECUTE_SYNC_COUNT.store(0, Ordering::SeqCst);
        PANIC_ERROR_COUNT.store(0, Ordering::SeqCst);
        let handler = SimpleHandler::new(RecordingExecutor, RecordingErrorListener);

        let wire = handler.wrap_sync::<TestCodec, _>(task_info(), || Ok(TestMessage(42)));

        assert_eq!(wire, 42);
        assert_eq!(EXECUTE_SYNC_COUNT.load(Ordering::SeqCst), 1);
        assert_eq!(PANIC_ERROR_COUNT.load(Ordering::SeqCst), 0);
    }

    /// Converts a task panic to the codec panic payload and reports it once.
    #[test]
    fn test_wrap_sync_reports_task_panic_with_codec_payload() {
        let _guard = TEST_LOCK.lock().unwrap();
        PANIC_ERROR_COUNT.store(0, Ordering::SeqCst);
        let handler = SimpleHandler::new(RecordingExecutor, RecordingErrorListener);

        let wire = handler
            .wrap_sync::<TestCodec, _>(task_info(), || -> Result<TestMessage, TestMessage> {
                panic!("task panic")
            });

        assert_eq!(wire, 99);
        assert_eq!(PANIC_ERROR_COUNT.load(Ordering::SeqCst), 1);
    }

    /// Falls back to the simplest wire value when panic reporting itself panics.
    #[test]
    fn test_wrap_sync_contains_listener_panic() {
        let _guard = TEST_LOCK.lock().unwrap();
        let handler = SimpleHandler::new(RecordingExecutor, PanickingErrorListener);

        let wire = handler
            .wrap_sync::<TestCodec, _>(task_info(), || -> Result<TestMessage, TestMessage> {
                panic!("task panic")
            });

        assert_eq!(wire, -1);
    }
}
