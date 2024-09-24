// TODO replace with re-export
// use flutter_rust_bridge::log_2_dart::log;
use flutter_rust_bridge::frb;
use log;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    log::info!("adding {} and {}", a, b);
    a + b
}

///////////////////
// TODO move code to macro
use crate::frb_generated::StreamSink;
// use flutter_rust_bridge::frb;
pub use log::{LevelFilter, Metadata, Record};

// #[frb(non_opaque)]
#[frb]
#[frb(dart_code = "
import 'package:logging/logging.dart';

static void _default_log_function(LogRecord record) {
  print('${record.level}:${record.loggerName}: ${record.message}');
}

static Logger init_logger(
    {String name = 'RootLogger', Level maxLoglevel = Level.INFO,
    Function(LogRecord) custom_log_function = _default_log_function}) {

   LogRecord _toLogRecord(Log2DartLogRecord record) {
    return LogRecord(
      record.level,
      record.message,
      record.loggerName,
    );
  }

  final logger = Logger(name);
  
  Logger.root.level = maxLoglevel;

  var stream = initializeLog2Dart(maxLogLevel: maxLoglevel);
  // logs from Rust
  stream.listen((record) {
    custom_log_function(_toLogRecord(record));
  });

  // logs from Dart
  Logger.root.onRecord.listen((record) {
    custom_log_function(record);
  });

    return logger;
  }


// convert from log crate's LevelFilter to Dart package logging->Level
static Level fromLevelFilter(int level) {
  switch (level) {
    case <= 500:
      return Level.ALL;
    case <= 700:
      return Level.CONFIG;
    case <= 800:
      return Level.INFO;
    case <= 900:
      return Level.WARNING;
    case < 2000:
      return Level.SEVERE;
    case >= 2000:
      return Level.OFF;
    default:
      return Level.ALL;
  }
}
")]
pub struct FRBLogger {
    pub stream_sink: StreamSink<Log2DartLogRecord>,
}

impl FRBLogger {
    pub fn new() -> FRBLogger {
        panic!("Initialize with `final LOGGER = FRBLogger.init_logger();`");
    }
}
// usees custom type translation to translate between log::LogLevel and Dart:logging::Level
pub fn initialize_log2dart(log_stream: StreamSink<Log2DartLogRecord>, max_log_level: LevelFilter) {
    log::set_boxed_logger(Box::new(FRBLogger {
        stream_sink: log_stream,
    }))
    .map(|()| log::set_max_level(max_log_level))
    .expect("initialize_log2dart is called only once!")
}

impl log::Log for FRBLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.stream_sink
                .add(record.into())
                .expect("could not add to stream while sending to dart ");
        }
    }

    fn flush(&self) {
        //no need to flush the StreamSink
    }
}

/// custom coders for log::LogLevel <-> Dart:logging::Level
#[frb(rust2dart(dart_type = "Level", dart_code = "FRBLogger.fromLevelFilter({})"))]
pub fn encode_log_level_filter(level: LevelFilter) -> u16 {
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
pub fn decode_log_level_filter(level: u16) -> LevelFilter {
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
        901..=1999 => LevelFilter::Error,
        // Level('OFF', 2000);
        2000.. => LevelFilter::Off,
    }
}

// mapping log crate's [Record](https://docs.rs/log/latest/log/struct.Record.html) to dart's Logger LogRecord
// intermediary struct to avoid Record's lifetimes
#[frb(opaque)]
pub struct Log2DartLogRecord {
    pub level: LevelFilter, // log::Recod::Level, Dart::Logger::LogRecord::Level
    pub message: String, // log::Recod::args + line_number + file_name + module_path , Dart::Logger::LogRecord::message + object + error + stackTrace
    pub logger_name: String, // log::Recod::target, Dart::Logger::LogRecord::loggerName
                         // pub time: String, // log::Recod::?, Dart::Logger::LogRecord::time --> omitted, as there is no time record in the log crate's Record.
}

impl From<&Record<'_>> for Log2DartLogRecord {
    fn from(record: &Record) -> Self {
        let mut message = String::new();

        if let Some(module_path) = record.module_path().or_else(|| record.module_path_static()) {
            message.push_str("in module ");
            message.push_str(module_path);
            message.push(' ');
        }
        if let Some(file_name) = record.file().or_else(|| record.file_static()) {
            message.push_str("in file ");
            message.push_str(file_name);
            message.push(' ');
        }
        if let Some(line_number) = record.line() {
            message += &format!("in line {} ", line_number);
        }
        message += ": ";
        message += &record.args().to_string();

        Self {
            level: record.level().to_level_filter(),
            message,
            logger_name: record.target().into(),
        }
    }
}
