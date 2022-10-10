use std::{
    panic::UnwindSafe,
    sync::{Arc, Mutex},
};

use flutter_rust_bridge::{
    handler::{Executor, ReportDartErrorHandler, SimpleHandler, ThreadPoolExecutor},
    rust2dart::TaskCallback,
    IntoDart, WrapInfo,
};

use crate::api::{Metric, Unit};
const ERROR_MUTEX_LOCK: &str = "Error on mutex lock";

lazy_static::lazy_static! {
  static ref METRICS: Arc<Mutex<Metrics>> = Arc::new(Mutex::new(Metrics(vec![])));
}

#[derive(Debug, Clone)]
pub struct Metrics(pub Vec<Metric>);

pub type BenchHandler = SimpleHandler<BenchExecutor, ReportDartErrorHandler>;

pub trait Benchmark {
    fn metrics(&self) -> Metrics;
    /// record a benchmark metric
    ///
    /// all benchmark metrics are stored in [`METRICS`]
    fn record(&self, debug_name_string: &str, elapsed: u64, unit: Unit);
}

impl Benchmark for METRICS {
    fn metrics(&self) -> Metrics {
        let guard = self.lock().expect(ERROR_MUTEX_LOCK);
        guard.clone()
    }
    fn record(&self, debug_name_string: &str, elapsed: u64, unit: Unit) {
        let mut guard = self.lock().expect(ERROR_MUTEX_LOCK);
        guard.0.push(Metric {
            name: debug_name_string.to_string(),
            value: Some(elapsed),
            extra: None,
            unit,
        });
    }
}

impl Benchmark for SimpleHandler<BenchExecutor, ReportDartErrorHandler> {
    fn metrics(&self) -> Metrics {
        METRICS.metrics()
    }
    fn record(&self, debug_name_string: &str, elapsed: u64, unit: Unit) {
        METRICS.record(debug_name_string, elapsed, unit);
    }
}

/// a slightly more elaborated newtype
///
/// this is the bridge's custom [executor](http://cjycode.com/flutter_rust_bridge/feature/handler.html#example-log-when-execution-starts-and-ends).
pub struct BenchExecutor {
    inner: ThreadPoolExecutor<ReportDartErrorHandler>,
    json: bool,
}

/// subscribe for tracing only on native platforms
#[cfg(not(target_family = "wasm"))]
fn maybe_subscribe(json: bool) {
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

/// on web platform, this is a no-op
#[cfg(target_family = "wasm")]
#[inline(always)]
fn maybe_subscribe(_: bool) {}

impl BenchExecutor {
    /// also retrieve env var to set mode to **stdout** or **json**
    /// and subscribe to `tracing` if needed
    pub(crate) fn new(error_handler: ReportDartErrorHandler) -> Self {
        let json = std::env::var("JSON")
            .unwrap_or_else(|_| "false".into())
            .parse::<bool>()
            .expect("Invalid JSON env var (expected boolean)");
        maybe_subscribe(json);
        Self {
            inner: ThreadPoolExecutor::new(error_handler),
            json,
        }
    }
}

impl Executor for BenchExecutor {
    /// wrap [`bench_around`] over async FFI calls
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

    /// wrap [`bench_around`] over sync FFI calls
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

/// executor (on native platforms)
///
/// use [tracing](https://docs.rs/crate/tracing/0.1.36) for stdout
/// or record metrics as json
#[cfg(not(target_family = "wasm"))]
impl BenchExecutor {
    /// benchmark every bridge-wired function call (on native platforms)
    ///
    /// - smallest time unit in rust is _nanoseconds_
    ///   > note that at this point we're out of dart vm, whose smallest time unit is _microseconds_
    /// - underlying implementation relies on standard [Instant](https://doc.rust-lang.org/std/time/struct.Instant.html)
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
            METRICS.record(bench_name, elapsed as u64, Unit::Nanoseconds);
        }
        ret
    }
}

/// executor (on web platform)
///
/// use [console](web_sys::console) a.k.a [console](https://developer.mozilla.org/en-US/docs/Web/API/console) for stdout
/// or record metrics as json
#[cfg(target_family = "wasm")]
impl BenchExecutor {
    /// benchmark every bridge-wired function call (on web platform)
    ///
    /// - smallest time unit in dart2js is _millisecond_
    ///   > note that at this point, you're mostly in a distorted version of JavaScript land
    /// - based on [DateTime<Utc>](https://docs.rs/chrono/0.4.20/chrono/struct.DateTime.html)
    /// - [instant](https://crates.io/crates/instant) was considered too but `stdweb` didn't play well
    ///   with [wasm-pack no-modules](https://rustwasm.github.io/docs/wasm-pack/commands/build.html?highlight=no-modules#target)
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
