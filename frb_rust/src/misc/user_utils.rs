use crate::misc::panic_backtrace::PanicBacktrace;

/// Setup defaults that is usually useful for a new project.
/// Surely, you are free to customize everything.
pub fn setup_default_user_utils() {
    // setup log before others, such that we can see logs in other setup functions
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

// TODO: check if web logging requires this setup
// #[cfg(feature = "log")]
// fn setup_log_to_console() {
//     #[cfg(target_family = "wasm")]
//     let _ = crate::misc::web_utils::WebConsoleLogger::init();

//     // TODO add more platforms
// }
