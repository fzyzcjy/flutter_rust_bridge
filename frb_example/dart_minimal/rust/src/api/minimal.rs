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
#[frb(dart_code = "
import 'dart:io';
import 'package:logging/logging.dart';

static void _default_log_function(Log2DartLogRecord record) {
  print('${DateTime.now()} [${log_level_from_number(record.levelNumber)} @${record.rustLog? 'Rust' : 'Dart' }]: ${record.loggerName} \\n   ${record.message}');
}

static Logger init_logger(
    {String name = 'RootLogger', Level maxLoglevel = Level.INFO,
    Function(Log2DartLogRecord) custom_log_function = _default_log_function}) {

      String? env_log_level = Platform.environment['LOG_LEVEL'];
    if (env_log_level != null) {
      print(
          'Taking log level from env: ${env_log_level} instead of the one given by code: ${maxLoglevel}');
      maxLoglevel = _log_level_from_str(env_log_level!);
    }

    Log2DartLogRecord _toLog2DartLogRecord(LogRecord record) {
      return Log2DartLogRecord(
          levelNumber: record.level.value,
          message: record.message,
          loggerName: record.loggerName,
          rustLog: false,
          modulePath: null,
          fileName: null,
          lineNumber: null);
    }
  
  final logger = Logger(name);
  
  Logger.root.level = maxLoglevel;
  
  var stream = initializeLog2Dart(maxLogLevel: maxLoglevel);
  // logs from Rust
  stream.listen((record) {
    // custom_log_function(_toLogRecord(record));
    custom_log_function(record);
    });

  // logs from Dart
  Logger.root.onRecord.listen((record) {
    custom_log_function(_toLog2DartLogRecord(record));
  });

    return logger;
  }

static Level _log_level_from_str(String levelStr) {
    switch (levelStr.toUpperCase()) {
      case 'ALL':
        return Level.ALL;
      case 'FINEST':
        return Level.FINEST;
      case 'FINER':
        return Level.FINER;
      case 'FINE':
        return Level.FINE;
      case 'CONFIG':
        return Level.CONFIG;
      case 'INFO':
        return Level.INFO;
      case 'WARNING':
        return Level.WARNING;
      case 'SEVERE':
        return Level.SEVERE;
      case 'SHOUT':
        return Level.SHOUT;
      case 'OFF':
        return Level.OFF;
      default:
        print(
            'unknown LOG_LEVEL: ${levelStr}. For potential values see https://pub.dev/documentation/logging/latest/logging/Level-class.html');
        exit(1);
    }
  }

// convert from log level number to Dart package logging->Level
static Level log_level_from_number(int level) {
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
    .expect("initialize_log2dart is called only once!");

    // log panics
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        log::error!("{}", info);
        prev(info);
    }));
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

// needed so we can convert u16 <-> DartLoggingLevel <-> log::LogLevel
enum DartLoggingLevel {
    All = 0,
    Finest = 300,
    Finer = 400,
    Fine = 500,
    Config = 700,
    Info = 800,
    Warning = 900,
    Severe = 1000,
    Shout = 1200,
    Off = 2000,
}

impl From<u16> for DartLoggingLevel {
    fn from(value: u16) -> Self {
        match value {
            0..=299 => DartLoggingLevel::All,
            300..=399 => DartLoggingLevel::Finest,
            400..=499 => DartLoggingLevel::Finer,
            500..=699 => DartLoggingLevel::Fine,
            700..=799 => DartLoggingLevel::Config,
            800..=899 => DartLoggingLevel::Info,
            900..=999 => DartLoggingLevel::Warning,
            1000..=1199 => DartLoggingLevel::Severe,
            1200..=1999 => DartLoggingLevel::Shout,
            2000.. => DartLoggingLevel::Off,
        }
    }
}

impl From<LevelFilter> for DartLoggingLevel {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::Trace => DartLoggingLevel::All,
            // LevelFilter::Trace => DartLoggingLevel::Finest,
            // LevelFilter::Trace => DartLoggingLevel::Finer,
            // LevelFilter::Trace => DartLoggingLevel::Fine,
            LevelFilter::Debug => DartLoggingLevel::Config,
            LevelFilter::Info => DartLoggingLevel::Info,
            LevelFilter::Warn => DartLoggingLevel::Warning,
            LevelFilter::Error => DartLoggingLevel::Severe,
            // LevelFilter::Error => DartLoggingLevel::Shout,
            LevelFilter::Off => DartLoggingLevel::Off,
        }
    }
}
impl From<DartLoggingLevel> for LevelFilter {
    fn from(level: DartLoggingLevel) -> Self {
        match level {
            DartLoggingLevel::All => LevelFilter::Trace,
            DartLoggingLevel::Finest => LevelFilter::Trace,
            DartLoggingLevel::Finer => LevelFilter::Trace,
            DartLoggingLevel::Fine => LevelFilter::Trace,
            DartLoggingLevel::Config => LevelFilter::Debug,
            DartLoggingLevel::Info => LevelFilter::Info,
            DartLoggingLevel::Warning => LevelFilter::Warn,
            DartLoggingLevel::Severe => LevelFilter::Error,
            DartLoggingLevel::Shout => LevelFilter::Error,
            DartLoggingLevel::Off => LevelFilter::Off,
        }
    }
}

/// custom coders for log::LogLevel <-> Dart:logging::Level
#[frb(rust2dart(dart_type = "Level", dart_code = "FRBLogger.log_level_from_number({})"))]
pub fn encode_log_level_filter(level: LevelFilter) -> u16 {
    DartLoggingLevel::from(level) as u16
}
#[frb(dart2rust(dart_type = "Level", dart_code = "{}.value"))]
pub fn decode_log_level_filter(level_number: u16) -> LevelFilter {
    DartLoggingLevel::from(level_number).into()
}

// mapping log crate's [Record](https://docs.rs/log/latest/log/struct.Record.html) to dart's Logger LogRecord
// intermediary struct to avoid Record's lifetimes
// #[frb(opaque)]
pub struct Log2DartLogRecord {
    pub level_number: u16,   // log::Recod::Level, Dart::Logger::LogRecord::Level
    pub message: String, // log::Recod::args + line_number + file_name + module_path , Dart::Logger::LogRecord::message + object + error + stackTrace
    pub logger_name: String, // log::Recod::target, Dart::Logger::LogRecord::loggerName
    // pub time: String, // log::Recod::?, Dart::Logger::LogRecord::time --> omitted, as there is no time record in the log crate's Record.
    pub rust_log: bool,
    pub module_path: Option<String>,
    pub file_name: Option<String>,
    pub line_number: Option<u32>,
}

impl From<&Record<'_>> for Log2DartLogRecord {
    fn from(record: &Record) -> Self {
        Self {
            level_number: DartLoggingLevel::from(record.level().to_level_filter()) as u16,
            message: record.args().to_string(),
            logger_name: record.target().into(),
            rust_log: true,
            module_path: (record.module_path().or_else(|| record.module_path_static()))
                .map(|s| s.into()),
            file_name: (record.file().or_else(|| record.file_static())).map(|s| s.into()),
            line_number: record.line(),
        }
    }
}
