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
impl Wire2Api<StructWithCommentsTwinNormal> for JsValue {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructWithCommentsTwinNormal {
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
pub fn wire_StructWithCommentsTwinNormal_instance_method(port_: MessagePort, that: JsValue) {
    wire_StructWithCommentsTwinNormal_instance_method_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinNormal_static_method(port_: MessagePort) {
    wire_StructWithCommentsTwinNormal_static_method_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star_twin_normal(port_: MessagePort) {
    wire_function_with_comments_slash_star_star_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line_twin_normal(port_: MessagePort) {
    wire_function_with_comments_triple_slash_multi_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line_twin_normal(port_: MessagePort) {
    wire_function_with_comments_triple_slash_single_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_simple_adder_twin_normal(port_: MessagePort, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}
