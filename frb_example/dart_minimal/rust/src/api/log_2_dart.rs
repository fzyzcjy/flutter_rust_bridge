use crate::frb_generated::StreamSink;
use log::{Level, LevelFilter, Metadata, Record};
use std::{
    fmt::format,
    sync::{atomic::AtomicBool, OnceLock},
};

pub(crate) static LOG2DART: Log2Dart = Log2Dart {};
// TODO change String into Record

static STREAM_SINK: OnceLock<StreamSink<String>> = OnceLock::new();
static IS_READY: AtomicBool = AtomicBool::new(false);
pub struct Log2Dart {}

//TODO avoid this and rather Box log_fn into a Send enabled box
// unsafe impl Sync for Log2Dart<'_> {}
// unsafe impl Send for Log2Dart<'_> {}

// TODO implement Debug for StreamSink in frb_generated
impl std::fmt::Debug for StreamSink<std::string::String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamSink")
            // .field("base", &self.base)
            .finish()
    }
}

fn debug_logging(dbg_msg: &str) {
    STREAM_SINK
        .get()
        .expect("steam sink has been set")
        .add(format!("DEBUG: {}", dbg_msg))
        .expect("message could be sent to dart");
}

// impl Log2Dart {
pub fn init_log2dart(log_stream: StreamSink<String>) {
    log_stream.add("DEBUG: init log2dart".to_string()).unwrap();
    if !IS_READY.load(std::sync::atomic::Ordering::Relaxed) {
        log_stream.add("DEBUG: setting up".to_string()).unwrap();
        STREAM_SINK
            .set(log_stream)
            .expect("Could not initialize log stream");

        debug_logging("stream stored");

        log::set_logger(&LOG2DART)
            .map(|()| log::set_max_level(LevelFilter::Info))
            .expect("error setting the log2dart logger");
        debug_logging("logger registered");
        IS_READY.store(true, std::sync::atomic::Ordering::Relaxed);

        log::info!("From Rust: Initialized logger");
    }
}
// impl<T: Send + Sync> log::Log for Log2Dart {
impl log::Log for Log2Dart {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        debug_logging("\n trying to log now ...");
        let is_ready = IS_READY.load(std::sync::atomic::Ordering::Relaxed);
        debug_logging(&format!("\n IS_READY: {is_ready}").to_string());
        if self.enabled(record.metadata()) && IS_READY.load(std::sync::atomic::Ordering::Relaxed) {
            debug_logging("conditions are ok");

            debug_logging(&format!("Shall log this message {record:?}").to_string());
            STREAM_SINK
                .get()
                .expect("steam has been set")
                .add(format!(
                    "in log2dart: {} - {}",
                    record.level(),
                    record.args()
                ))
                .expect("could not add to stream while sending to dart ");
            debug_logging("log over");
            // println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        //TODO flush the StreamSink
    }
}
