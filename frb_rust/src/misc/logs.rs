pub(crate) fn log_warn_or_println(message: &str) {
    if log::log_enabled!(log::Level::Warn) {
        log::warn!("{}", message);
    } else {
        println!("{}", message);
    }
}
