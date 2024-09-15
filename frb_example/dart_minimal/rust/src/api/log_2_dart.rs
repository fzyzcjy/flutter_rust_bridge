use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use log::{LevelFilter, Metadata, Record};

#[frb(non_opaque)]
pub struct Log2Dart {
    // TODO change String into Record
    stream_sink: StreamSink<String>,
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
// use custom type translation to translate between log::LogLevel and Dart:logging::Level
pub fn initialize_log2dart(log_stream: StreamSink<String>, max_log_level: LevelFilter) {
    log::set_boxed_logger(Box::new(Log2Dart {
        stream_sink: log_stream,
    }))
    .map(|()| log::set_max_level(max_log_level))
    .expect("initialize_log2dart is called only once!")
}
// impl<T: Send + Sync> log::Log for Log2Dart {
impl log::Log for Log2Dart {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
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

// TODO enable after implementing custom translater
// pub fn change_log_level(level: LevelFilter) {
//     log::set_max_level(level);
// }

/// custom coders for log::LogLevel <-> Dart:logging::Level
#[frb(rust2dart(dart_type = "Level", dart_code = "fromLevelFilter({})"))]
pub fn encode_fancy_type(level: LevelFilter) -> u16 {
    match level {
        LevelFilter::Trace => 0,    //Level('ALL', 0),
        LevelFilter::Debug => 700,  // Level('CONFIG', 700);
        LevelFilter::Info => 800,   // Level('INFO', 800);
        LevelFilter::Warn => 900,   // Level('WARNING', 900);
        LevelFilter::Error => 1000, // Level('SEVERE', 1000); & Level('SHOUT', 1200);
        LevelFilter::Off => 2000,   // Level('OFF', 2000);
    }
}

#[frb(dart2rust(dart_type = "Level", dart_code = "{}.value"))]
pub fn decode_fancy_type(level: u16) -> LevelFilter {
    match level {
        // Level('ALL', 0);
        // Level('OFF', 2000);
        // Level('FINEST', 300);
        // Level('FINER', 400);
        // Level('FINE', 500);
        0..=500 => LevelFilter::Trace,
        // Level('CONFIG', 700);
        501..=700 => LevelFilter::Debug,
        // Level('INFO', 800);
        701..=800 => LevelFilter::Info,
        // Level('WARNING', 900);
        801..=900 => LevelFilter::Warn,
        // Level('SEVERE', 1000);
        // Level('SHOUT', 1200);
        901..2000 => LevelFilter::Error,
        // Level('OFF', 2000);
        2000.. => LevelFilter::Off,
    }
}
