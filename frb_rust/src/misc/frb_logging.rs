#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! enable_frb_rust_to_dart_logging {
    () => {
        $crate::enable_frb_rust_to_dart_logging!(
            max_level = log::LevelFilter::Info,
            setup_dart_logging_output = true
        );
    };
    (max_level = $max_level:expr) => {
        $crate::enable_frb_rust_to_dart_logging!(
            max_level = $max_level,
            setup_dart_logging_output = true
        );
    };
    (setup_dart_logging_output = $setup_dart_logging_output:expr) => {
        $crate::enable_frb_rust_to_dart_logging!(
            max_level = log::LevelFilter::Info,
            setup_dart_logging_output = $setup_dart_logging_output
        );
    };
    (setup_dart_logging_output = $setup_dart_logging_output:expr, max_level = $max_level:expr) => {
        $crate::enable_frb_rust_to_dart_logging!(
            max_level = $max_level,
            setup_dart_logging_output = $setup_dart_logging_output
        );
    };
    (max_level = $max_level:expr, setup_dart_logging_output = $setup_dart_logging_output:expr) => {
        #[derive(Clone, Debug)]
        pub struct FrbLogRecord {
            pub level: String,
            pub message: String,
            pub target: String,
            pub module_path: Option<String>,
            pub file: Option<String>,
            pub line: Option<u32>,
        }

        struct FrbDartLogger {
            sink: std::sync::RwLock<
                Option<std::sync::Arc<crate::frb_generated::StreamSink<FrbLogRecord>>>,
            >,
        }

        impl log::Log for FrbDartLogger {
            fn enabled(&self, metadata: &log::Metadata) -> bool {
                metadata.level() <= log::max_level()
            }

            fn log(&self, record: &log::Record) {
                if !self.enabled(record.metadata()) {
                    return;
                }

                let Ok(sink) = self.sink.read().map(|sink| sink.clone()) else {
                    $crate::for_generated::print_to_console("FRB logging sink lock is poisoned");
                    return;
                };
                let Some(sink) = sink.as_ref() else {
                    frb_log_record_to_console(record);
                    return;
                };

                if let Err(error) = sink.add(FrbLogRecord {
                    level: record.level().to_string(),
                    message: record.args().to_string(),
                    target: record.target().to_owned(),
                    module_path: record.module_path().map(ToOwned::to_owned),
                    file: record.file().map(ToOwned::to_owned),
                    line: record.line(),
                }) {
                    $crate::for_generated::print_to_console(&format!(
                        "Failed to forward Rust log to Dart: {error:?}",
                    ));
                }
            }

            fn flush(&self) {}
        }

        fn frb_log_record_to_console(record: &log::Record) {
            $crate::for_generated::print_to_console(&format!(
                "{} - {}",
                record.level(),
                record.args()
            ));
        }

        #[doc(hidden)]
        #[flutter_rust_bridge::frb(init_dart_code = r#"
                    kFrbDartLogging.init(
                      rustLogStream: frbInternalInitLogger(maxLevel: frbInternalLoggingMaxLevel()),
                      mapRecord: (record) => FrbLogRecordData(
                        level: record.level,
                        message: record.message,
                        target: record.target,
                        modulePath: record.modulePath,
                        file: record.file,
                        line: record.line,
                      ),
                      setupDefaultOutput: frbInternalLoggingSetupDartLoggingOutput(),
                      disposeRustLogger: frbInternalDisposeLogger,
                    );
"#)]
        pub fn frb_internal_init_logger(
            sink: crate::frb_generated::StreamSink<FrbLogRecord>,
            max_level: String,
        ) {
            let max_level = frb_parse_logging_max_level(&max_level);
            let logger = FRB_DART_LOGGER.get_or_init(|| FrbDartLogger {
                sink: std::sync::RwLock::new(None),
            });

            let _ = log::set_logger(logger);
            log::set_max_level(max_level);

            *logger.sink.write().expect("FRB logger sink lock poisoned") =
                Some(std::sync::Arc::new(sink));
        }

        #[doc(hidden)]
        #[flutter_rust_bridge::frb(sync)]
        pub fn frb_internal_dispose_logger() {
            if let Some(logger) = FRB_DART_LOGGER.get() {
                *logger.sink.write().expect("FRB logger sink lock poisoned") = None;
            }
        }

        static FRB_DART_LOGGER: std::sync::OnceLock<FrbDartLogger> = std::sync::OnceLock::new();

        fn frb_parse_logging_max_level(max_level: &str) -> log::LevelFilter {
            match max_level.to_uppercase().as_str() {
                "OFF" => log::LevelFilter::Off,
                "ERROR" => log::LevelFilter::Error,
                "WARN" => log::LevelFilter::Warn,
                "INFO" => log::LevelFilter::Info,
                "DEBUG" => log::LevelFilter::Debug,
                "TRACE" => log::LevelFilter::Trace,
                _ => panic!("Unknown FRB logging max level: {max_level}"),
            }
        }

        #[doc(hidden)]
        #[flutter_rust_bridge::frb(sync)]
        pub fn frb_internal_logging_max_level() -> String {
            $max_level.to_string()
        }

        #[doc(hidden)]
        #[flutter_rust_bridge::frb(sync)]
        pub fn frb_internal_logging_setup_dart_logging_output() -> bool {
            $setup_dart_logging_output
        }
    };
}
