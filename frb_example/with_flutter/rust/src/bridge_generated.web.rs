use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_draw_mandelbrot(
    port_: MessagePort,
    image_size: JsValue,
    zoom_point: JsValue,
    scale: f64,
    num_threads: i32,
) {
    wire_draw_mandelbrot_impl(port_, image_size, zoom_point, scale, num_threads)
}

#[wasm_bindgen]
pub fn wire_passing_complex_structs(port_: MessagePort, root: JsValue) {
    wire_passing_complex_structs_impl(port_, root)
}

#[wasm_bindgen]
pub fn wire_returning_structs_with_boxed_fields(port_: MessagePort) {
    wire_returning_structs_with_boxed_fields_impl(port_)
}

#[wasm_bindgen]
pub fn wire_off_topic_memory_test_input_array(port_: MessagePort, input: Box<[u8]>) {
    wire_off_topic_memory_test_input_array_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_off_topic_memory_test_output_zero_copy_buffer(port_: MessagePort, len: i32) {
    wire_off_topic_memory_test_output_zero_copy_buffer_impl(port_, len)
}

#[wasm_bindgen]
pub fn wire_off_topic_memory_test_output_vec_u8(port_: MessagePort, len: i32) {
    wire_off_topic_memory_test_output_vec_u8_impl(port_, len)
}

#[wasm_bindgen]
pub fn wire_off_topic_memory_test_input_vec_of_object(port_: MessagePort, input: JsValue) {
    wire_off_topic_memory_test_input_vec_of_object_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_off_topic_memory_test_output_vec_of_object(port_: MessagePort, len: i32) {
    wire_off_topic_memory_test_output_vec_of_object_impl(port_, len)
}

#[wasm_bindgen]
pub fn wire_off_topic_memory_test_input_complex_struct(port_: MessagePort, input: JsValue) {
    wire_off_topic_memory_test_input_complex_struct_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_off_topic_memory_test_output_complex_struct(port_: MessagePort, len: i32) {
    wire_off_topic_memory_test_output_complex_struct_impl(port_, len)
}

#[wasm_bindgen]
pub fn wire_off_topic_deliberately_return_error(port_: MessagePort) {
    wire_off_topic_deliberately_return_error_impl(port_)
}

#[wasm_bindgen]
pub fn wire_off_topic_deliberately_panic(port_: MessagePort) {
    wire_off_topic_deliberately_panic_impl(port_)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> Result<String, &'static str> {
        Ok(self)
    }
}

impl Wire2Api<Vec<Size>> for JsValue {
    fn wire2api(self) -> Result<Vec<Size>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<TreeNode>> for JsValue {
    fn wire2api(self) -> Result<Vec<TreeNode>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Point> for JsValue {
    fn wire2api(self) -> Result<Point, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let x = self_.get(0).wire2api();
        let y = self_.get(1).wire2api();
        Ok(Point { x: x?, y: y? })
    }
}
impl Wire2Api<Size> for JsValue {
    fn wire2api(self) -> Result<Size, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let width = self_.get(0).wire2api();
        let height = self_.get(1).wire2api();
        Ok(Size {
            width: width?,
            height: height?,
        })
    }
}
impl Wire2Api<TreeNode> for JsValue {
    fn wire2api(self) -> Result<TreeNode, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let name = self_.get(0).wire2api();
        let children = self_.get(1).wire2api();
        Ok(TreeNode {
            name: name?,
            children: children?,
        })
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Result<Vec<u8>, &'static str> {
        Ok(self.into_vec())
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> Result<String, &'static str> {
        Ok(self.as_string().expect("non-UTF-8 string, or not a string"))
    }
}
impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> Result<f64, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> Result<i32, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> Result<u8, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Result<Vec<u8>, &'static str> {
        Ok(self.unchecked_into::<js_sys::Uint8Array>().to_vec().into())
    }
}
