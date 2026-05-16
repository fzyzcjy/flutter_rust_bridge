/// Print a message directly to the platform console.
///
/// This deliberately avoids the `log` crate, because it is used by logging
/// failure paths where calling `log::*` could recursively re-enter the same
/// failing logger.
#[doc(hidden)]
#[cfg(all(target_os = "android", feature = "user-utils", feature = "log"))]
pub fn print_to_console(message: &str) {
    let record = log::Record::builder()
        .args(format_args!("{message}"))
        .level(log::Level::Error)
        .target("flutter_rust_bridge")
        .module_path_static(Some("flutter_rust_bridge"))
        .build();

    android_logger::log(&record);
}

#[doc(hidden)]
#[cfg(all(any(target_os = "ios", target_os = "macos"), feature = "user-utils"))]
pub fn print_to_console(message: &str) {
    oslog::OsLog::new("flutter_rust_bridge", "console").error(message);
}

#[doc(hidden)]
#[cfg(target_family = "wasm")]
pub fn print_to_console(message: &str) {
    crate::for_generated::web_utils::js_console_error(message);
}

#[doc(hidden)]
#[cfg(all(
    not(target_family = "wasm"),
    not(all(target_os = "android", feature = "user-utils", feature = "log")),
    not(all(any(target_os = "ios", target_os = "macos"), feature = "user-utils"))
))]
pub fn print_to_console(message: &str) {
    eprintln!("{message}");
}
