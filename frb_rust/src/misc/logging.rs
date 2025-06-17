#[allow(clippy::crate_in_macro_def)]
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

    fn _default_log_fn (record: MirLogRecord) {
      println!("{}", _construct_default_message(record));
    }
    fn _construct_default_message(record: MirLogRecord) -> String {
      let timestamp = chrono::Local::now();
      let max_log_level = from_u16(record.level_number);
      let lang = if record.rust_log {"Rust"} else {"Dart"};
      let logger_name = record.logger_name;
      let message = record.message;
      format!("[{timestamp:?} {max_log_level} @{lang} {logger_name}] {message})")
    }

    fn _default_logger_name () -> String {
      "FRBLogger".to_string()
    }
    fn _default_max_log_level  () -> String {
      "INFO".to_string()
    }

    #[flutter_rust_bridge::frb(sync)]
    #[allow(dead_code)] // used by generated dart code
    pub fn root_logger_name() -> String {
      $RootLoggerName.to_string()
    }
    #[flutter_rust_bridge::frb(sync)]
    #[allow(dead_code)] // used by generated dart code
    pub fn max_log_level() -> String {
      $maxLogLevel.to_string()
    }

    /// this is the call for logging (from Rust and Dart (as logFn))
    #[flutter_rust_bridge::frb(sync)]
    #[allow(dead_code)] // used by generated dart code
    pub fn log_fn(record: MirLogRecord) {
      ($log_fn(record));
    }

    use flutter_rust_bridge::frb;
    use crate::__FrbStreamSinkForLogging as StreamSink;
    use $crate::for_generated::chrono;

    #[flutter_rust_bridge::frb(dart_code = "
    import 'package:logging/logging.dart';

    static FRBDartLogger initLogger(
      {String name = 'FRBLogger',
      LogLevel maxLogLevel = LogLevel.info,
      Function({required MirLogRecord record}) customLogFunction = logFn}) {
        //initialize the rust side
        int maxLogLevelNumber = maxLogLevel.levelNumberThreshold;
        Stream<MirLogRecord> stream =
            initializeLog2Dart(maxLogLevel: maxLogLevelNumber);

        // Functions for type conversion for interaction with frb_dart/utils/frb_logging.dart
        // Wrap logFn to match `void Function({required dynamic record})`
        void Function({required dynamic record}) wrappedLogFn =
            ({required dynamic record}) {
          // Safely cast `dynamic` record back to `MirLogRecord` for the original `logFn`
          logFn(record: record as MirLogRecord);
        };
        // Wrap fromDartLogRecord to match `dynamic Function(LogRecord record)`
        MirLogRecord Function(LogRecord record) wrappedFromDartLogRecord =
            (LogRecord record) {
          return MirLogRecord.fromDartLogRecord(record);
        };
        // Wrap customLogFunction if provided, to match `void Function({required dynamic record})?`
        void Function({required dynamic record})? wrappedCustomLogFunction;
        wrappedCustomLogFunction = ({required dynamic record}) {
          customLogFunction(record: record as MirLogRecord);
        };

        return FRBDartLogger.initAndGetSingleton<MirLogRecord>(
          streamSink: stream,
          name: name,
          logFn: wrappedLogFn,
          fromDartLogRecord: wrappedFromDartLogRecord,
          maxLogLevel: maxLogLevel,
          customLogFunction: wrappedCustomLogFunction,
        );
      }
    ")]
    pub struct FRBLogger {
      #[allow(clippy::crate_in_macro_def)]
      pub stream_sink: StreamSink<MirLogRecord>,
    }

    impl FRBLogger {
      #[allow(clippy::new_without_default)]
      #[allow(dead_code)] // needed for frb_code generation, even though it shouldn't be used!
      pub fn new() -> FRBLogger {
        panic!("Initialize with `final LOGGER = FRBLogger.getLogger();` or `final LOGGER = FRBLogger.initLogger();`");
      }
    }
    /// uses custom type translation to translate between log::LogLevel and Dart:logging::Level
    /// loglevel is represented by a number, so that we don't need to put \import `import 'package:logging/logging.dart';`
    /// into the dart preamble in flutter_rust_bridge.yaml
    #[allow(dead_code)] // used by generated dart code
    pub fn initialize_log_2_dart(log_stream: StreamSink<MirLogRecord>, max_log_level: u16) {
      log::set_boxed_logger(Box::new(FRBLogger {
        stream_sink: log_stream,
      }))
      .map(|()| log::set_max_level(from_u16(max_log_level)))
      .expect("initialize_log_2_dart is called only once!");

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
    #[frb(rust2dart(dart_type = "Level", dart_code = "LogLevel.fromNumber({}).toLoggingLevel()"))]
    #[allow(dead_code)] // used by generated dart code
    pub fn encode_log_level_filter(level: log::LevelFilter) -> u16 {
      to_u16(level)
    }
    #[frb(dart2rust(dart_type = "Level", dart_code = "{}.value"))]
    #[allow(dead_code)] // used by generated dart code
    pub fn decode_log_level_filter(level_number: u16) -> log::LevelFilter {
      from_u16(level_number)
    }

    /// mapping log crate's [Record](https://docs.rs/log/latest/log/struct.Record.html) to dart's Logger [LogRecord](https://pub.dev/documentation/logging/latest/logging/LogRecord-class.html).
    /// intermediary struct to avoid Record's lifetimes
    #[derive(Clone)]
    #[flutter_rust_bridge::frb(dart_code = "
      static MirLogRecord fromDartLogRecord(LogRecord record) {
        return MirLogRecord(
          message: record.message,
          levelNumber: record.level.value,
          loggerName: record.loggerName,
          rustLog: false,
        );
      }
      static LogRecord toDartLogRecordFromMir(MirLogRecord record) {
        return LogRecord(
          LogLevel.fromNumber(record.levelNumber).toLoggingLevel(),
          record.message,
          record.loggerName,
        );
      }
      LogRecord toDartLogRecord() {
        return toDartLogRecordFromMir(this);
      }
    ")]
    pub struct MirLogRecord {
      pub level_number: u16,   // The log level encoded. Decode with `LogLevel.fromNumber(x).toLoggingLevel()` in Dart or `from_u16(x) in Rust. : Rust::log::Recod::Level, Dart::Logger::LogRecord::Level
      pub message: String, // The String given to the log statement: Rust::log::Recod::args, Dart::Logger::LogRecord::message
      pub logger_name: String, // The name of the logger given by `FRBLogger.initLogger(name: "MyClass");`, Rust::log::Recod::target, Dart::Logger::LogRecord::loggerName
      // pub time: String, // log::Recod::?, Dart::Logger::LogRecord::time --> omitted, as there is no time record in the log crate's Record
      pub rust_log: bool, // true, if the log statement originates from Rust code
      #[allow(dead_code)] // not used, but there for completeness. A custom log function might want to use this.
      pub module_path: Option<String>, // Rust::log::Recod::module_path, None for Dart
      #[allow(dead_code)] // not used, but there for completeness. A custom log function might want to use this.
      pub file_name: Option<String>, // Rust::log::Recod::file_name, None for Dart
      #[allow(dead_code)] // not used, but there for completeness. A custom log function might want to use this.
      pub line_number: Option<u32>, // Rust::log::Recod::line_number, None for Dart
    }

    impl From<&log::Record<'_>> for MirLogRecord {
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

#[cfg(test)]
pub(crate) mod test {
    use flutter_rust_bridge_macros as flutter_rust_bridge;
    use log::{Level, Log, Metadata, Record};
    use std::sync::{Arc, Mutex};

    // A simple mock for StreamSink for testing
    #[derive(Debug, Clone)]
    pub struct MockStreamSink<T> {
        pub sent_records: Arc<Mutex<Vec<T>>>,
    }

    impl<T> MockStreamSink<T> {
        pub fn new() -> Self {
            Self {
                sent_records: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn add(&self, item: T) -> Result<(), crate::Rust2DartSendError> {
            self.sent_records.lock().unwrap().push(item);
            Ok(())
        }
    }

    #[test]
    fn test_default_logger_name() {
        enable_frb_logging!();
        assert_eq!(_default_logger_name(), "FRBLogger".to_string());
    }

    #[test]
    fn test_default_max_log_level() {
        enable_frb_logging!();
        assert_eq!(_default_max_log_level(), "INFO".to_string());
    }

    #[test]
    fn test_default_log_fn() {
        enable_frb_logging!();

        let record = MirLogRecord {
            level_number: to_u16(log::LevelFilter::Debug),
            message: "Test message".to_string(),
            logger_name: "TestLogger".to_string(),
            rust_log: true,
            module_path: Some("test_module".to_string()),
            file_name: Some("test_file.rs".to_string()),
            line_number: Some(123),
        };

        assert!(_construct_default_message(record).contains("DEBUG @Rust TestLogger] Test message"));
    }

    #[test]
    #[should_panic(
        expected = "Initialize with `final LOGGER = FRBLogger.getLogger();` or `final LOGGER = FRBLogger.initLogger();`"
    )]
    fn test_frb_logger_new_panics() {
        enable_frb_logging!();
        FRBLogger::new();
    }

    #[test]
    fn test_initialize_log_2_dart() {
        enable_frb_logging!();

        // Reset the logger and panic hook for a clean test
        // This is crucial because `set_boxed_logger` can only be called once.
        // In a real application, you might use a `once_cell` or similar to ensure this.
        // For testing, we might need to "reset" the global logger state if possible,
        // or ensure tests run in isolation (e.g., with `cargo test -- --test-threads=1`).
        // For simplicity here, we'll try to catch a potential panic if already set.
        let stream_sink = MockStreamSink::new();
        let max_log_level = to_u16(log::LevelFilter::Info);

        let result = std::panic::catch_unwind(|| {
            initialize_log_2_dart(stream_sink, max_log_level);
        });

        if result.is_err() {
            if let Err(err) = result {
                if let Some(msg) = err.downcast_ref::<&str>() {
                    if msg.contains("initialize_log_2_dart is called only once!") {
                        // This is expected if another test already initialized it.
                        // To truly test this, you'd need to ensure test isolation
                        // or use a different testing approach that doesn't rely on global state.
                        eprintln!("Warning: initialize_log_2_dart already called in another test. This test might not fully cover the `expect` path.");
                    } else {
                        panic!("Unexpected panic: {err:?}");
                    }
                } else {
                    panic!("Unexpected panic type: {err:?}");
                }
            }
        }
    }

    #[test]
    fn test_frb_logger_enabled() {
        enable_frb_logging!();
        let stream_sink = MockStreamSink::new();
        let logger = FRBLogger { stream_sink };

        // Temporarily set max log level for this test
        // NOTE: This modifies global state, which can lead to flaky tests if not carefully managed.
        // In a production setup, consider using a logging facade that allows for logger replacement
        // or ensure tests run sequentially.
        let original_max_level = log::max_level();
        log::set_max_level(log::LevelFilter::Info);

        let metadata_info = Metadata::builder()
            .level(Level::Info)
            .target("test")
            .build();
        let metadata_debug = Metadata::builder()
            .level(Level::Debug)
            .target("test")
            .build();
        let metadata_error = Metadata::builder()
            .level(Level::Error)
            .target("test")
            .build();

        assert!(logger.enabled(&metadata_info));
        assert!(!logger.enabled(&metadata_debug)); // Debug < Info
        assert!(logger.enabled(&metadata_error)); // Error > Info

        log::set_max_level(original_max_level); // Restore original level
    }

    #[test]
    fn test_frb_logger_log() {
        enable_frb_logging!();
        let logger = FRBLogger {
            stream_sink: MockStreamSink::new(),
        };

        // Temporarily set max log level for this test
        let original_max_level = log::max_level();
        log::set_max_level(log::LevelFilter::Trace); // Enable all for logging

        let record_info = Record::builder()
            .level(Level::Info)
            .target("test_target")
            .args(format_args!("Test message info"))
            .module_path(Some("test_module"))
            .file(Some("test_file.rs"))
            .line(Some(10))
            .build();

        let record_debug = Record::builder()
            .level(Level::Debug)
            .target("test_target_debug")
            .args(format_args!("Test message debug"))
            .build();

        logger.log(&record_info);
        logger.log(&record_debug);

        let sent_records = logger.stream_sink.sent_records.lock().unwrap();
        assert_eq!(sent_records.len(), 2);

        let record1 = &sent_records[0];
        assert_eq!(record1.level_number, to_u16(log::LevelFilter::Info));
        assert_eq!(record1.message, "Test message info");
        assert_eq!(record1.logger_name, "test_target");
        assert!(record1.rust_log);
        assert_eq!(record1.module_path, Some("test_module".to_string()));
        assert_eq!(record1.file_name, Some("test_file.rs".to_string()));
        assert_eq!(record1.line_number, Some(10));

        let record2 = &sent_records[1];
        assert_eq!(record2.level_number, to_u16(log::LevelFilter::Debug));
        assert_eq!(record2.message, "Test message debug");

        log::set_max_level(original_max_level); // Restore original level
    }

    #[test]
    fn test_frb_logger_flush() {
        enable_frb_logging!();
        let stream_sink = MockStreamSink::new();
        let logger = FRBLogger { stream_sink };
        // This method does nothing, so we just call it to ensure it doesn't panic.
        logger.flush();
    }

    #[test]
    fn test_from_u16_trace() {
        enable_frb_logging!();
        assert_eq!(from_u16(0), log::LevelFilter::Trace);
        assert_eq!(from_u16(300), log::LevelFilter::Trace);
        assert_eq!(from_u16(500), log::LevelFilter::Trace);
    }

    #[test]
    fn test_from_u16_debug() {
        enable_frb_logging!();
        assert_eq!(from_u16(501), log::LevelFilter::Debug);
        assert_eq!(from_u16(700), log::LevelFilter::Debug);
    }

    #[test]
    fn test_from_u16_info() {
        enable_frb_logging!();
        assert_eq!(from_u16(701), log::LevelFilter::Info);
        assert_eq!(from_u16(800), log::LevelFilter::Info);
    }

    #[test]
    fn test_from_u16_warn() {
        enable_frb_logging!();
        assert_eq!(from_u16(801), log::LevelFilter::Warn);
        assert_eq!(from_u16(900), log::LevelFilter::Warn);
    }

    #[test]
    fn test_from_u16_error() {
        enable_frb_logging!();
        assert_eq!(from_u16(901), log::LevelFilter::Error);
        assert_eq!(from_u16(1500), log::LevelFilter::Error);
        assert_eq!(from_u16(1999), log::LevelFilter::Error);
    }

    #[test]
    fn test_from_u16_off() {
        enable_frb_logging!();
        assert_eq!(from_u16(2000), log::LevelFilter::Off);
        assert_eq!(from_u16(3000), log::LevelFilter::Off);
    }

    #[test]
    fn test_to_u16_trace() {
        enable_frb_logging!();
        assert_eq!(to_u16(log::LevelFilter::Trace), 0);
    }

    #[test]
    fn test_to_u16_debug() {
        enable_frb_logging!();
        assert_eq!(to_u16(log::LevelFilter::Debug), 700);
    }

    #[test]
    fn test_to_u16_info() {
        enable_frb_logging!();
        assert_eq!(to_u16(log::LevelFilter::Info), 800);
    }

    #[test]
    fn test_to_u16_warn() {
        enable_frb_logging!();
        assert_eq!(to_u16(log::LevelFilter::Warn), 900);
    }

    #[test]
    fn test_to_u16_error() {
        enable_frb_logging!();
        assert_eq!(to_u16(log::LevelFilter::Error), 1000);
    }

    #[test]
    fn test_to_u16_off() {
        enable_frb_logging!();
        assert_eq!(to_u16(log::LevelFilter::Off), 2000);
    }

    #[test]
    fn test_mir_log_record_from_log_record() {
        enable_frb_logging!();
        let log_record = Record::builder()
            .level(Level::Info)
            .target("my_target")
            .args(format_args!("Hello, world!"))
            .module_path(Some("my_module"))
            .file(Some("my_file.rs"))
            .line(Some(42))
            .build();

        let frb_record: MirLogRecord = (&log_record).into();

        assert_eq!(frb_record.level_number, to_u16(log::LevelFilter::Info));
        assert_eq!(frb_record.message, "Hello, world!");
        assert_eq!(frb_record.logger_name, "my_target");
        assert!(frb_record.rust_log);
        assert_eq!(frb_record.module_path, Some("my_module".to_string()));
        assert_eq!(frb_record.file_name, Some("my_file.rs".to_string()));
        assert_eq!(frb_record.line_number, Some(42));
    }

    #[test]
    fn test_mir_log_record_from_log_record_no_optional_fields() {
        enable_frb_logging!();
        let log_record = Record::builder()
            .level(Level::Warn)
            .target("another_target")
            .args(format_args!("Simple message"))
            .build();

        let frb_record: MirLogRecord = (&log_record).into();

        assert_eq!(frb_record.level_number, to_u16(log::LevelFilter::Warn));
        assert_eq!(frb_record.message, "Simple message");
        assert_eq!(frb_record.logger_name, "another_target");
        assert!(frb_record.rust_log);
        assert_eq!(frb_record.module_path, None);
        assert_eq!(frb_record.file_name, None);
        assert_eq!(frb_record.line_number, None);
    }
}
