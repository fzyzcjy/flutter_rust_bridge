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
pub fn wire_handle_sync_bool(input: bool) -> support::WireSyncReturnStruct {
    wire_handle_sync_bool_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_u8(input: u8) -> support::WireSyncReturnStruct {
    wire_handle_sync_u8_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_u16(input: u16) -> support::WireSyncReturnStruct {
    wire_handle_sync_u16_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_u32(input: u32) -> support::WireSyncReturnStruct {
    wire_handle_sync_u32_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_u64(input: u64) -> support::WireSyncReturnStruct {
    wire_handle_sync_u64_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_i8(input: i8) -> support::WireSyncReturnStruct {
    wire_handle_sync_i8_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_i16(input: i16) -> support::WireSyncReturnStruct {
    wire_handle_sync_i16_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_i32(input: i32) -> support::WireSyncReturnStruct {
    wire_handle_sync_i32_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_i64(input: i64) -> support::WireSyncReturnStruct {
    wire_handle_sync_i64_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_f32(input: f32) -> support::WireSyncReturnStruct {
    wire_handle_sync_f32_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_f64(input: f64) -> support::WireSyncReturnStruct {
    wire_handle_sync_f64_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_string(input: String) -> support::WireSyncReturnStruct {
    wire_handle_sync_string_impl(input)
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
pub fn wire_handle_increment_boxed_optional(port_: MessagePort, opt: *mut f64) {
    wire_handle_increment_boxed_optional_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_handle_option_box_arguments(
    port_: MessagePort,
    i8box: *mut i8,
    u8box: *mut u8,
    i32box: *mut i32,
    i64box: *mut i64,
    f64box: *mut f64,
    boolbox: *mut bool,
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
pub fn wire_get_sum_array(port_: MessagePort, a: u32, b: u32, c: u32) {
    wire_get_sum_array_impl(port_, a, b, c)
}

#[wasm_bindgen]
pub fn wire_multiply_by_ten(port_: MessagePort, measure: JsValue) {
    wire_multiply_by_ten_impl(port_, measure)
}

#[wasm_bindgen]
pub fn wire_call_old_module_system(port_: MessagePort) {
    wire_call_old_module_system_impl(port_)
}

#[wasm_bindgen]
pub fn wire_call_new_module_system(port_: MessagePort) {
    wire_call_new_module_system_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_big_buffers(port_: MessagePort) {
    wire_handle_big_buffers_impl(port_)
}

#[wasm_bindgen]
pub fn wire_datetime_utc(port_: MessagePort, d: i64) {
    wire_datetime_utc_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_datetime_local(port_: MessagePort, d: i64) {
    wire_datetime_local_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_naivedatetime(port_: MessagePort, d: i64) {
    wire_naivedatetime_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_duration(port_: MessagePort, d: i64) {
    wire_duration_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_how_long_does_it_take(port_: MessagePort, mine: JsValue) {
    wire_how_long_does_it_take_impl(port_, mine)
}

#[wasm_bindgen]
pub fn wire_handle_uuid(port_: MessagePort, id: Box<[u8]>) {
    wire_handle_uuid_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_handle_uuids(port_: MessagePort, ids: Box<[u8]>) {
    wire_handle_uuids_impl(port_, ids)
}

#[wasm_bindgen]
pub fn wire_handle_nested_uuids(port_: MessagePort, ids: JsValue) {
    wire_handle_nested_uuids_impl(port_, ids)
}

#[wasm_bindgen]
pub fn wire_new_msgid(port_: MessagePort, id: Box<[u8]>) {
    wire_new_msgid_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_use_msgid(port_: MessagePort, id: JsValue) {
    wire_use_msgid_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_boxed_blob(port_: MessagePort, blob: Box<[u8]>) {
    wire_boxed_blob_impl(port_, blob)
}

#[wasm_bindgen]
pub fn wire_use_boxed_blob(port_: MessagePort, blob: JsValue) {
    wire_use_boxed_blob_impl(port_, blob)
}

#[wasm_bindgen]
pub fn wire_return_boxed_feed_id(port_: MessagePort, id: Box<[u8]>) {
    wire_return_boxed_feed_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_return_boxed_raw_feed_id(port_: MessagePort, id: JsValue) {
    wire_return_boxed_raw_feed_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_test_id(port_: MessagePort, id: JsValue) {
    wire_test_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_last_number(port_: MessagePort, array: Box<[f64]>) {
    wire_last_number_impl(port_, array)
}

#[wasm_bindgen]
pub fn wire_nested_id(port_: MessagePort, id: JsValue) {
    wire_nested_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_create_opaque(port_: MessagePort) {
    wire_create_opaque_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_array_opaque_enum(port_: MessagePort) {
    wire_create_array_opaque_enum_impl(port_)
}

#[wasm_bindgen]
pub fn wire_run_enum_opaque(port_: MessagePort, opaque: JsValue) {
    wire_run_enum_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_run_opaque(port_: MessagePort, opaque: JsValue) {
    wire_run_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_run_opaque_with_delay(port_: MessagePort, opaque: JsValue) {
    wire_run_opaque_with_delay_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_opaque_array(port_: MessagePort) {
    wire_opaque_array_impl(port_)
}

#[wasm_bindgen]
pub fn wire_opaque_array_run(port_: MessagePort, data: JsValue) {
    wire_opaque_array_run_impl(port_, data)
}

#[wasm_bindgen]
pub fn wire_opaque_vec(port_: MessagePort) {
    wire_opaque_vec_impl(port_)
}

#[wasm_bindgen]
pub fn wire_opaque_vec_run(port_: MessagePort, data: JsValue) {
    wire_opaque_vec_run_impl(port_, data)
}

#[wasm_bindgen]
pub fn wire_create_nested_opaque(port_: MessagePort) {
    wire_create_nested_opaque_impl(port_)
}

#[wasm_bindgen]
pub fn wire_run_nested_opaque(port_: MessagePort, opaque: JsValue) {
    wire_run_nested_opaque_impl(port_, opaque)
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

// Section: related functions

#[wasm_bindgen]
pub fn drop_opaque_BoxDartDebug(ptr: *const c_void) {
    unsafe {
        Arc::<Box<dyn DartDebug>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_BoxDartDebug(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Box<dyn DartDebug>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_HideData(ptr: *const c_void) {
    unsafe {
        Arc::<HideData>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_HideData(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<HideData>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_I32(ptr: *const c_void) {
    unsafe {
        Arc::<i32>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_I32(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<i32>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_MutexHideData(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_MutexHideData(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RwLockHideData(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockHideData(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> Result<chrono::Duration, &'static str> {
        Ok(chrono::Duration::milliseconds(self))
    }
}
impl Wire2Api<chrono::DateTime<chrono::Local>> for i64 {
    fn wire2api(self) -> Result<chrono::DateTime<chrono::Local>, &'static str> {
        let Timestamp { s, ns } = wire2api_timestamp(self);
        Ok(chrono::DateTime::<chrono::Local>::from(chrono::DateTime::<
            chrono::Utc,
        >::from_utc(
            chrono::NaiveDateTime::from_timestamp(s, ns),
            chrono::Utc,
        )))
    }
}
impl Wire2Api<chrono::NaiveDateTime> for i64 {
    fn wire2api(self) -> Result<chrono::NaiveDateTime, &'static str> {
        let Timestamp { s, ns } = wire2api_timestamp(self);
        Ok(chrono::NaiveDateTime::from_timestamp(s, ns))
    }
}
impl Wire2Api<chrono::DateTime<chrono::Utc>> for i64 {
    fn wire2api(self) -> Result<chrono::DateTime<chrono::Utc>, &'static str> {
        let Timestamp { s, ns } = wire2api_timestamp(self);
        Ok(chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(s, ns),
            chrono::Utc,
        ))
    }
}

impl Wire2Api<String> for String {
    fn wire2api(self) -> Result<String, &'static str> {
        Ok(self)
    }
}
impl Wire2Api<Vec<String>> for JsValue {
    fn wire2api(self) -> Result<Vec<String>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}

impl Wire2Api<uuid::Uuid> for Box<[u8]> {
    fn wire2api(self) -> Result<uuid::Uuid, &'static str> {
        let single: Vec<u8> = self.wire2api()?;
        Ok(wire2api_uuid_ref(single.as_slice()))
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for Box<[u8]> {
    fn wire2api(self) -> Result<Vec<uuid::Uuid>, &'static str> {
        let multiple: Vec<u8> = self.wire2api()?;
        Ok(wire2api_uuids(multiple))
    }
}
impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for Box<[u8]> {
    fn wire2api(self) -> Result<ZeroCopyBuffer<Vec<u8>>, &'static str> {
        Ok(ZeroCopyBuffer(self.wire2api()?))
    }
}
impl Wire2Api<ApplicationEnv> for JsValue {
    fn wire2api(self) -> Result<ApplicationEnv, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let vars = self_.get(0).wire2api();
        Ok(ApplicationEnv { vars: vars? })
    }
}
impl Wire2Api<ApplicationEnvVar> for JsValue {
    fn wire2api(self) -> Result<ApplicationEnvVar, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let field0 = self_.get(0).wire2api();
        let field1 = self_.get(1).wire2api();
        Ok(ApplicationEnvVar(field0?, field1?))
    }
}

impl Wire2Api<ApplicationSettings> for JsValue {
    fn wire2api(self) -> Result<ApplicationSettings, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        let name = self_.get(0).wire2api();
        let version = self_.get(1).wire2api();
        let mode = self_.get(2).wire2api();
        let env = self_.get(3).wire2api();
        Ok(ApplicationSettings {
            name: name?,
            version: version?,
            mode: mode?,
            env: env?,
        })
    }
}
impl Wire2Api<Attribute> for JsValue {
    fn wire2api(self) -> Result<Attribute, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let key = self_.get(0).wire2api();
        let value = self_.get(1).wire2api();
        Ok(Attribute {
            key: key?,
            value: value?,
        })
    }
}
impl Wire2Api<Blob> for JsValue {
    fn wire2api(self) -> Result<Blob, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let field0 = self_.get(0).wire2api();
        Ok(Blob(field0?))
    }
}

impl Wire2Api<Box<[u8; 1600]>> for Box<[u8]> {
    fn wire2api(self) -> Result<Box<[u8; 1600]>, &'static str> {
        Wire2Api::<[u8; 1600]>::wire2api(self).map(Into::into)
    }
}

impl Wire2Api<ConcatenateWith> for JsValue {
    fn wire2api(self) -> Result<ConcatenateWith, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let a = self_.get(0).wire2api();
        Ok(ConcatenateWith { a: a? })
    }
}
impl Wire2Api<Customized> for JsValue {
    fn wire2api(self) -> Result<Customized, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let final_field = self_.get(0).wire2api();
        let non_final_field = self_.get(1).wire2api();
        Ok(Customized {
            final_field: final_field?,
            non_final_field: non_final_field?,
        })
    }
}
impl Wire2Api<Distance> for JsValue {
    fn wire2api(self) -> Result<Distance, &'static str> {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Ok(Distance::Unknown),
            1 => {
                let field0 = self_.get(1).wire2api();
                Ok(Distance::Map(field0?))
            }
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumOpaque> for JsValue {
    fn wire2api(self) -> Result<EnumOpaque, &'static str> {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => {
                let field0 = self_.get(1).wire2api();
                Ok(EnumOpaque::Struct(field0?))
            }
            1 => {
                let field0 = self_.get(1).wire2api();
                Ok(EnumOpaque::Primitive(field0?))
            }
            2 => {
                let field0 = self_.get(1).wire2api();
                Ok(EnumOpaque::TraitObj(field0?))
            }
            3 => {
                let field0 = self_.get(1).wire2api();
                Ok(EnumOpaque::Mutex(field0?))
            }
            4 => {
                let field0 = self_.get(1).wire2api();
                Ok(EnumOpaque::RwLock(field0?))
            }
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<ExoticOptionals> for JsValue {
    fn wire2api(self) -> Result<ExoticOptionals, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            14,
            "Expected 14 elements, got {}",
            self_.length()
        );
        let int32 = self_.get(0).wire2api();
        let int64 = self_.get(1).wire2api();
        let float64 = self_.get(2).wire2api();
        let boolean = self_.get(3).wire2api();
        let zerocopy = self_.get(4).wire2api();
        let int8list = self_.get(5).wire2api();
        let uint8list = self_.get(6).wire2api();
        let int32list = self_.get(7).wire2api();
        let float32list = self_.get(8).wire2api();
        let float64list = self_.get(9).wire2api();
        let attributes = self_.get(10).wire2api();
        let attributes_nullable = self_.get(11).wire2api();
        let nullable_attributes = self_.get(12).wire2api();
        let newtypeint = self_.get(13).wire2api();
        Ok(ExoticOptionals {
            int32: int32?,
            int64: int64?,
            float64: float64?,
            boolean: boolean?,
            zerocopy: zerocopy?,
            int8list: int8list?,
            uint8list: uint8list?,
            int32list: int32list?,
            float32list: float32list?,
            float64list: float64list?,
            attributes: attributes?,
            attributes_nullable: attributes_nullable?,
            nullable_attributes: nullable_attributes?,
            newtypeint: newtypeint?,
        })
    }
}

impl Wire2Api<[f64; 16]> for Box<[f64]> {
    fn wire2api(self) -> Result<[f64; 16], &'static str> {
        let vec: Vec<f64> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<FeatureChrono> for JsValue {
    fn wire2api(self) -> Result<FeatureChrono, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        let utc = self_.get(0).wire2api();
        let local = self_.get(1).wire2api();
        let duration = self_.get(2).wire2api();
        let naive = self_.get(3).wire2api();
        Ok(FeatureChrono {
            utc: utc?,
            local: local?,
            duration: duration?,
            naive: naive?,
        })
    }
}
impl Wire2Api<FeatureUuid> for JsValue {
    fn wire2api(self) -> Result<FeatureUuid, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let one = self_.get(0).wire2api();
        let many = self_.get(1).wire2api();
        Ok(FeatureUuid {
            one: one?,
            many: many?,
        })
    }
}
impl Wire2Api<FeedId> for JsValue {
    fn wire2api(self) -> Result<FeedId, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let field0 = self_.get(0).wire2api();
        Ok(FeedId(field0?))
    }
}
impl Wire2Api<Vec<f32>> for Box<[f32]> {
    fn wire2api(self) -> Result<Vec<f32>, &'static str> {
        Ok(self.into_vec())
    }
}
impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Result<Vec<f64>, &'static str> {
        Ok(self.into_vec())
    }
}

impl Wire2Api<[i32; 2]> for Box<[i32]> {
    fn wire2api(self) -> Result<[i32; 2], &'static str> {
        let vec: Vec<i32> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}

impl Wire2Api<Vec<i32>> for Box<[i32]> {
    fn wire2api(self) -> Result<Vec<i32>, &'static str> {
        Ok(self.into_vec())
    }
}
impl Wire2Api<Vec<i8>> for Box<[i8]> {
    fn wire2api(self) -> Result<Vec<i8>, &'static str> {
        Ok(self.into_vec())
    }
}
impl Wire2Api<KitchenSink> for JsValue {
    fn wire2api(self) -> Result<KitchenSink, &'static str> {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Ok(KitchenSink::Empty),
            1 => {
                let int32 = self_.get(1).wire2api();
                let float64 = self_.get(2).wire2api();
                let boolean = self_.get(3).wire2api();
                Ok(KitchenSink::Primitives {
                    int32: int32?,
                    float64: float64?,
                    boolean: boolean?,
                })
            }
            2 => {
                let field0 = self_.get(1).wire2api();
                let field1 = self_.get(2).wire2api();
                Ok(KitchenSink::Nested(field0?, field1?))
            }
            3 => {
                let field0 = self_.get(1).wire2api();
                let field1 = self_.get(2).wire2api();
                Ok(KitchenSink::Optional(field0?, field1?))
            }
            4 => {
                let field0 = self_.get(1).wire2api();
                Ok(KitchenSink::Buffer(field0?))
            }
            5 => {
                let field0 = self_.get(1).wire2api();
                Ok(KitchenSink::Enums(field0?))
            }
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Vec<Opaque<HideData>>> for JsValue {
    fn wire2api(self) -> Result<Vec<Opaque<HideData>>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<ApplicationEnvVar>> for JsValue {
    fn wire2api(self) -> Result<Vec<ApplicationEnvVar>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Attribute>> for JsValue {
    fn wire2api(self) -> Result<Vec<Attribute>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<MySize>> for JsValue {
    fn wire2api(self) -> Result<Vec<MySize>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<MyTreeNode>> for JsValue {
    fn wire2api(self) -> Result<Vec<MyTreeNode>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<Attribute>>> for JsValue {
    fn wire2api(self) -> Result<Vec<Option<Attribute>>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<TestId>> for JsValue {
    fn wire2api(self) -> Result<Vec<TestId>, &'static str> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Measure> for JsValue {
    fn wire2api(self) -> Result<Measure, &'static str> {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => {
                let field0 = self_.get(1).wire2api();
                Ok(Measure::Speed(field0?))
            }
            1 => {
                let field0 = self_.get(1).wire2api();
                Ok(Measure::Distance(field0?))
            }
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<MessageId> for JsValue {
    fn wire2api(self) -> Result<MessageId, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let field0 = self_.get(0).wire2api();
        Ok(MessageId(field0?))
    }
}

impl Wire2Api<MySize> for JsValue {
    fn wire2api(self) -> Result<MySize, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let width = self_.get(0).wire2api();
        let height = self_.get(1).wire2api();
        Ok(MySize {
            width: width?,
            height: height?,
        })
    }
}
impl Wire2Api<MyStruct> for JsValue {
    fn wire2api(self) -> Result<MyStruct, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let content = self_.get(0).wire2api();
        Ok(MyStruct { content: content? })
    }
}
impl Wire2Api<MyTreeNode> for JsValue {
    fn wire2api(self) -> Result<MyTreeNode, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        let value_i32 = self_.get(0).wire2api();
        let value_vec_u8 = self_.get(1).wire2api();
        let value_boolean = self_.get(2).wire2api();
        let children = self_.get(3).wire2api();
        Ok(MyTreeNode {
            value_i32: value_i32?,
            value_vec_u8: value_vec_u8?,
            value_boolean: value_boolean?,
            children: children?,
        })
    }
}
impl Wire2Api<NewTypeInt> for JsValue {
    fn wire2api(self) -> Result<NewTypeInt, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let field0 = self_.get(0).wire2api();
        Ok(NewTypeInt(field0?))
    }
}
impl Wire2Api<Note> for JsValue {
    fn wire2api(self) -> Result<Note, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let day = self_.get(0).wire2api();
        let body = self_.get(1).wire2api();
        Ok(Note {
            day: day?,
            body: body?,
        })
    }
}
impl Wire2Api<Numbers> for JsValue {
    fn wire2api(self) -> Result<Numbers, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let field0 = self_.get(0).wire2api();
        Ok(Numbers(field0?))
    }
}
impl Wire2Api<OpaqueNested> for JsValue {
    fn wire2api(self) -> Result<OpaqueNested, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        let first = self_.get(0).wire2api();
        let second = self_.get(1).wire2api();
        Ok(OpaqueNested {
            first: first?,
            second: second?,
        })
    }
}
impl Wire2Api<Option<String>> for Option<String> {
    fn wire2api(self) -> Result<Option<String>, &'static str> {
        self.map(Wire2Api::wire2api)
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<ZeroCopyBuffer<Vec<u8>>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Result<Option<ZeroCopyBuffer<Vec<u8>>>, &'static str> {
        self.map(Wire2Api::wire2api)
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Attribute>> for JsValue {
    fn wire2api(self) -> Result<Option<Attribute>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}

impl Wire2Api<Option<ExoticOptionals>> for JsValue {
    fn wire2api(self) -> Result<Option<ExoticOptionals>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}

impl Wire2Api<Option<NewTypeInt>> for JsValue {
    fn wire2api(self) -> Result<Option<NewTypeInt>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}

impl Wire2Api<Option<Box<ExoticOptionals>>> for JsValue {
    fn wire2api(self) -> Result<Option<Box<ExoticOptionals>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}

impl Wire2Api<Option<Vec<f32>>> for Option<Box<[f32]>> {
    fn wire2api(self) -> Result<Option<Vec<f32>>, &'static str> {
        self.map(Wire2Api::wire2api)
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<f64>>> for Option<Box<[f64]>> {
    fn wire2api(self) -> Result<Option<Vec<f64>>, &'static str> {
        self.map(Wire2Api::wire2api)
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<i32>>> for Option<Box<[i32]>> {
    fn wire2api(self) -> Result<Option<Vec<i32>>, &'static str> {
        self.map(Wire2Api::wire2api)
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<i8>>> for Option<Box<[i8]>> {
    fn wire2api(self) -> Result<Option<Vec<i8>>, &'static str> {
        self.map(Wire2Api::wire2api)
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<Attribute>>> for JsValue {
    fn wire2api(self) -> Result<Option<Vec<Attribute>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<Option<Attribute>>>> for JsValue {
    fn wire2api(self) -> Result<Option<Vec<Option<Attribute>>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<u8>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Result<Option<Vec<u8>>, &'static str> {
        self.map(Wire2Api::wire2api)
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Sequences> for JsValue {
    fn wire2api(self) -> Result<Sequences, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let field0 = self_.get(0).wire2api();
        Ok(Sequences(field0?))
    }
}
impl Wire2Api<Speed> for JsValue {
    fn wire2api(self) -> Result<Speed, &'static str> {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Ok(Speed::Unknown),
            1 => {
                let field0 = self_.get(1).wire2api();
                Ok(Speed::GPS(field0?))
            }
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<SumWith> for JsValue {
    fn wire2api(self) -> Result<SumWith, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let x = self_.get(0).wire2api();
        Ok(SumWith { x: x? })
    }
}
impl Wire2Api<TestId> for JsValue {
    fn wire2api(self) -> Result<TestId, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let field0 = self_.get(0).wire2api();
        Ok(TestId(field0?))
    }
}

impl Wire2Api<[u8; 1600]> for Box<[u8]> {
    fn wire2api(self) -> Result<[u8; 1600], &'static str> {
        let vec: Vec<u8> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<[u8; 32]> for Box<[u8]> {
    fn wire2api(self) -> Result<[u8; 32], &'static str> {
        let vec: Vec<u8> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<[u8; 8]> for Box<[u8]> {
    fn wire2api(self) -> Result<[u8; 8], &'static str> {
        let vec: Vec<u8> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Result<Vec<u8>, &'static str> {
        Ok(self.into_vec())
    }
}
impl Wire2Api<UserId> for JsValue {
    fn wire2api(self) -> Result<UserId, &'static str> {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        let value = self_.get(0).wire2api();
        Ok(UserId { value: value? })
    }
}

// Section: impl Wire2Api for JsValue

impl Wire2Api<Opaque<Box<dyn DartDebug>>> for JsValue {
    fn wire2api(self) -> Result<Opaque<Box<dyn DartDebug>>, &'static str> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<chrono::Duration> for JsValue {
    fn wire2api(self) -> Result<chrono::Duration, &'static str> {
        Wire2Api::<i64>::wire2api(self)?.wire2api()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Local>> for JsValue {
    fn wire2api(self) -> Result<chrono::DateTime<chrono::Local>, &'static str> {
        Wire2Api::<i64>::wire2api(self)?.wire2api()
    }
}
impl Wire2Api<chrono::NaiveDateTime> for JsValue {
    fn wire2api(self) -> Result<chrono::NaiveDateTime, &'static str> {
        Wire2Api::<i64>::wire2api(self)?.wire2api()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Utc>> for JsValue {
    fn wire2api(self) -> Result<chrono::DateTime<chrono::Utc>, &'static str> {
        Wire2Api::<i64>::wire2api(self)?.wire2api()
    }
}
impl Wire2Api<Opaque<HideData>> for JsValue {
    fn wire2api(self) -> Result<Opaque<HideData>, &'static str> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<[Opaque<HideData>; 2]> for JsValue {
    fn wire2api(self) -> Result<[Opaque<HideData>; 2], &'static str> {
        let vec: Vec<Opaque<HideData>> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<Opaque<i32>> for JsValue {
    fn wire2api(self) -> Result<Opaque<i32>, &'static str> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<Opaque<Mutex<HideData>>> for JsValue {
    fn wire2api(self) -> Result<Opaque<Mutex<HideData>>, &'static str> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<Opaque<RwLock<HideData>>> for JsValue {
    fn wire2api(self) -> Result<Opaque<RwLock<HideData>>, &'static str> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> Result<String, &'static str> {
        Ok(self.as_string().expect("non-UTF-8 string, or not a string"))
    }
}
impl Wire2Api<[TestId; 4]> for JsValue {
    fn wire2api(self) -> Result<[TestId; 4], &'static str> {
        let vec: Vec<TestId> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<uuid::Uuid> for JsValue {
    fn wire2api(self) -> Result<uuid::Uuid, &'static str> {
        self.unchecked_into::<js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for JsValue {
    fn wire2api(self) -> Result<Vec<uuid::Uuid>, &'static str> {
        self.unchecked_into::<js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for JsValue {
    fn wire2api(self) -> Result<ZeroCopyBuffer<Vec<u8>>, &'static str> {
        Ok(ZeroCopyBuffer(self.wire2api()?))
    }
}
impl Wire2Api<ApplicationMode> for JsValue {
    fn wire2api(self) -> Result<ApplicationMode, &'static str> {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> Result<bool, &'static str> {
        Ok(self.is_truthy())
    }
}
impl Wire2Api<Box<ApplicationEnv>> for JsValue {
    fn wire2api(self) -> Result<Box<ApplicationEnv>, &'static str> {
        Ok(Box::new(self.wire2api()?))
    }
}
impl Wire2Api<Box<Blob>> for JsValue {
    fn wire2api(self) -> Result<Box<Blob>, &'static str> {
        Ok(Box::new(self.wire2api()?))
    }
}
impl Wire2Api<Box<bool>> for JsValue {
    fn wire2api(self) -> Result<Box<bool>, &'static str> {
        (self.unchecked_into_f64() as usize as *mut bool).wire2api()
    }
}
impl Wire2Api<Box<Distance>> for JsValue {
    fn wire2api(self) -> Result<Box<Distance>, &'static str> {
        Ok(Box::new(self.wire2api()?))
    }
}
impl Wire2Api<Box<ExoticOptionals>> for JsValue {
    fn wire2api(self) -> Result<Box<ExoticOptionals>, &'static str> {
        Ok(Box::new(self.wire2api()?))
    }
}
impl Wire2Api<Box<f64>> for JsValue {
    fn wire2api(self) -> Result<Box<f64>, &'static str> {
        (self.unchecked_into_f64() as usize as *mut f64).wire2api()
    }
}
impl Wire2Api<Box<i32>> for JsValue {
    fn wire2api(self) -> Result<Box<i32>, &'static str> {
        (self.unchecked_into_f64() as usize as *mut i32).wire2api()
    }
}
impl Wire2Api<Box<i64>> for JsValue {
    fn wire2api(self) -> Result<Box<i64>, &'static str> {
        (self.unchecked_into_f64() as usize as *mut i64).wire2api()
    }
}
impl Wire2Api<Box<i8>> for JsValue {
    fn wire2api(self) -> Result<Box<i8>, &'static str> {
        (self.unchecked_into_f64() as usize as *mut i8).wire2api()
    }
}
impl Wire2Api<Box<KitchenSink>> for JsValue {
    fn wire2api(self) -> Result<Box<KitchenSink>, &'static str> {
        Ok(Box::new(self.wire2api()?))
    }
}
impl Wire2Api<Box<MySize>> for JsValue {
    fn wire2api(self) -> Result<Box<MySize>, &'static str> {
        Ok(Box::new(self.wire2api()?))
    }
}
impl Wire2Api<Box<Speed>> for JsValue {
    fn wire2api(self) -> Result<Box<Speed>, &'static str> {
        Ok(Box::new(self.wire2api()?))
    }
}
impl Wire2Api<Box<u8>> for JsValue {
    fn wire2api(self) -> Result<Box<u8>, &'static str> {
        (self.unchecked_into_f64() as usize as *mut u8).wire2api()
    }
}
impl Wire2Api<Box<[u8; 1600]>> for JsValue {
    fn wire2api(self) -> Result<Box<[u8; 1600]>, &'static str> {
        let vec: Vec<u8> = self.wire2api()?;
        Ok(Box::new(support::from_vec_to_array(vec)))
    }
}
impl Wire2Api<Box<Weekdays>> for JsValue {
    fn wire2api(self) -> Result<Box<Weekdays>, &'static str> {
        Ok(Box::new(self.wire2api()?))
    }
}
impl Wire2Api<f32> for JsValue {
    fn wire2api(self) -> Result<f32, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> Result<f64, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<[f64; 16]> for JsValue {
    fn wire2api(self) -> Result<[f64; 16], &'static str> {
        let vec: Vec<f64> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<Vec<f32>> for JsValue {
    fn wire2api(self) -> Result<Vec<f32>, &'static str> {
        Ok(self
            .unchecked_into::<js_sys::Float32Array>()
            .to_vec()
            .into())
    }
}
impl Wire2Api<Vec<f64>> for JsValue {
    fn wire2api(self) -> Result<Vec<f64>, &'static str> {
        Ok(self
            .unchecked_into::<js_sys::Float64Array>()
            .to_vec()
            .into())
    }
}
impl Wire2Api<i16> for JsValue {
    fn wire2api(self) -> Result<i16, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> Result<i32, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<[i32; 2]> for JsValue {
    fn wire2api(self) -> Result<[i32; 2], &'static str> {
        let vec: Vec<i32> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> Result<i64, &'static str> {
        Ok(::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap())
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> Result<i8, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<Vec<i32>> for JsValue {
    fn wire2api(self) -> Result<Vec<i32>, &'static str> {
        Ok(self.unchecked_into::<js_sys::Int32Array>().to_vec().into())
    }
}
impl Wire2Api<Vec<i8>> for JsValue {
    fn wire2api(self) -> Result<Vec<i8>, &'static str> {
        Ok(self.unchecked_into::<js_sys::Int8Array>().to_vec().into())
    }
}
impl Wire2Api<MyEnum> for JsValue {
    fn wire2api(self) -> Result<MyEnum, &'static str> {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<Option<String>> for JsValue {
    fn wire2api(self) -> Result<Option<String>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<ZeroCopyBuffer<Vec<u8>>>> for JsValue {
    fn wire2api(self) -> Result<Option<ZeroCopyBuffer<Vec<u8>>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<bool>> for JsValue {
    fn wire2api(self) -> Result<Option<bool>, &'static str> {
        (self != 0)
            .then(|| Wire2Api::<Box<bool>>::wire2api(self).map(|v| *v))
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<f64>> for JsValue {
    fn wire2api(self) -> Result<Option<f64>, &'static str> {
        (self != 0)
            .then(|| Wire2Api::<Box<f64>>::wire2api(self).map(|v| *v))
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<i32>> for JsValue {
    fn wire2api(self) -> Result<Option<i32>, &'static str> {
        (self != 0)
            .then(|| Wire2Api::<Box<i32>>::wire2api(self).map(|v| *v))
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<i64>> for JsValue {
    fn wire2api(self) -> Result<Option<i64>, &'static str> {
        (self != 0)
            .then(|| Wire2Api::<Box<i64>>::wire2api(self).map(|v| *v))
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Box<bool>>> for JsValue {
    fn wire2api(self) -> Result<Option<Box<bool>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Box<f64>>> for JsValue {
    fn wire2api(self) -> Result<Option<Box<f64>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Box<i32>>> for JsValue {
    fn wire2api(self) -> Result<Option<Box<i32>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Box<i64>>> for JsValue {
    fn wire2api(self) -> Result<Option<Box<i64>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Box<i8>>> for JsValue {
    fn wire2api(self) -> Result<Option<Box<i8>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Box<u8>>> for JsValue {
    fn wire2api(self) -> Result<Option<Box<u8>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<f32>>> for JsValue {
    fn wire2api(self) -> Result<Option<Vec<f32>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<f64>>> for JsValue {
    fn wire2api(self) -> Result<Option<Vec<f64>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<i32>>> for JsValue {
    fn wire2api(self) -> Result<Option<Vec<i32>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<i8>>> for JsValue {
    fn wire2api(self) -> Result<Option<Vec<i8>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<Option<Vec<u8>>> for JsValue {
    fn wire2api(self) -> Result<Option<Vec<u8>>, &'static str> {
        (!self.is_undefined() && !self.is_null())
            .then(|| self.wire2api())
            .map_or(Ok(None), |v| v.map(Some))
    }
}
impl Wire2Api<u16> for JsValue {
    fn wire2api(self) -> Result<u16, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> Result<u32, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<u64> for JsValue {
    fn wire2api(self) -> Result<u64, &'static str> {
        Ok(::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap())
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> Result<u8, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<[u8; 1600]> for JsValue {
    fn wire2api(self) -> Result<[u8; 1600], &'static str> {
        let vec: Vec<u8> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<[u8; 32]> for JsValue {
    fn wire2api(self) -> Result<[u8; 32], &'static str> {
        let vec: Vec<u8> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<[u8; 8]> for JsValue {
    fn wire2api(self) -> Result<[u8; 8], &'static str> {
        let vec: Vec<u8> = self.wire2api()?;
        Ok(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Result<Vec<u8>, &'static str> {
        Ok(self.unchecked_into::<js_sys::Uint8Array>().to_vec().into())
    }
}
impl Wire2Api<usize> for JsValue {
    fn wire2api(self) -> Result<usize, &'static str> {
        Ok(self.unchecked_into_f64() as _)
    }
}
impl Wire2Api<Weekdays> for JsValue {
    fn wire2api(self) -> Result<Weekdays, &'static str> {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
