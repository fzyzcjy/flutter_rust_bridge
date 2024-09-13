use flutter_rust_bridge::DartFnFuture;
use log::{Level, LevelFilter, Metadata, Record};
use std::sync::{Arc, OnceLock, RwLock};

// static dart_log_fn: OnceLock<Fn(String) -> DartFnFuture<String>> = OnceLock::new();

pub struct Log2Dart<'a> {
    // level_filter: LevelFilter,
    // log_fn: Arc<dyn Fn(String) -> DartFnFuture<String>>,

    // log_fn: Fn(String) -> DartFnFuture<String>,
    log_fn: Arc<dyn Fn(String) -> String + 'a>,
    // log_fn: Arc<dyn Fn(String)>,
    // log_fn: (dyn Fn(
    //     String,
    // )
    //     -> Pin<Box<(dyn std::future::Future<Output = String> + std::marker::Send + 'static)>>
    //      + 'static),
    // logfn: (dyn Fn(String) + Send),
    // makes it not sync & send:
    // log_fn: Fn(String) -> DartFnFuture<String>,
}

//TODO avoid this and rather Box log_fn into a Send enabled box
unsafe impl Sync for Log2Dart<'_> {}
unsafe impl Send for Log2Dart<'_> {}

impl<'a> Log2Dart<'a> {
    pub fn new(log_fn: impl Fn(String) -> String + 'a) -> Self {
        // pub fn new(log_fn: impl Fn(String) -> DartFnFuture<String>) -> Self {
        // Self { RwLock<dyn Fn(String) -> DartFnFuture<String>>::new(log_fn) }
        Self {
            log_fn: Arc::new(log_fn),
            // log_fn,
        }
    }
    // pub fn init(dart_log_callback: impl Fn(String) -> DartFnFuture<String>) {
    //     dart_log_fn.store(dart_log_callback);
    // }
}

impl log::Log for Log2Dart<'_> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // dart_log_fn
            //     .get()
            //     .expect("log fn has been initialized")
            // .log_fn("{} - {}", record.level(), record.args());
            (self.log_fn)(format!("{} - {}", record.level(), record.args()));
            // println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
