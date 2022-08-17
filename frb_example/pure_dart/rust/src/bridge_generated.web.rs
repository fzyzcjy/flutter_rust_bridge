use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_simple_adder(port_: MessagePort, a: i32, b: i32) {
    wire_simple_adder_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_primitive_types(
    port_: MessagePort,
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) {
    wire_primitive_types_impl(port_, my_i32, my_i64, my_f64, my_bool)
}

#[wasm_bindgen]
pub fn wire_primitive_u32(port_: MessagePort, my_u32: u32) {
    wire_primitive_u32_impl(port_, my_u32)
}

#[wasm_bindgen]
pub fn wire_handle_string(port_: MessagePort, s: String) {
    wire_handle_string_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_return_unit(port_: MessagePort) {
    wire_handle_return_unit_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_vec_u8(port_: MessagePort, v: Box<[u8]>) {
    wire_handle_vec_u8_impl(port_, v)
}

#[wasm_bindgen]
pub fn wire_handle_vec_of_primitive(port_: MessagePort, n: i32) {
    wire_handle_vec_of_primitive_impl(port_, n)
}

#[wasm_bindgen]
pub fn wire_handle_zero_copy_vec_of_primitive(port_: MessagePort, n: i32) {
    wire_handle_zero_copy_vec_of_primitive_impl(port_, n)
}

#[wasm_bindgen]
pub fn wire_handle_struct(port_: MessagePort, arg: JsValue, boxed: JsValue) {
    wire_handle_struct_impl(port_, arg, boxed)
}

#[wasm_bindgen]
pub fn wire_handle_newtype(port_: MessagePort, arg: JsValue) {
    wire_handle_newtype_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_list_of_struct(port_: MessagePort, l: JsValue) {
    wire_handle_list_of_struct_impl(port_, l)
}

#[wasm_bindgen]
pub fn wire_handle_string_list(port_: MessagePort, names: JsValue) {
    wire_handle_string_list_impl(port_, names)
}

#[wasm_bindgen]
pub fn wire_handle_complex_struct(port_: MessagePort, s: JsValue) {
    wire_handle_complex_struct_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_sync_return(mode: String) -> support::WireSyncReturnStruct {
    wire_handle_sync_return_impl(mode)
}

#[wasm_bindgen]
pub fn wire_handle_stream(port_: MessagePort, arg: String) {
    wire_handle_stream_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_stream_of_struct(port_: MessagePort) {
    wire_handle_stream_of_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_err(port_: MessagePort) {
    wire_return_err_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_panic(port_: MessagePort) {
    wire_return_panic_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_optional_return(port_: MessagePort, left: f64, right: f64) {
    wire_handle_optional_return_impl(port_, left, right)
}

#[wasm_bindgen]
pub fn wire_handle_optional_struct(port_: MessagePort, document: Option<String>) {
    wire_handle_optional_struct_impl(port_, document)
}

#[wasm_bindgen]
pub fn wire_handle_optional_increment(port_: MessagePort, opt: JsValue) {
    wire_handle_optional_increment_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_handle_increment_boxed_optional(port_: MessagePort, opt: Option<Pointer<f64>>) {
    wire_handle_increment_boxed_optional_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_handle_option_box_arguments(
    port_: MessagePort,
    i8box: Option<Pointer<i8>>,
    u8box: Option<Pointer<u8>>,
    i32box: Option<Pointer<i32>>,
    i64box: Option<Pointer<i64>>,
    f64box: Option<Pointer<f64>>,
    boolbox: Option<Pointer<bool>>,
    structbox: JsValue,
) {
    wire_handle_option_box_arguments_impl(
        port_, i8box, u8box, i32box, i64box, f64box, boolbox, structbox,
    )
}

#[wasm_bindgen]
pub fn wire_print_note(port_: MessagePort, note: JsValue) {
    wire_print_note_impl(port_, note)
}

#[wasm_bindgen]
pub fn wire_handle_return_enum(port_: MessagePort, input: String) {
    wire_handle_return_enum_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_enum_parameter(port_: MessagePort, weekday: i32) {
    wire_handle_enum_parameter_impl(port_, weekday)
}

#[wasm_bindgen]
pub fn wire_handle_customized_struct(port_: MessagePort, val: JsValue) {
    wire_handle_customized_struct_impl(port_, val)
}

#[wasm_bindgen]
pub fn wire_handle_enum_struct(port_: MessagePort, val: JsValue) {
    wire_handle_enum_struct_impl(port_, val)
}

#[wasm_bindgen]
pub fn wire_use_imported_struct(port_: MessagePort, my_struct: JsValue) {
    wire_use_imported_struct_impl(port_, my_struct)
}

#[wasm_bindgen]
pub fn wire_use_imported_enum(port_: MessagePort, my_enum: i32) {
    wire_use_imported_enum_impl(port_, my_enum)
}

#[wasm_bindgen]
pub fn wire_get_app_settings(port_: MessagePort) {
    wire_get_app_settings_impl(port_)
}

#[wasm_bindgen]
pub fn wire_is_app_embedded(port_: MessagePort, app_settings: JsValue) {
    wire_is_app_embedded_impl(port_, app_settings)
}

#[wasm_bindgen]
pub fn wire_get_message(port_: MessagePort) {
    wire_get_message_impl(port_)
}

#[wasm_bindgen]
pub fn wire_repeat_number(port_: MessagePort, num: i32, times: usize) {
    wire_repeat_number_impl(port_, num, times)
}

#[wasm_bindgen]
pub fn wire_repeat_sequence(port_: MessagePort, seq: i32, times: usize) {
    wire_repeat_sequence_impl(port_, seq, times)
}

#[wasm_bindgen]
pub fn wire_first_number(port_: MessagePort, nums: JsValue) {
    wire_first_number_impl(port_, nums)
}

#[wasm_bindgen]
pub fn wire_first_sequence(port_: MessagePort, seqs: JsValue) {
    wire_first_sequence_impl(port_, seqs)
}

#[wasm_bindgen]
pub fn wire_get_array(port_: MessagePort) {
    wire_get_array_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_complex_array(port_: MessagePort) {
    wire_get_complex_array_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_usize(port_: MessagePort, u: usize) {
    wire_get_usize_impl(port_, u)
}

#[wasm_bindgen]
pub fn wire_next_user_id(port_: MessagePort, user_id: JsValue) {
    wire_next_user_id_impl(port_, user_id)
}

#[wasm_bindgen]
pub fn wire_register_event_listener(port_: MessagePort) {
    wire_register_event_listener_impl(port_)
}

#[wasm_bindgen]
pub fn wire_close_event_listener(port_: MessagePort) {
    wire_close_event_listener_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_event(port_: MessagePort, address: String, payload: String) {
    wire_create_event_impl(port_, address, payload)
}

#[wasm_bindgen]
pub fn wire_handle_stream_sink_at_1(port_: MessagePort, key: u32, max: u32) {
    wire_handle_stream_sink_at_1_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_handle_stream_sink_at_2(port_: MessagePort, key: u32, max: u32) {
    wire_handle_stream_sink_at_2_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_handle_stream_sink_at_3(port_: MessagePort, key: u32, max: u32) {
    wire_handle_stream_sink_at_3_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_get_sum_struct(port_: MessagePort) {
    wire_get_sum_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_multiply_by_ten(port_: MessagePort, measure: JsValue) {
    wire_multiply_by_ten_impl(port_, measure)
}

#[wasm_bindgen]
pub fn wire_sum__method__SumWith(port_: MessagePort, that: JsValue, y: u32, z: u32) {
    wire_sum__method__SumWith_impl(port_, that, y, z)
}

#[wasm_bindgen]
pub fn wire_new__static_method__ConcatenateWith(port_: MessagePort, a: String) {
    wire_new__static_method__ConcatenateWith_impl(port_, a)
}

#[wasm_bindgen]
pub fn wire_concatenate__method__ConcatenateWith(port_: MessagePort, that: JsValue, b: String) {
    wire_concatenate__method__ConcatenateWith_impl(port_, that, b)
}

#[wasm_bindgen]
pub fn wire_concatenate_static__static_method__ConcatenateWith(
    port_: MessagePort,
    a: String,
    b: String,
) {
    wire_concatenate_static__static_method__ConcatenateWith_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_handle_some_stream_sink__method__ConcatenateWith(
    port_: MessagePort,
    that: JsValue,
    key: u32,
    max: u32,
) {
    wire_handle_some_stream_sink__method__ConcatenateWith_impl(port_, that, key, max)
}

#[wasm_bindgen]
pub fn wire_handle_some_stream_sink_at_1__method__ConcatenateWith(
    port_: MessagePort,
    that: JsValue,
) {
    wire_handle_some_stream_sink_at_1__method__ConcatenateWith_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_handle_some_static_stream_sink__static_method__ConcatenateWith(
    port_: MessagePort,
    key: u32,
    max: u32,
) {
    wire_handle_some_static_stream_sink__static_method__ConcatenateWith_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_handle_some_static_stream_sink_single_arg__static_method__ConcatenateWith(
    port_: MessagePort,
) {
    wire_handle_some_static_stream_sink_single_arg__static_method__ConcatenateWith_impl(port_)
}

// Section: allocate functions

#[wasm_bindgen]
pub fn new_box_autoadd_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_autoadd_f64_0(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_autoadd_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_autoadd_i64_0(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_f64_0(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_i64_0(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_i8_0(value: i8) -> *mut i8 {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_u8_0(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_weekdays_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<Vec<String>> for JsValue {
    fn wire2api(self) -> Vec<String> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for Box<[u8]> {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
        ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<ApplicationEnv> for JsValue {
    fn wire2api(self) -> ApplicationEnv {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        ApplicationEnv {
            vars: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<ApplicationEnvVar> for JsValue {
    fn wire2api(self) -> ApplicationEnvVar {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        ApplicationEnvVar(self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}

impl Wire2Api<ApplicationSettings> for JsValue {
    fn wire2api(self) -> ApplicationSettings {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        ApplicationSettings {
            name: self_.get(0).wire2api(),
            version: self_.get(1).wire2api(),
            mode: self_.get(2).wire2api(),
            env: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<Attribute> for JsValue {
    fn wire2api(self) -> Attribute {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        Attribute {
            key: self_.get(0).wire2api(),
            value: self_.get(1).wire2api(),
        }
    }
}

impl Wire2Api<bool> for Pointer<bool> {
    fn wire2api(self) -> bool {
        unsafe { *support::box_from_leak_ptr(self.as_mut()) }
    }
}

impl Wire2Api<f64> for Pointer<f64> {
    fn wire2api(self) -> f64 {
        unsafe { *support::box_from_leak_ptr(self.as_mut()) }
    }
}
impl Wire2Api<i32> for Pointer<i32> {
    fn wire2api(self) -> i32 {
        unsafe { *support::box_from_leak_ptr(self.as_mut()) }
    }
}
impl Wire2Api<i64> for Pointer<i64> {
    fn wire2api(self) -> i64 {
        unsafe { *support::box_from_leak_ptr(self.as_mut()) }
    }
}

impl Wire2Api<Box<bool>> for Pointer<bool> {
    fn wire2api(self) -> Box<bool> {
        unsafe { support::box_from_leak_ptr(self.as_mut()) }
    }
}

impl Wire2Api<Box<f64>> for Pointer<f64> {
    fn wire2api(self) -> Box<f64> {
        unsafe { support::box_from_leak_ptr(self.as_mut()) }
    }
}
impl Wire2Api<Box<i32>> for Pointer<i32> {
    fn wire2api(self) -> Box<i32> {
        unsafe { support::box_from_leak_ptr(self.as_mut()) }
    }
}
impl Wire2Api<Box<i64>> for Pointer<i64> {
    fn wire2api(self) -> Box<i64> {
        unsafe { support::box_from_leak_ptr(self.as_mut()) }
    }
}
impl Wire2Api<Box<i8>> for Pointer<i8> {
    fn wire2api(self) -> Box<i8> {
        unsafe { support::box_from_leak_ptr(self.as_mut()) }
    }
}

impl Wire2Api<Box<u8>> for Pointer<u8> {
    fn wire2api(self) -> Box<u8> {
        unsafe { support::box_from_leak_ptr(self.as_mut()) }
    }
}

impl Wire2Api<ConcatenateWith> for JsValue {
    fn wire2api(self) -> ConcatenateWith {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        ConcatenateWith {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<Customized> for JsValue {
    fn wire2api(self) -> Customized {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        Customized {
            final_field: self_.get(0).wire2api(),
            non_final_field: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<Distance> for JsValue {
    fn wire2api(self) -> Distance {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Distance::Unknown,
            1 => Distance::Map(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<ExoticOptionals> for JsValue {
    fn wire2api(self) -> ExoticOptionals {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            14,
            "Expected 14 elements, got {}",
            self_.length()
        );
        ExoticOptionals {
            int32: self_.get(0).wire2api(),
            int64: self_.get(1).wire2api(),
            float64: self_.get(2).wire2api(),
            boolean: self_.get(3).wire2api(),
            zerocopy: self_.get(4).wire2api(),
            int8list: self_.get(5).wire2api(),
            uint8list: self_.get(6).wire2api(),
            int32list: self_.get(7).wire2api(),
            float32list: self_.get(8).wire2api(),
            float64list: self_.get(9).wire2api(),
            attributes: self_.get(10).wire2api(),
            attributes_nullable: self_.get(11).wire2api(),
            nullable_attributes: self_.get(12).wire2api(),
            newtypeint: self_.get(13).wire2api(),
        }
    }
}

impl Wire2Api<Vec<f32>> for Box<[f32]> {
    fn wire2api(self) -> Vec<f32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Vec<f64> {
        self.into_vec()
    }
}

impl Wire2Api<Vec<i32>> for Box<[i32]> {
    fn wire2api(self) -> Vec<i32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i8>> for Box<[i8]> {
    fn wire2api(self) -> Vec<i8> {
        self.into_vec()
    }
}
impl Wire2Api<KitchenSink> for JsValue {
    fn wire2api(self) -> KitchenSink {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => KitchenSink::Empty,
            1 => KitchenSink::Primitives {
                int32: self_.get(1).wire2api(),
                float64: self_.get(2).wire2api(),
                boolean: self_.get(3).wire2api(),
            },
            2 => KitchenSink::Nested(self_.get(1).wire2api(), self_.get(2).wire2api()),
            3 => KitchenSink::Optional(self_.get(1).wire2api(), self_.get(2).wire2api()),
            4 => KitchenSink::Buffer(self_.get(1).wire2api()),
            5 => KitchenSink::Enums(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Vec<ApplicationEnvVar>> for JsValue {
    fn wire2api(self) -> Vec<ApplicationEnvVar> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Attribute>> for JsValue {
    fn wire2api(self) -> Vec<Attribute> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<MySize>> for JsValue {
    fn wire2api(self) -> Vec<MySize> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<MyTreeNode>> for JsValue {
    fn wire2api(self) -> Vec<MyTreeNode> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<Attribute>>> for JsValue {
    fn wire2api(self) -> Vec<Option<Attribute>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Measure> for JsValue {
    fn wire2api(self) -> Measure {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Measure::Speed(self_.get(1).wire2api()),
            1 => Measure::Distance(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<MySize> for JsValue {
    fn wire2api(self) -> MySize {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        MySize {
            width: self_.get(0).wire2api(),
            height: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<MyStruct> for JsValue {
    fn wire2api(self) -> MyStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        MyStruct {
            content: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<MyTreeNode> for JsValue {
    fn wire2api(self) -> MyTreeNode {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        MyTreeNode {
            value_i32: self_.get(0).wire2api(),
            value_vec_u8: self_.get(1).wire2api(),
            value_boolean: self_.get(2).wire2api(),
            children: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<NewTypeInt> for JsValue {
    fn wire2api(self) -> NewTypeInt {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        NewTypeInt(self_.get(0).wire2api())
    }
}
impl Wire2Api<Note> for JsValue {
    fn wire2api(self) -> Note {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        Note {
            day: self_.get(0).wire2api(),
            body: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<Numbers> for JsValue {
    fn wire2api(self) -> Numbers {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        Numbers(self_.get(0).wire2api())
    }
}
impl Wire2Api<Option<String>> for Option<String> {
    fn wire2api(self) -> Option<String> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<ZeroCopyBuffer<Vec<u8>>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Option<ZeroCopyBuffer<Vec<u8>>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Attribute>> for JsValue {
    fn wire2api(self) -> Option<Attribute> {
        (!self.is_null() && !self.is_undefined()).then(|| Wire2Api::wire2api(self))
    }
}
impl Wire2Api<Option<bool>> for Option<Pointer<bool>> {
    fn wire2api(self) -> Option<bool> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<ExoticOptionals>> for JsValue {
    fn wire2api(self) -> Option<ExoticOptionals> {
        (!self.is_null() && !self.is_undefined()).then(|| Wire2Api::wire2api(self))
    }
}
impl Wire2Api<Option<f64>> for Option<Pointer<f64>> {
    fn wire2api(self) -> Option<f64> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<i32>> for Option<Pointer<i32>> {
    fn wire2api(self) -> Option<i32> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<i64>> for Option<Pointer<i64>> {
    fn wire2api(self) -> Option<i64> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<NewTypeInt>> for JsValue {
    fn wire2api(self) -> Option<NewTypeInt> {
        (!self.is_null() && !self.is_undefined()).then(|| Wire2Api::wire2api(self))
    }
}
impl Wire2Api<Option<Box<bool>>> for Option<Pointer<bool>> {
    fn wire2api(self) -> Option<Box<bool>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Box<ExoticOptionals>>> for JsValue {
    fn wire2api(self) -> Option<Box<ExoticOptionals>> {
        (!self.is_null() && !self.is_undefined()).then(|| Wire2Api::wire2api(self))
    }
}
impl Wire2Api<Option<Box<f64>>> for Option<Pointer<f64>> {
    fn wire2api(self) -> Option<Box<f64>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Box<i32>>> for Option<Pointer<i32>> {
    fn wire2api(self) -> Option<Box<i32>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Box<i64>>> for Option<Pointer<i64>> {
    fn wire2api(self) -> Option<Box<i64>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Box<i8>>> for Option<Pointer<i8>> {
    fn wire2api(self) -> Option<Box<i8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Box<u8>>> for Option<Pointer<u8>> {
    fn wire2api(self) -> Option<Box<u8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<f32>>> for Option<Box<[f32]>> {
    fn wire2api(self) -> Option<Vec<f32>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<f64>>> for Option<Box<[f64]>> {
    fn wire2api(self) -> Option<Vec<f64>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<i32>>> for Option<Box<[i32]>> {
    fn wire2api(self) -> Option<Vec<i32>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<i8>>> for Option<Box<[i8]>> {
    fn wire2api(self) -> Option<Vec<i8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<Attribute>>> for JsValue {
    fn wire2api(self) -> Option<Vec<Attribute>> {
        (!self.is_null() && !self.is_undefined()).then(|| Wire2Api::wire2api(self))
    }
}
impl Wire2Api<Option<Vec<Option<Attribute>>>> for JsValue {
    fn wire2api(self) -> Option<Vec<Option<Attribute>>> {
        (!self.is_null() && !self.is_undefined()).then(|| Wire2Api::wire2api(self))
    }
}
impl Wire2Api<Option<Vec<u8>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Option<Vec<u8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Sequences> for JsValue {
    fn wire2api(self) -> Sequences {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        Sequences(self_.get(0).wire2api())
    }
}
impl Wire2Api<Speed> for JsValue {
    fn wire2api(self) -> Speed {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Speed::Unknown,
            1 => Speed::GPS(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<SumWith> for JsValue {
    fn wire2api(self) -> SumWith {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        SumWith {
            x: self_.get(0).wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl Wire2Api<UserId> for JsValue {
    fn wire2api(self) -> UserId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        debug_assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        UserId {
            value: self_.get(0).wire2api(),
        }
    }
}

// Section: impl Wire2Api for JsValue

impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for JsValue {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
        ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<ApplicationMode> for JsValue {
    fn wire2api(self) -> ApplicationMode {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<Box<ApplicationEnv>> for JsValue {
    fn wire2api(self) -> Box<ApplicationEnv> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<bool>> for JsValue {
    fn wire2api(self) -> Box<bool> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<Distance>> for JsValue {
    fn wire2api(self) -> Box<Distance> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<ExoticOptionals>> for JsValue {
    fn wire2api(self) -> Box<ExoticOptionals> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<f64>> for JsValue {
    fn wire2api(self) -> Box<f64> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<i32>> for JsValue {
    fn wire2api(self) -> Box<i32> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<i64>> for JsValue {
    fn wire2api(self) -> Box<i64> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<i8>> for JsValue {
    fn wire2api(self) -> Box<i8> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<KitchenSink>> for JsValue {
    fn wire2api(self) -> Box<KitchenSink> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<MySize>> for JsValue {
    fn wire2api(self) -> Box<MySize> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<Speed>> for JsValue {
    fn wire2api(self) -> Box<Speed> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<u8>> for JsValue {
    fn wire2api(self) -> Box<u8> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<Weekdays>> for JsValue {
    fn wire2api(self) -> Box<Weekdays> {
        Box::new(self.wire2api())
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
impl Wire2Api<Vec<f32>> for JsValue {
    fn wire2api(self) -> Vec<f32> {
        self.unchecked_into::<js_sys::Float32Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<f64>> for JsValue {
    fn wire2api(self) -> Vec<f64> {
        self.unchecked_into::<js_sys::Float64Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<i32>> for JsValue {
    fn wire2api(self) -> Vec<i32> {
        self.unchecked_into::<js_sys::Int32Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<i8>> for JsValue {
    fn wire2api(self) -> Vec<i8> {
        self.unchecked_into::<js_sys::Int8Array>().to_vec().into()
    }
}
impl Wire2Api<MyEnum> for JsValue {
    fn wire2api(self) -> MyEnum {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<Option<String>> for JsValue {
    fn wire2api(self) -> Option<String> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<ZeroCopyBuffer<Vec<u8>>>> for JsValue {
    fn wire2api(self) -> Option<ZeroCopyBuffer<Vec<u8>>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<bool>> for JsValue {
    fn wire2api(self) -> Option<bool> {
        if !self.is_null() && !self.is_undefined() {
            Pointer::from_js(&self).map(Wire2Api::wire2api)
        } else {
            None
        }
    }
}
impl Wire2Api<Option<f64>> for JsValue {
    fn wire2api(self) -> Option<f64> {
        if !self.is_null() && !self.is_undefined() {
            Pointer::from_js(&self).map(Wire2Api::wire2api)
        } else {
            None
        }
    }
}
impl Wire2Api<Option<i32>> for JsValue {
    fn wire2api(self) -> Option<i32> {
        if !self.is_null() && !self.is_undefined() {
            Pointer::from_js(&self).map(Wire2Api::wire2api)
        } else {
            None
        }
    }
}
impl Wire2Api<Option<i64>> for JsValue {
    fn wire2api(self) -> Option<i64> {
        if !self.is_null() && !self.is_undefined() {
            Pointer::from_js(&self).map(Wire2Api::wire2api)
        } else {
            None
        }
    }
}
impl Wire2Api<Option<Box<bool>>> for JsValue {
    fn wire2api(self) -> Option<Box<bool>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<f64>>> for JsValue {
    fn wire2api(self) -> Option<Box<f64>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<i32>>> for JsValue {
    fn wire2api(self) -> Option<Box<i32>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<i64>>> for JsValue {
    fn wire2api(self) -> Option<Box<i64>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<i8>>> for JsValue {
    fn wire2api(self) -> Option<Box<i8>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Box<u8>>> for JsValue {
    fn wire2api(self) -> Option<Box<u8>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<f32>>> for JsValue {
    fn wire2api(self) -> Option<Vec<f32>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<f64>>> for JsValue {
    fn wire2api(self) -> Option<Vec<f64>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<i32>>> for JsValue {
    fn wire2api(self) -> Option<Vec<i32>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<i8>>> for JsValue {
    fn wire2api(self) -> Option<Vec<i8>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<u8>>> for JsValue {
    fn wire2api(self) -> Option<Vec<u8>> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
impl Wire2Api<usize> for JsValue {
    fn wire2api(self) -> usize {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Weekdays> for JsValue {
    fn wire2api(self) -> Weekdays {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
