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

        type FrbLogSink = crate::frb_generated::StreamSink<FrbLogRecord>;

        struct FrbDartLogger {
            sinks: std::sync::RwLock<std::collections::BTreeMap<u64, std::sync::Arc<FrbLogSink>>>,
            next_sink_id: std::sync::atomic::AtomicU64,
        }

        impl log::Log for FrbDartLogger {
            fn enabled(&self, metadata: &log::Metadata) -> bool {
                metadata.level() <= log::max_level()
            }

            fn log(&self, record: &log::Record) {
                if !self.enabled(record.metadata()) {
                    return;
                }

                let sinks = self.load_sinks();
                if sinks.is_empty() {
                    frb_log_record_to_console(record);
                    return;
                }

                let mut failed_sink_ids = Vec::new();
                let mut first_error = None;
                for (sink_id, sink) in sinks {
                    if let Err(error) = sink.add(FrbLogRecord {
                        level: record.level().to_string(),
                        message: record.args().to_string(),
                        target: record.target().to_owned(),
                        module_path: record.module_path().map(ToOwned::to_owned),
                        file: record.file().map(ToOwned::to_owned),
                        line: record.line(),
                    }) {
                        first_error.get_or_insert_with(|| format!("{error:?}"));
                        failed_sink_ids.push(sink_id);
                    }
                }
                if let Some(first_error) = first_error {
                    $crate::for_generated::print_to_console(&format!(
                        "Failed to forward Rust log to {} Dart sink(s): {first_error}",
                        failed_sink_ids.len(),
                    ));
                }
                for failed_sink_id in failed_sink_ids {
                    self.clear_sink(failed_sink_id);
                }
            }

            fn flush(&self) {}
        }

        impl FrbDartLogger {
            fn load_sinks(&self) -> Vec<(u64, std::sync::Arc<FrbLogSink>)> {
                self.sinks
                    .read()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .iter()
                    .map(|(&id, sink)| (id, sink.clone()))
                    .collect()
            }

            fn allocate_sink_id(&self) -> u64 {
                self.next_sink_id
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            }

            fn insert_sink(&self, id: u64, sink: FrbLogSink) {
                let old_sink = self
                    .sinks
                    .write()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .insert(id, std::sync::Arc::new(sink));

                drop(old_sink);
            }

            fn clear_sink(&self, id: u64) {
                let old_sink = self
                    .sinks
                    .write()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .remove(&id);

                drop(old_sink);
            }
        }

        fn frb_log_record_to_console(record: &log::Record) {
            $crate::for_generated::print_to_console(&format!(
                "{} - {}",
                record.level(),
                record.args()
            ));
        }

        #[doc(hidden)]
        #[flutter_rust_bridge::frb(sync)]
        #[flutter_rust_bridge::frb(init_dart_code = r#"
                    final rustLoggerId = frbInternalLoggingAllocateSinkId();
                    kFrbDartLogging.init(
                      rustLogStream: frbInternalInitLogger(id: rustLoggerId, maxLevel: frbInternalLoggingMaxLevel()),
                      mapRecord: (record) => FrbLogRecordData(
                        level: record.level,
                        message: record.message,
                        target: record.target,
                        modulePath: record.modulePath,
                        file: record.file,
                        line: record.line,
                      ),
                      setupDefaultOutput: frbInternalLoggingSetupDartLoggingOutput(),
                      disposeRustLogger: () => frbInternalDisposeLogger(id: rustLoggerId),
                    );
"#)]
        pub fn frb_internal_init_logger(
            sink: crate::frb_generated::StreamSink<FrbLogRecord>,
            id: u64,
            max_level: String,
        ) {
            let max_level = frb_parse_logging_max_level(&max_level);
            let logger = FRB_DART_LOGGER.get_or_init(|| FrbDartLogger {
                sinks: std::sync::RwLock::new(std::collections::BTreeMap::new()),
                next_sink_id: std::sync::atomic::AtomicU64::new(1),
            });

            let _ = log::set_logger(logger);
            log::set_max_level(max_level);

            logger.insert_sink(id, sink);
        }

        #[doc(hidden)]
        #[flutter_rust_bridge::frb(sync)]
        pub fn frb_internal_dispose_logger(id: u64) {
            if let Some(logger) = FRB_DART_LOGGER.get() {
                logger.clear_sink(id);
            }
        }

        static FRB_DART_LOGGER: std::sync::OnceLock<FrbDartLogger> = std::sync::OnceLock::new();

        #[doc(hidden)]
        #[flutter_rust_bridge::frb(sync)]
        pub fn frb_internal_logging_allocate_sink_id() -> u64 {
            FRB_DART_LOGGER
                .get_or_init(|| FrbDartLogger {
                    sinks: std::sync::RwLock::new(std::collections::BTreeMap::new()),
                    next_sink_id: std::sync::atomic::AtomicU64::new(1),
                })
                .allocate_sink_id()
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
