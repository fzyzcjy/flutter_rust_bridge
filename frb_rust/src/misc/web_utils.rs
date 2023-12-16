
use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! console_error {
    ($lit:literal) => {
        $crate::misc::web_utils::js_console_error($lit)
    };
    ($($tt:tt)*) => {
        $crate::misc::web_utils::js_console_error(&format!($($tt)*))
    };
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "error")]
    pub fn js_console_error(msg: &str);
}

/// Copied from https://github.com/chemicstry/wasm_thread/blob/main/src/script_path.js
/// Extracts current script file path from artificially generated stack trace
pub(crate) fn script_path() -> Option<String> {
    js_sys::eval(
        r"
(() => {
    try {
        throw new Error();
    } catch (e) {
        let parts = e.stack.match(/(?:\(|@)(\S+):\d+:\d+/);
        return parts[1];
    }
})()",
    )
    .ok()?
    .as_string()
}
