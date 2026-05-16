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
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int};

    const ANDROID_LOG_ERROR: c_int = 6;

    #[link(name = "log")]
    extern "C" {
        fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
    }

    let tag = CString::new("flutter_rust_bridge")
        .expect("static Android log tag should not contain interior null");
    let text = string_to_c_string(message);

    unsafe {
        __android_log_write(ANDROID_LOG_ERROR, tag.as_ptr(), text.as_ptr());
    }
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

#[cfg(target_os = "android")]
fn string_to_c_string(message: &str) -> std::ffi::CString {
    std::ffi::CString::new(message.replace('\0', "(null)"))
        .expect("interior nulls should be replaced")
}
