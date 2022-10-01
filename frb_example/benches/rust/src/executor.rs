use std::{panic::UnwindSafe, time::Instant};

use flutter_rust_bridge::{
    handler::{
        self, ErrorHandler, Executor, ReportDartErrorHandler, SimpleHandler, ThreadPoolExecutor,
    },
    rust2dart::TaskCallback,
    IntoDart, WrapInfo,
};

pub type BenchHandler =
    SimpleHandler<ThreadPoolExecutor<ReportDartErrorHandler>, ReportDartErrorHandler>;

#[derive(Clone, Copy)]
pub struct BenchErrorHandler(ReportDartErrorHandler);

impl ErrorHandler for BenchErrorHandler {
    fn handle_error(&self, port: i64, error: handler::Error) {
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
        let start = Instant::now();
        println!("(Rust) execute [{}] start", bench_name);
        let ret = f();
        println!(
            "(Rust) execute [{}] end delta_time={}ms",
            bench_name,
            start.elapsed().as_millis()
        );
        ret
    }
}
