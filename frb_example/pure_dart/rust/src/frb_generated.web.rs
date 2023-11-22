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
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
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
pub fn wire_simple_adder(port_: MessagePort, a: i32, b: i32) {
    wire_simple_adder_impl(port_, a, b)
}
