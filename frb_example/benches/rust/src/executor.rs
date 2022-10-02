use std::{panic::UnwindSafe, time::Instant};

use flutter_rust_bridge::{
    handler::{
        self, ErrorHandler, Executor, ReportDartErrorHandler, SimpleHandler, ThreadPoolExecutor,
    },
    rust2dart::TaskCallback,
    IntoDart, MessagePort, WrapInfo,
};

pub type BenchHandler = SimpleHandler<BenchExecutor, BenchErrorHandler>;

#[derive(Clone, Copy)]
pub struct BenchErrorHandler(ReportDartErrorHandler);

impl Default for BenchErrorHandler {
    fn default() -> Self {
        Self(ReportDartErrorHandler)
    }
}

impl ErrorHandler for BenchErrorHandler {
    fn handle_error(&self, port: MessagePort, error: handler::Error) {
        self.0.handle_error(port, error)
    }

    fn handle_error_sync(
        &self,
        error: flutter_rust_bridge::handler::Error,
    ) -> flutter_rust_bridge::support::WireSyncReturnStruct {
        self.0.handle_error_sync(error)
    }
}

pub struct BenchExecutor(ThreadPoolExecutor<BenchErrorHandler>);

impl BenchExecutor {
    pub(crate) fn new(error_handler: BenchErrorHandler) -> Self {
        Self(ThreadPoolExecutor::new(error_handler))
    }
}

impl Executor for BenchExecutor {
    fn execute<TaskFn, TaskRet>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> anyhow::Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart,
    {
        let debug_name_string = wrap_info.debug_name.to_string();
        self.0.execute(wrap_info, move |task_callback| {
            Self::bench_around(&debug_name_string, move || task(task_callback))
        })
    }

    fn execute_sync<SyncTaskFn, TaskRet>(
        &self,
        wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> anyhow::Result<flutter_rust_bridge::SyncReturn<TaskRet>>
    where
        flutter_rust_bridge::support::WireSyncReturnData: From<TaskRet>,
        SyncTaskFn: FnOnce() -> anyhow::Result<flutter_rust_bridge::SyncReturn<TaskRet>>
            + std::panic::UnwindSafe,
    {
        let debug_name_string = wrap_info.debug_name.to_string();
        self.0.execute_sync(wrap_info, move || {
            Self::bench_around(&debug_name_string, move || sync_task())
        })
    }
}

impl BenchExecutor {
    fn bench_around<F, R>(bench_name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        use tracing::{event, span, trace, Level};
        span!(Level::DEBUG, "frb-executor");
        let start = Instant::now();
        trace!("(Rust) execute [{}] start", bench_name);
        let ret = f();
        let elapsed = start.elapsed().as_nanos();
        trace!(
            "(Rust) execute [{}] end delta_time={}ns",
            bench_name,
            elapsed
        );
        // as per continuous-benchmark
        event!(
            Level::DEBUG,
            name = bench_name,
            unit = "ns",
            value = elapsed
        );
        ret
    }
}
