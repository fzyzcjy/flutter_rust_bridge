#[macro_export]
macro_rules! enable_frb_logging {
  () => {
    enable_frb_logging!(name = _default_logger_name(), maxLoglevel = _default_max_log_level(), customLogFunction = _default_log_fn);
  };
  (name = $RootLoggerName:expr) => {
      enable_frb_logging!(name = $RootLoggerName, maxLoglevel = _default_max_log_level(), customLogFunction = _default_log_fn);
  };
  (maxLoglevel = $maxLogLevel:expr) => {
      enable_frb_logging!(name = _default_logger_name(), maxLoglevel = $maxLogLevel, customLogFunction = _default_log_fn);
  };
  (customLogFunction = $log_fn:expr) => {
      enable_frb_logging!(name = _default_logger_name(), maxLoglevel = _default_max_log_level(), customLogFunction = $log_fn);
  };
  (name = $RootLoggerName:expr, maxLoglevel = $maxLogLevel:expr) => {
      enable_frb_logging!(name = $RootLoggerName, maxLoglevel = $maxLogLevel, customLogFunction = _default_log_fn);
  };
  (name = $RootLoggerName:expr, customLogFunction = $log_fn:expr) => {
      enable_frb_logging!(name = $RootLoggerName, maxLoglevel = _default_log_fn(), customLogFunction = $log_fn);
  };
  (maxLoglevel = $maxLogLevel:expr, customLogFunction = $log_fn:expr) => {
      enable_frb_logging!(name = _default_logger_name(), maxLoglevel = $maxLogLevel, customLogFunction = $log_fn);
  };
  (name = $RootLoggerName:expr, maxLoglevel = $maxLogLevel:expr, customLogFunction = $log_fn:expr) => {

    fn _default_log_fn (record: Log2DartLogRecord) {
      let timestamp = chrono::Local::now();
      let max_log_level = from_u16(record.level_number);
      let lang = if record.rust_log {"Rust"} else {"Dart"};
      let logger_name = record.logger_name;
      let message = record.message;
      println!("[{timestamp:?} {max_log_level} @{lang} {logger_name}] {message})");
    }
    fn _default_logger_name () -> String {
      "FRBLogger".to_string()
    }
    fn _default_max_log_level  () -> String {
      "INFO".to_string()
    }
    #[flutter_rust_bridge::frb(sync)]
    pub fn root_logger_name() -> String {
      $RootLoggerName.to_string()
    }
    #[flutter_rust_bridge::frb(sync)]
    pub fn max_log_level() -> String {
      $maxLogLevel.to_string()
    }
    #[flutter_rust_bridge::frb(sync)]
    pub fn log_fn(record: Log2DartLogRecord) {
      ($log_fn(record));
    }

    #[flutter_rust_bridge::frb(dart_code = "
      import 'dart:io';

      import 'package:logging/logging.dart';

      /// initialize the logging system, including the rust logger
      static Logger initLogger(
      {String name = 'FRBLogger', String maxLogLevel = 'INFO',
      Function({required Log2DartLogRecord record}) customLogFunction =
          logFn}) {
        String? env_log_level = Platform.environment['LOG_LEVEL'];
        if (env_log_level != null) {
          print(
            'Taking log level from env: ${env_log_level} instead of the one given by code: ${maxLogLevel}');
          maxLogLevel = env_log_level!;
        }

        Log2DartLogRecord _toLog2DartLogRecord(LogRecord record) {
          return Log2DartLogRecord(
            levelNumber: record.level.value,
            message: record.message,
            loggerName: record.loggerName,
            rustLog: false,
            modulePath: null,
            fileName: null,
            lineNumber: null
          );
        }
  
        final logger = Logger(name);
  
        Logger.root.level = _logLevelFromStr(maxLogLevel);
  
        var stream = initializeLog2Dart(maxLogLevel: Logger.root.level.value);
        // logs from Rust
        stream.listen((record) {
          customLogFunction(record: record);
        });

        // logs from Dart
        Logger.root.onRecord.listen((record) {
          customLogFunction(record: _toLog2DartLogRecord(record));
        });

        return logger;
      }

      /// get a new named logger, can be used for getting the inital logger instead initLogger()
      static Logger getLogger([String? name]) {
        var loggerName = name ?? rootLoggerName();
        if (Logger.attachedLoggers.isEmpty) {
           initLogger(
            name: loggerName,
            maxLogLevel: maxLogLevel(),
            customLogFunction: logFn);
          }
        return Logger(loggerName);
      }

      static Level _logLevelFromStr(String levelStr) {
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
      static Level logLevelFromNumber(int level) {
        switch (level) {
          case < 300: 
            return Level.ALL;
          case < 400:
            return Level.FINEST;
          case < 500:
            return Level.FINER;
          case < 700:
            return Level.FINE;
          case < 800:
            return Level.CONFIG;
          case < 900: 
            return Level.INFO;
          case < 1000:
            return Level.WARNING;
          case < 1200:
            return Level.SEVERE;
          case < 2000:
            return Level.SHOUT;
          case >= 2000:
            return Level.OFF;
          default:
            return Level.ALL;
        }  
      }
    ")]
    pub struct FRBLogger {
      #[allow(clippy::crate_in_macro_def)]
      pub stream_sink: crate::frb_generated::StreamSink<Log2DartLogRecord>,
    }

    impl FRBLogger {
      #[allow(clippy::new_without_default)]
      pub fn new() -> FRBLogger {
        panic!("Initialize with `final LOGGER = FRBLogger.getLogger();` or `final LOGGER = FRBLogger.initLogger();`");
      }
    }
    /// usees custom type translation to translate between log::LogLevel and Dart:logging::Level
    /// loglevel is represented by a number, so that we don't need to put \import `import 'package:logging/logging.dart';`
    /// into the dart preamble in flutter_rust_bridge.yaml
    pub fn initialize_log2dart(log_stream: crate::frb_generated::StreamSink<Log2DartLogRecord>, max_log_level: u16) {
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
      fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
      }

      fn log(&self, record: &log::Record) {
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

    fn from_u16(level: u16) -> log::LevelFilter {
      match level{
        // Level('ALL', 0);
        // Level('OFF', 2000);
        // Level('FINEST', 300);
        // Level('FINER', 400);
        // Level('FINE', 500);
        0..=500 => log::LevelFilter::Trace,
        // Level('CONFIG', 700);
        501..=700 => log::LevelFilter::Debug,
        // Level('INFO', 800);
        701..=800 => log::LevelFilter::Info,
        // Level('WARNING', 900);
        801..=900 => log::LevelFilter::Warn,
        // Level('SEVERE', 1000);
        // Level('SHOUT', 1200);
        901..2000 => log::LevelFilter::Error,
        // Level('OFF', 2000);
        2000.. => log::LevelFilter::Off,
      }
    }

    fn to_u16(value: log::LevelFilter) -> u16 {
      match value {
        // ALL → value = 0
        log::LevelFilter::Trace => 0,
        // FINEST → value = 300.
        // FINER → value = 400.
        // FINE → value = 500.
        // CONFIG → value = 700.
        log::LevelFilter::Debug => 700,
        // INFO → value = 800.
        log::LevelFilter::Info => 800,
        // WARNING → value = 900.
        log::LevelFilter::Warn => 900,
        // SEVERE → value = 1000.
        log::LevelFilter::Error => 1000,
        // SHOUT → value = 1200.
        // OFF → value = 2000.
        log::LevelFilter::Off => 2000,
      }
    }

    /// custom coders for log::LogLevel <-> Dart:logging::Level
    #[frb(rust2dart(dart_type = "Level", dart_code = "FRBLogger.logLevelFromNumber({})"))]
    pub fn encode_log_level_filter(level: log::LevelFilter) -> u16 {
      to_u16(level)
    }
    #[frb(dart2rust(dart_type = "Level", dart_code = "{}.value"))]
    pub fn decode_log_level_filter(level_number: u16) -> log::LevelFilter {
      from_u16(level_number)
    }

    /// mapping log crate's [Record](https://docs.rs/log/latest/log/struct.Record.html) to dart's Logger [LogRecord](https://pub.dev/documentation/logging/latest/logging/LogRecord-class.html).
    /// intermediary struct to avoid Record's lifetimes
    pub struct Log2DartLogRecord {
      pub level_number: u16,   // The log level encoded. Decode with `FRBLogger.logLevelFromNumber(x)` in Dart or `from_u16(x) in Rust. : Rust::log::Recod::Level, Dart::Logger::LogRecord::Level
      pub message: String, // The String given to the log statement: Rust::log::Recod::args, Dart::Logger::LogRecord::message
      pub logger_name: String, // The name of the logger given by `FRBLogger.initLogger(name: "MyClass");`, Rust::log::Recod::target, Dart::Logger::LogRecord::loggerName
      // pub time: String, // log::Recod::?, Dart::Logger::LogRecord::time --> omitted, as there is no time record in the log crate's Record
      pub rust_log: bool, // true, if the log statement originates from Rust code
      pub module_path: Option<String>, // Rust::log::Recod::module_path, None for Dart
      pub file_name: Option<String>, // Rust::log::Recod::file_name, None for Dart
      pub line_number: Option<u32>, // Rust::log::Recod::line_number, None for Dart
    }

    impl From<&log::Record<'_>> for Log2DartLogRecord {
      fn from(record: &log::Record) -> Self {
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
  };
}
