#[macro_export]
macro_rules! enable_frb_logging {
    () => {
use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb as frb_4_log;
pub use log::{LevelFilter, Metadata, Record};

#[frb(dart_code = "
import 'dart:io';
import 'package:logging/logging.dart';

static void default_log_function(Log2DartLogRecord record) {
  print('${DateTime.now()} [${log_level_from_number(record.levelNumber)} @${record.rustLog? 'Rust' : 'Dart' }]: ${record.loggerName} \\n   ${record.message}');
}

/// initialize the logging system, including the rust logger
static Logger init_logger(
    {String name = 'RootLogger', String maxLoglevel = 'INFO',
    Function(Log2DartLogRecord) custom_log_function = default_log_function}) {

      String? env_log_level = Platform.environment['LOG_LEVEL'];
    if (env_log_level != null) {
      print(
          'Taking log level from env: ${env_log_level} instead of the one given by code: ${maxLoglevel}');
      maxLoglevel = env_log_level!;
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
  
  Logger.root.level = _log_level_from_str(maxLoglevel);
  
  var stream = initializeLog2Dart(maxLogLevel: Logger.root.level.value);
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

  /// get a new named logger after getting the inital logger with init_logger()
  static Logger getLogger(String name) {
      if (Logger.attachedLoggers.isEmpty) {
        throw Exception('FRBLogger.initLogging() must be called first');
    }
    return Logger(name);
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
/// usees custom type translation to translate between log::LogLevel and Dart:logging::Level
/// loglevel is represented by a number, so that we don't need to put \import `import 'package:logging/logging.dart';`
/// into the dart preamble in flutter_rust_bridge.yaml
pub fn initialize_log2dart(log_stream: StreamSink<Log2DartLogRecord>, max_log_level: u16) {
    log::set_boxed_logger(Box::new(FRBLogger {
        stream_sink: log_stream,
    }))
    .map(|()| log::set_max_level(from_u16(max_log_level)))
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

fn from_u16(value: u16) -> LevelFilter {
    match value {
        // 0..=299 => LevelFilter::All,
        // 300..=399 => LevelFilter::Finest,
        // 400..=499 => LevelFilter::Finer,
        // 500..=699 => LevelFilter::Fine,
        0..=699 => LevelFilter::Trace,
        300..=799 => LevelFilter::Debug,
        800..=899 => LevelFilter::Info,
        900..=999 => LevelFilter::Warn,
        // 1200..=1999 => LevelFilter::Shout,
        1000..=1999 => LevelFilter::Error,
        2000.. => LevelFilter::Off,
    }
}

fn to_u16(value: LevelFilter) -> u16 {
    match value {
        // 0..=299 => LevelFilter::All,
        // 300..=399 => LevelFilter::Finest,
        // 400..=499 => LevelFilter::Finer,
        // 500..=699 => LevelFilter::Fine,
        LevelFilter::Trace => 0,
        LevelFilter::Debug => 500,
        LevelFilter::Info => 800,
        LevelFilter::Warn => 900,
        // 1200..=1999 => LevelFilter::Shout,
        LevelFilter::Error => 1000,
        LevelFilter::Off => 2000,
    }
}

/// custom coders for log::LogLevel <-> Dart:logging::Level
#[frb(rust2dart(dart_type = "Level", dart_code = "FRBLogger.log_level_from_number({})"))]
pub fn encode_log_level_filter(level: LevelFilter) -> u16 {
    to_u16(level)
}
#[frb(dart2rust(dart_type = "Level", dart_code = "{}.value"))]
pub fn decode_log_level_filter(level_number: u16) -> LevelFilter {
    from_u16(level_number)
}

/// mapping log crate's [Record](https://docs.rs/log/latest/log/struct.Record.html) to dart's Logger [LogRecord](https://pub.dev/documentation/logging/latest/logging/LogRecord-class.html).
/// intermediary struct to avoid Record's lifetimes
pub struct Log2DartLogRecord {
    pub level_number: u16,   // The log level encoded. Decode with `FRBLogger.log_level_from_number(x)` : Rust::log::Recod::Level, Dart::Logger::LogRecord::Level
    pub message: String, // The String given to the log statement: Rust::log::Recod::args, Dart::Logger::LogRecord::message
    pub logger_name: String, // The name of the logger given by `FRBLogger.init_logger(name: "MyClass");`, Rust::log::Recod::target, Dart::Logger::LogRecord::loggerName
    // pub time: String, // log::Recod::?, Dart::Logger::LogRecord::time --> omitted, as there is no time record in the log crate's Record
    pub rust_log: bool, // true, if the log statement originates from Rust code
    pub module_path: Option<String>, // Rust::log::Recod::module_path, None for Dart
    pub file_name: Option<String>, // Rust::log::Recod::file_name, None for Dart
    pub line_number: Option<u32>, // Rust::log::Recod::line_number, None for Dart
}

impl From<&Record<'_>> for Log2DartLogRecord {
    fn from(record: &Record) -> Self {
        Self {
            level_number: to_u16(record.level().to_level_filter()),
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
}
}
