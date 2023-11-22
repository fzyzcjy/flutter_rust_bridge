use super::*;

// Section: impl_wire2api

impl<T> Wire2Api<Option<T>> for JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<StructWithComments> for JsValue {
    fn wire2api(self) -> StructWithComments {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructWithComments {
            field_with_comments: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<f32> for JsValue {
    fn wire2api(self) -> f32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i16> for JsValue {
    fn wire2api(self) -> i16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u16> for JsValue {
    fn wire2api(self) -> u16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u64> for JsValue {
    fn wire2api(self) -> u64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}

#[wasm_bindgen]
pub fn wire_StructWithComments_instance_method(port_: MessagePort, that: JsValue) {
    wire_StructWithComments_instance_method_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_StructWithComments_static_method(port_: MessagePort) {
    wire_StructWithComments_static_method_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star(port_: MessagePort) {
    wire_function_with_comments_slash_star_star_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line(port_: MessagePort) {
    wire_function_with_comments_triple_slash_multi_line_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line(port_: MessagePort) {
    wire_function_with_comments_triple_slash_single_line_impl(port_)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_bool(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_bool_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f32(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_f32_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f64(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_f64_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i16(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i16_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i32(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i32_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i64(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i64_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i8(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i8_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u16(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u16_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u32(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u32_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u64(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u64_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u8(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u8_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_bool(port_: MessagePort, arg: bool) {
    wire_example_primitive_type_bool_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f32(port_: MessagePort, arg: f32) {
    wire_example_primitive_type_f32_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f64(port_: MessagePort, arg: f64) {
    wire_example_primitive_type_f64_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i16(port_: MessagePort, arg: i16) {
    wire_example_primitive_type_i16_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i32(port_: MessagePort, arg: i32) {
    wire_example_primitive_type_i32_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i64(port_: MessagePort, arg: i64) {
    wire_example_primitive_type_i64_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i8(port_: MessagePort, arg: i8) {
    wire_example_primitive_type_i8_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u16(port_: MessagePort, arg: u16) {
    wire_example_primitive_type_u16_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u32(port_: MessagePort, arg: u32) {
    wire_example_primitive_type_u32_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u64(port_: MessagePort, arg: u64) {
    wire_example_primitive_type_u64_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u8(port_: MessagePort, arg: u8) {
    wire_example_primitive_type_u8_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_simple_adder(port_: MessagePort, a: i32, b: i32) {
    wire_simple_adder_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_simple_adder_sync(a: i32, b: i32) -> support::WireSyncReturn {
    wire_simple_adder_sync_impl(a, b)
}
