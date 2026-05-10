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
        pub fn frb_init_logger(
            sink: crate::frb_generated::StreamSink<FrbLogRecord>,
            max_level: u16,
        ) {
            let max_level = match max_level {
                0 => log::LevelFilter::Off,
                1 => log::LevelFilter::Error,
                2 => log::LevelFilter::Warn,
                3 => log::LevelFilter::Info,
                4 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            };

            log::set_logger(Box::leak(Box::new(FrbDartLogger { sink })))
                .map(|()| log::set_max_level(max_level))
                .expect("FRB logging should only be initialized once");

            let previous_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                log::error!("{info}");
                previous_hook(info);
            }));
        }

        #[doc(hidden)]
        pub fn frb_logging_max_level() -> u16 {
            match $max_level {
                log::LevelFilter::Off => 0,
                log::LevelFilter::Error => 1,
                log::LevelFilter::Warn => 2,
                log::LevelFilter::Info => 3,
                log::LevelFilter::Debug => 4,
                log::LevelFilter::Trace => 5,
            }
        }

        #[doc(hidden)]
        pub fn frb_logging_setup_dart_logging_output() -> bool {
            $setup_dart_logging_output
        }
    };
}
