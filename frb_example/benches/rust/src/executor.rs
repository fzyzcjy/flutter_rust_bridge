use std::{
    panic::UnwindSafe,
    sync::{Arc, Mutex},
};

use flutter_rust_bridge::{
    handler::{
        self, ErrorHandler, Executor, ReportDartErrorHandler, SimpleHandler, ThreadPoolExecutor,
    },
    rust2dart::TaskCallback,
    IntoDart, MessagePort, WrapInfo,
};

use crate::api::{Metric, Unit};
const ERROR_MUTEX_LOCK: &str = "Error on mutex lock";

lazy_static::lazy_static! {
  static ref METRICS: Arc<Mutex<Vec<Metric>>> = Arc::new(Mutex::new(vec![]));
}

pub type BenchHandler = SimpleHandler<BenchExecutor, BenchErrorHandler>;

pub trait Metrics {
    fn metrics(&self) -> Vec<Metric>;
}

impl Metrics for SimpleHandler<BenchExecutor, BenchErrorHandler> {
    fn metrics(&self) -> Vec<Metric> {
        let guard = METRICS.lock().expect(ERROR_MUTEX_LOCK);
        guard.clone()
    }
}

fn record(debug_name_string: &str, elapsed: u64, unit: Unit) {
    let mut guard = METRICS.lock().expect(ERROR_MUTEX_LOCK);
    guard.push(Metric {
        name: debug_name_string.to_string(),
        value: Some(elapsed),
        extra: None,
        unit,
    });
}

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

pub struct BenchExecutor {
    inner: ThreadPoolExecutor<BenchErrorHandler>,
    json: bool,
}

#[cfg(not(target_family = "wasm"))]
fn maybe_trace(json: bool) {
    use tracing_subscriber::FmtSubscriber;
    let subscriber = if json {
        None
    } else {
        Some(
            FmtSubscriber::builder()
                .with_max_level(tracing::Level::TRACE)
                .finish(),
        )
    };
    if let Some(subscriber) = subscriber {
        tracing::subscriber::set_global_default(subscriber)
            .expect("Setting default subscriber failed");
    }
}

#[cfg(target_family = "wasm")]
#[inline(always)]
fn maybe_trace(_: bool) {}

impl BenchExecutor {
    pub(crate) fn new(error_handler: BenchErrorHandler) -> Self {
        let json = std::env::var("JSON")
            .unwrap_or_else(|_| "false".into())
            .parse::<bool>()
            .expect("Invalid JSON env var (expected boolean)");
        maybe_trace(json);
        Self {
            inner: ThreadPoolExecutor::new(error_handler),
            json,
        }
    }
}

impl Executor for BenchExecutor {
    fn execute<TaskFn, TaskRet>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> anyhow::Result<TaskRet> + Send + UnwindSafe + 'static,
        TaskRet: IntoDart,
    {
        let debug_name_string = wrap_info.debug_name.to_string();
        let json = self.json;
        self.inner.execute(wrap_info, move |task_callback| {
            Self::bench_around(&debug_name_string, json, move || task(task_callback))
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
        self.inner.execute_sync(wrap_info, move || {
            Self::bench_around(&debug_name_string, self.json, sync_task)
        })
    }
}

#[cfg(not(target_family = "wasm"))]
impl BenchExecutor {
    fn bench_around<F, R>(bench_name: &str, json: bool, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        if !json {
            use tracing::{span, trace, Level};
            span!(Level::TRACE, "frb-executor");
            trace!("(Rust) execute [{}] start", bench_name);
        }
        let start = std::time::Instant::now();
        let ret = f();
        let elapsed = start.elapsed().as_nanos();
        if !json {
            use tracing::{span, trace, Level};
            span!(Level::TRACE, "frb-executor");
            trace!(
                "(Rust) execute [{}] end delta_time={}{}",
                bench_name,
                elapsed,
                Unit::Nanoseconds.acronym()
            );
        } else {
            record(bench_name, elapsed as u64, Unit::Nanoseconds);
        }
        ret
    }
}

#[cfg(target_family = "wasm")]
impl BenchExecutor {
    fn bench_around<F, R>(bench_name: &str, json: bool, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        if !json {
            use js_sys::Array;
            use wasm_bindgen::JsValue;
            use web_sys::console;
            let log = format!("(Rust) execute [{bench_name}] start");
            let prepare = Array::new();
            prepare.push(&JsValue::from_str(&log));
            console::log(&prepare);
        }
        let start = chrono::Utc::now().timestamp_millis() as u64;
        let ret = f();
        let end = chrono::Utc::now().timestamp_millis() as u64;
        let elapsed = end - start;
        if !json {
            use js_sys::Array;
            use wasm_bindgen::JsValue;
            use web_sys::console;
            let log = format!(
                "(Rust) execute [{bench_name}] end delta_time={elapsed}{}",
                Unit::Milliseconds.acronym()
            );
            let prepare = Array::new();
            prepare.push(&JsValue::from_str(&log));
            console::log(&prepare);
        }
        record(bench_name, elapsed, Unit::Milliseconds);
        ret
    }
}
