use crate::misc::panic_backtrace::PanicBacktrace;

/// Setup defaults that is usually useful for a new project.
/// Surely, you are free to customize everything.
pub fn setup_default_user_utils() {
    // setup log before others, such that we can see logs in other setup functions
    #[cfg(feature = "log")]
    setup_log_to_console(log::LevelFilter::Trace);
    setup_backtrace();
}

/// Like [`setup_default_user_utils`], but allows configuring the log level filter.
///
/// # Example
///
/// ```rust,no_run
/// flutter_rust_bridge::setup_default_user_utils_with_log_level(
///     if cfg!(debug_assertions) { log::LevelFilter::Trace } else { log::LevelFilter::Warn }
/// );
/// ```
#[cfg(feature = "log")]
pub fn setup_default_user_utils_with_log_level(level: log::LevelFilter) {
    setup_log_to_console(level);
    setup_backtrace();
}

fn setup_backtrace() {
    #[cfg(not(target_family = "wasm"))]
    if std::env::var("RUST_BACKTRACE").err() == Some(std::env::VarError::NotPresent) {
        std::env::set_var("RUST_BACKTRACE", "1");
    } else {
        #[cfg(feature = "log")]
        log::debug!("Skip setup RUST_BACKTRACE because there is already environment variable");
    }

    PanicBacktrace::setup();
}

#[cfg(feature = "log")]
fn setup_log_to_console(#[allow(unused)] level: log::LevelFilter) {
    #[cfg(target_os = "android")]
    let _ = android_logger::init_once(
        android_logger::Config::default().with_max_level(level),
    );

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    let _ = oslog::OsLogger::new("frb_user")
        .level_filter(level)
        .init();

    #[cfg(target_family = "wasm")]
    let _ = crate::misc::web_utils::WebConsoleLogger::init(level);

    // TODO add more platforms
}
