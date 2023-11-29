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
impl Wire2Api<i32> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}

#[wasm_bindgen]
pub fn wire_add(left: i32, right: i32) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_add_impl(left, right)
}
