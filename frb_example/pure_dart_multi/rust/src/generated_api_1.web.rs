use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test_inbuilt_type_in_block_1(port_: MessagePort, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_1_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_test_string_in_block_1(port_: MessagePort, s: String, i: u64) {
    wire_test_string_in_block_1_impl(port_, s, i)
}

#[wasm_bindgen]
pub fn wire_test_string_in_sync_in_block_1(s: String, i: u64) -> support::WireSyncReturn {
    wire_test_string_in_sync_in_block_1_impl(s, i)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1(
    score: f64,
) -> support::WireSyncReturn {
    wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1_impl(score)
}

#[wasm_bindgen]
pub fn wire_test_all_shared_struct_in_block_1(
    port_: MessagePort,
    custom: JsValue,
    s: String,
    i: i32,
) {
    wire_test_all_shared_struct_in_block_1_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_in_block_1_for_1_and_2(
    port_: MessagePort,
    custom: JsValue,
    s: String,
    i: i32,
) {
    wire_test_shared_struct_in_block_1_for_1_and_2_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_cross_shared_struct_in_block_1_for_1_and_2(port_: MessagePort, custom: JsValue) {
    wire_test_cross_shared_struct_in_block_1_for_1_and_2_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_unique_struct_1(port_: MessagePort, custom: JsValue, s: String, i: i8) {
    wire_test_unique_struct_1_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_struct_defined_in_block_1(port_: MessagePort, custom: JsValue) {
    wire_test_struct_defined_in_block_1_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_method__method__StructDefinedInBlock1(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__StructDefinedInBlock1_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__StructDefinedInBlock1(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__StructDefinedInBlock1_impl(port_, message)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<Option<String>> for Option<String> {
    fn wire2api(self) -> Option<String> {
        self.map(Wire2Api::wire2api)
    }
}

impl Wire2Api<StructDefinedInBlock1> for JsValue {
    fn wire2api(self) -> StructDefinedInBlock1 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructDefinedInBlock1 {
            name: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<StructOnlyForBlock1> for JsValue {
    fn wire2api(self) -> StructOnlyForBlock1 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        StructOnlyForBlock1 {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
        }
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Option<String>> for JsValue {
    fn wire2api(self) -> Option<String> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<f64>> for JsValue {
    fn wire2api(self) -> Option<f64> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<i8>> for JsValue {
    fn wire2api(self) -> Option<i8> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
