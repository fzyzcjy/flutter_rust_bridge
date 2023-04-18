use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test_inbuilt_type_2(port_: MessagePort, a: i32, b: f32) {
    wire_test_inbuilt_type_2_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_test_string_2(port_: MessagePort, s: String, i: u64) {
    wire_test_string_2_impl(port_, s, i)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_2(port_: MessagePort, custom: JsValue, s: String, i: i32) {
    wire_test_shared_struct_2_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_unique_struct_2(port_: MessagePort, custom: JsValue, s: String, i: i64) {
    wire_test_unique_struct_2_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_cross_shared_struct_2(port_: MessagePort, name: String) {
    wire_test_cross_shared_struct_2_impl(port_, name)
}

#[wasm_bindgen]
pub fn wire_test_StructDefinedInApi2(port_: MessagePort, custom: JsValue) {
    wire_test_StructDefinedInApi2_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_method__method__StructDefinedInApi2(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__StructDefinedInApi2_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__StructDefinedInApi2(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__StructDefinedInApi2_impl(port_, message)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<OnlyForApi2Struct> for JsValue {
    fn wire2api(self) -> OnlyForApi2Struct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        OnlyForApi2Struct {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
        }
    }
}
impl Wire2Api<StructDefinedInApi2> for JsValue {
    fn wire2api(self) -> StructDefinedInApi2 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructDefinedInApi2 {
            name: self_.get(0).wire2api(),
        }
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
