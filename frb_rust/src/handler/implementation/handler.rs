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
