use crate::misc::panic_backtrace::PanicBacktrace;

/// Setup defaults that is usually useful for a new project.
/// Surely, you are free to customize everything.
pub fn setup_default_user_utils() {
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
fn setup_log_to_console() {
// Todo: Remove after verification that println indeed writes to the mobile loggers
 //     #[cfg(target_os = "android")]
//     let _ = android_logger::init_once(
//         android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
//     );

//     #[cfg(any(target_os = "ios", target_os = "macos"))]
//     let _ = oslog::OsLogger::new("frb_user")
//         .level_filter(log::LevelFilter::Trace)
//         .init();
// TODO test and remove if not needed, i.e. printline writes into the console as well
    #[cfg(target_family = "wasm")]
    let _ = crate::misc::web_utils::WebConsoleLogger::init();

    // TODO add more platforms
}