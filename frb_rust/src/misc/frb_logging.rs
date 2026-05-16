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
            sink: crate::frb_generated::StreamSink<FrbLogRecord>,
        }

        impl log::Log for FrbDartLogger {
            fn enabled(&self, metadata: &log::Metadata) -> bool {
                metadata.level() <= log::max_level()
            }

            fn log(&self, record: &log::Record) {
                if !self.enabled(record.metadata()) {
                    return;
                }

                let _ = self.sink.add(FrbLogRecord {
                    level: record.level().to_string(),
                    message: record.args().to_string(),
                    target: record.target().to_owned(),
                    module_path: record.module_path().map(ToOwned::to_owned),
                    file: record.file().map(ToOwned::to_owned),
                    line: record.line(),
                });
            }

            fn flush(&self) {}
        }

        #[doc(hidden)]
        #[flutter_rust_bridge::frb(init_dart_code = r#"
                    FrbDartLogging.init(
                      rustLogStream: frbInitLogger(maxLevel: frbLoggingMaxLevel()),
                      mapRecord: (record) => FrbLogRecordData(
                        level: record.level,
                        message: record.message,
                        target: record.target,
                        modulePath: record.modulePath,
                        file: record.file,
                        line: record.line,
                      ),
                      setupDefaultOutput: frbLoggingSetupDartLoggingOutput(),
                    );
"#)]
        pub fn frb_init_logger(
            sink: crate::frb_generated::StreamSink<FrbLogRecord>,
            max_level: String,
        ) {
            let max_level = frb_parse_logging_max_level(&max_level);

            log::set_logger(Box::leak(Box::new(FrbDartLogger { sink })))
                .map(|()| log::set_max_level(max_level))
                // This initializer is expected to run only once. If a second intentional
                // initialization use case appears, please open an issue to discuss it.
                .expect("FRB logging should only be initialized once");

            let previous_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                log::error!("{info}");
                previous_hook(info);
            }));
        }

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
        pub fn frb_logging_max_level() -> String {
            $max_level.to_string()
        }

        #[doc(hidden)]
        #[flutter_rust_bridge::frb(sync)]
        pub fn frb_logging_setup_dart_logging_output() -> bool {
            $setup_dart_logging_output
        }
    };
}
