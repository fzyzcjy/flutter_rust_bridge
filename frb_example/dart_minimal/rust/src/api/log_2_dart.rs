use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use log::{LevelFilter, Metadata, Record};

#[frb(non_opaque)]
pub struct Log2Dart {
    // TODO change String into Record
    stream_sink: StreamSink<String>,
    log_level: LevelFilter,
}

//TODO avoid this and rather Box log_fn into a Send enabled box
// unsafe impl Sync for Log2Dart<'_> {}
// unsafe impl Send for Log2Dart<'_> {}

// TODO implement Debug for StreamSink in frb_generated
// impl std::fmt::Debug for StreamSink<std::string::String> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("StreamSink")
//             // .field("base", &self.base)
//             .finish()
//     }
// }

// TODO take a log level
pub fn initialize_log2dart(log_stream: StreamSink<String>) {
    log::set_boxed_logger(Box::new(Log2Dart {
        stream_sink: log_stream,
        log_level: LevelFilter::Info,
    }))
    .map(|()| log::set_max_level(LevelFilter::Info))
    .expect("initialize_log2dart is called only once!")
}
// impl<T: Send + Sync> log::Log for Log2Dart {
impl log::Log for Log2Dart {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.stream_sink
                .add(format!("{} - {}", record.level(), record.args()))
                .expect("could not add to stream while sending to dart ");
        }
    }

    fn flush(&self) {
        //no need to flush the StreamSink
    }
}
