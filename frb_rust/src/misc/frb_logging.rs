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
            sink: std::sync::RwLock<Option<crate::frb_generated::StreamSink<FrbLogRecord>>>,
        }

        impl log::Log for FrbDartLogger {
            fn enabled(&self, metadata: &log::Metadata) -> bool {
                metadata.level() <= log::max_level()
            }

            fn log(&self, record: &log::Record) {
                if !self.enabled(record.metadata()) {
                    return;
                }

                let Ok(sink) = self.sink.read() else {
                    $crate::for_generated::frb_logging_fallback_print(
                        "FRB logging sink lock is poisoned",
                    );
                    return;
                };
                let Some(sink) = sink.as_ref() else {
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
                    $crate::for_generated::frb_logging_fallback_print(&format!(
                        "Failed to forward Rust log to Dart: {error:?}",
                    ));
                }
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
            let logger = FRB_DART_LOGGER.get_or_init(|| FrbDartLogger {
                sink: std::sync::RwLock::new(None),
            });

            let _ = log::set_logger(logger);
            log::set_max_level(max_level);

            *logger.sink.write().expect("FRB logger sink lock poisoned") = Some(sink);

            FRB_DART_LOGGER_PANIC_HOOK.call_once(|| {
                let previous_hook = std::panic::take_hook();
                std::panic::set_hook(Box::new(move |info| {
                    log::error!("{info}");
                    previous_hook(info);
                }));
            });
        }

        static FRB_DART_LOGGER: std::sync::OnceLock<FrbDartLogger> = std::sync::OnceLock::new();
        static FRB_DART_LOGGER_PANIC_HOOK: std::sync::Once = std::sync::Once::new();

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

#[doc(hidden)]
pub fn frb_logging_fallback_print(message: &str) {
    #[cfg(target_os = "android")]
    {
        frb_logging_fallback_print_android(message);
        return;
    }

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    {
        frb_logging_fallback_print_apple(message);
        return;
    }

    #[cfg(target_family = "wasm")]
    {
        crate::for_generated::web_utils::js_console_error(message);
        return;
    }

    #[cfg(not(any(
        target_os = "android",
        target_os = "ios",
        target_os = "macos",
        target_family = "wasm"
    )))]
    {
        eprintln!("{message}");
    }
}

#[cfg(target_os = "android")]
fn frb_logging_fallback_print_android(message: &str) {
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int};

    const ANDROID_LOG_ERROR: c_int = 6;

    #[link(name = "log")]
    extern "C" {
        fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
    }

    let tag = CString::new("flutter_rust_bridge")
        .expect("static Android log tag should not contain interior null");
    let text = frb_logging_to_c_string(message);

    unsafe {
        __android_log_write(ANDROID_LOG_ERROR, tag.as_ptr(), text.as_ptr());
    }
}

#[cfg(all(any(target_os = "ios", target_os = "macos"), feature = "user-utils"))]
fn frb_logging_fallback_print_apple(message: &str) {
    oslog::OsLog::new("flutter_rust_bridge", "logging").error(message);
}

#[cfg(all(
    any(target_os = "ios", target_os = "macos"),
    not(feature = "user-utils")
))]
fn frb_logging_fallback_print_apple(message: &str) {
    eprintln!("{message}");
}

#[cfg(target_os = "android")]
fn frb_logging_to_c_string(message: &str) -> std::ffi::CString {
    std::ffi::CString::new(message.replace('\0', "(null)"))
        .expect("interior nulls should be replaced")
}
