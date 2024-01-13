pub(crate) fn log_warn_or_println(message: &str) {
    #[cfg(feature = "log")]
    let log_enabled = log::log_enabled!(log::Level::Warn);
    #[cfg(not(feature = "log"))]
    let log_enabled = false;

    if log_enabled {
        #[cfg(feature = "log")]
        log::warn!("{}", message);
        // frb-coverage:ignore-start
        // this is not reachable, so not coverable
        #[cfg(not(feature = "log"))]
        unreachable!();
        // frb-coverage:ignore-end
    } else {
        println!("{}", message);
    }
}
