/// Print a message directly to the platform console.
///
/// This deliberately avoids the `log` crate, because it is used by logging
/// failure paths where calling `log::*` could recursively re-enter the same
/// failing logger.
#[doc(hidden)]
pub fn print_to_console(message: &str) {
    #[cfg(target_os = "android")]
    {
        print_to_console_android(message);
        return;
    }

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    {
        print_to_console_apple(message);
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
fn print_to_console_android(message: &str) {
    print_to_console_android_impl(message);
}

#[cfg(all(target_os = "android", feature = "user-utils", feature = "log"))]
fn print_to_console_android_impl(message: &str) {
    let record = log::Record::builder()
        .args(format_args!("{message}"))
        .level(log::Level::Error)
        .target("flutter_rust_bridge")
        .module_path_static(Some("flutter_rust_bridge"))
        .build();

    android_logger::log(&record);
}

#[cfg(all(
    target_os = "android",
    not(all(feature = "user-utils", feature = "log"))
))]
fn print_to_console_android_impl(message: &str) {
    eprintln!("{message}");
}

#[cfg(all(any(target_os = "ios", target_os = "macos"), feature = "user-utils"))]
fn print_to_console_apple(message: &str) {
    oslog::OsLog::new("flutter_rust_bridge", "console").error(message);
}

#[cfg(all(
    any(target_os = "ios", target_os = "macos"),
    not(feature = "user-utils")
))]
fn print_to_console_apple(message: &str) {
    eprintln!("{message}");
}
