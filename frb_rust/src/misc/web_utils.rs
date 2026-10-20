use wasm_bindgen::prelude::*;

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
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    pub fn js_console_log(msg: &str);

    #[wasm_bindgen(js_namespace = console, js_name = "error")]
    pub fn js_console_error(msg: &str);
}

/// Followed solution from https://stackoverflow.com/a/27369985
/// Extracts current script file path from artificially generated stack trace
pub(crate) fn script_path() -> Option<String> {
    web_sys::window()?.document().and_then(|doc| {
        doc.current_script()?.get_attribute("src").or_else(|| {
            let scripts = doc.get_elements_by_tag_name("script");
            scripts.item(scripts.length() - 1)?.get_attribute("src")
        })
    })
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
