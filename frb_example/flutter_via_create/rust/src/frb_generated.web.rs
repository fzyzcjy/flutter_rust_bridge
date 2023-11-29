// Section: imports

use super::*;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::wasm_bindgen;
use flutter_rust_bridge::wasm_bindgen::prelude::*;
use flutter_rust_bridge::Handler;

// Section: impl_wire2api

impl<T> Wire2Api<Option<T>> for flutter_rust_bridge::wasm_bindgen::JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl Wire2Api<String> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<Vec<u8>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Uint8Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<u8> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}

#[wasm_bindgen]
pub fn wire_greet(name: String) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_greet_impl(name)
}
