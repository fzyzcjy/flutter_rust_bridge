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

#[wasm_bindgen]
pub fn wire_make_data_race(port_: flutter_rust_bridge::MessagePort) {
    wire_make_data_race_impl(port_)
}

#[wasm_bindgen]
pub fn wire_make_heap_use_after_free(port_: flutter_rust_bridge::MessagePort) {
    wire_make_heap_use_after_free_impl(port_)
}

#[wasm_bindgen]
pub fn wire_make_memory_leak(port_: flutter_rust_bridge::MessagePort) {
    wire_make_memory_leak_impl(port_)
}

#[wasm_bindgen]
pub fn wire_make_stack_buffer_overflow(port_: flutter_rust_bridge::MessagePort) {
    wire_make_stack_buffer_overflow_impl(port_)
}

#[wasm_bindgen]
pub fn wire_make_use_of_uninitialized_value(port_: flutter_rust_bridge::MessagePort) {
    wire_make_use_of_uninitialized_value_impl(port_)
}
