use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test_inbuilt_type_in_block_2(port_: MessagePort, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_2_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_test_string_in_block_2(port_: MessagePort, s: String, i: u64) {
    wire_test_string_in_block_2_impl(port_, s, i)
}

#[wasm_bindgen]
pub fn wire_test_all_shared_struct_in_block_2(
    port_: MessagePort,
    custom: JsValue,
    s: String,
    i: i32,
) {
    wire_test_all_shared_struct_in_block_2_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_in_block_2_for_1_and_2(
    port_: MessagePort,
    custom: JsValue,
    s: String,
    i: i32,
) {
    wire_test_shared_struct_in_block_2_for_1_and_2_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_cross_shared_struct_in_block_2_for_1_and_2(port_: MessagePort, name: String) {
    wire_test_cross_shared_struct_in_block_2_for_1_and_2_impl(port_, name)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_in_block_2_for_2_and_3(
    port_: MessagePort,
    custom: JsValue,
    s: String,
    i: i32,
) {
    wire_test_shared_struct_in_block_2_for_2_and_3_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_cross_shared_struct_in_block_2_for_2_and_3(port_: MessagePort, custom: JsValue) {
    wire_test_cross_shared_struct_in_block_2_for_2_and_3_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_unique_struct_2(port_: MessagePort, custom: JsValue, s: String, i: i16) {
    wire_test_unique_struct_2_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_struct_defined_in_block_2(port_: MessagePort, custom: JsValue) {
    wire_test_struct_defined_in_block_2_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_method__method__StructDefinedInBlock2(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__StructDefinedInBlock2_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__StructDefinedInBlock2(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__StructDefinedInBlock2_impl(port_, message)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<StructDefinedInBlock2> for JsValue {
    fn wire2api(self) -> StructDefinedInBlock2 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructDefinedInBlock2 {
            name: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<StructOnlyForBlock2> for JsValue {
    fn wire2api(self) -> StructOnlyForBlock2 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        StructOnlyForBlock2 {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
        }
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<i16> for JsValue {
    fn wire2api(self) -> i16 {
        self.unchecked_into_f64() as _
    }
}
