use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test_inbuilt_type_1(port_: MessagePort, a: i32, b: f32) {
    wire_test_inbuilt_type_1_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_test_string_1(port_: MessagePort, s: String, i: u64) {
    wire_test_string_1_impl(port_, s, i)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_1(port_: MessagePort, custom: JsValue, s: String, i: i32) {
    wire_test_shared_struct_1_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_unique_struct_1(port_: MessagePort, custom: JsValue, s: String, i: i16) {
    wire_test_unique_struct_1_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_cross_shared_struct_1(port_: MessagePort, custom: JsValue) {
    wire_test_cross_shared_struct_1_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_StructDefinedInApi1(port_: MessagePort, custom: JsValue) {
    wire_test_StructDefinedInApi1_impl(port_, custom)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<OnlyForApi1Struct> for JsValue {
    fn wire2api(self) -> OnlyForApi1Struct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        OnlyForApi1Struct {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
        }
    }
}
impl Wire2Api<StructDefinedInApi1> for JsValue {
    fn wire2api(self) -> StructDefinedInApi1 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructDefinedInApi1 {
            name: self_.get(0).wire2api(),
        }
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<i16> for JsValue {
    fn wire2api(self) -> i16 {
        self.unchecked_into_f64() as _
    }
}
