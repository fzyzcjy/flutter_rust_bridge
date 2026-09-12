use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn schedule_callback(callback: impl FnOnce() + 'static) {
    let callback = Closure::once_into_js(callback);
    js_set_timeout(callback.unchecked_ref(), 0).expect("Failed to schedule Web callback");
}

#[macro_export]
macro_rules! console_error {
    ($lit:literal) => {
        $crate::for_generated::web_utils::js_console_error($lit)
    };
    ($($tt:tt)*) => {
        $crate::for_generated::web_utils::js_console_error(&format!($($tt)*))
    };
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout, catch)]
    fn js_set_timeout(callback: &js_sys::Function, delay: i32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    pub fn js_console_log(msg: &str);

    #[wasm_bindgen(js_namespace = console, js_name = "error")]
    pub fn js_console_error(msg: &str);
}

/// Copied from https://github.com/chemicstry/wasm_thread/blob/main/src/script_path.js
/// Extracts current script file path from artificially generated stack trace
pub(crate) fn script_path() -> Option<String> {
    js_sys::eval(
        r#"
(() => {
    try {
        throw new Error("Harmless thrown-and-caught error to get script_path");
    } catch (e) {
        let parts = e.stack.match(/(?:\(|@)(\S+):\d+:\d+/);
        return parts[1];
    }
})()"#,
    )
    .ok()?
    .as_string()
}

#[cfg(feature = "log")]
#[derive(Clone, Copy)]
pub(crate) struct WebConsoleLogger;

#[cfg(feature = "log")]
static WEB_CONSOLE_LOGGER: WebConsoleLogger = WebConsoleLogger;

#[cfg(feature = "log")]
impl WebConsoleLogger {
    pub(crate) fn init(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
        log::set_logger(&WEB_CONSOLE_LOGGER).map(|()| log::set_max_level(level))
    }
}

#[cfg(feature = "log")]
impl log::Log for WebConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            js_console_log(&format!("{} - {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}
