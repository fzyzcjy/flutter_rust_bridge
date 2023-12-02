#[macro_export]
macro_rules! console_error {
    ($lit:literal) => {
        $crate::js_console_error($lit)
    };
    ($($tt:tt)*) => {
        $crate::js_console_error(&format!($($tt)*))
    };
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "error")]
    pub fn js_console_error(msg: &str);
}
