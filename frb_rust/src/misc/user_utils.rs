use crate::misc::panic_backtrace::PanicBacktrace;

/// Setup defaults that is usually useful for a new project.
/// Surely, you are free to customize everything.
pub fn setup_default_user_utils() {
    // setup log before others, such that we can see logs in other setup functions
    #[cfg(feature = "log")]
    setup_log_to_console();
    setup_backtrace();
}

fn setup_backtrace() {
    #[cfg(not(wasm))]
    if std::env::var("RUST_BACKTRACE").err() == Some(std::env::VarError::NotPresent) {
        std::env::set_var("RUST_BACKTRACE", "1");
    } else {
        #[cfg(feature = "log")]
        log::debug!("Skip setup RUST_BACKTRACE because there is already environment variable");
    }

    PanicBacktrace::setup();
}

#[cfg(feature = "log")]
fn setup_log_to_console() {
    #[cfg(target_os = "android")]
    let _ = android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    let _ = oslog::OsLogger::new("frb_user")
        .level_filter(log::LevelFilter::Trace)
        .init();

    #[cfg(wasm)]
    let _ = crate::misc::web_utils::WebConsoleLogger::init();

    // TODO add more platforms
}
