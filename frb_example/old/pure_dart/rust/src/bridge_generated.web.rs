use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_simple_adder(port_: MessagePort, a: i32, b: i32) {
    wire_simple_adder_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_simple_adder_sync(a: i32, b: i32) -> support::WireSyncReturn {
    wire_simple_adder_sync_impl(a, b)
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
pub fn wire_primitive_optional_types(
    port_: MessagePort,
    my_i32: JsValue,
    my_i64: JsValue,
    my_f64: JsValue,
    my_bool: JsValue,
) {
    wire_primitive_optional_types_impl(port_, my_i32, my_i64, my_f64, my_bool)
}

#[wasm_bindgen]
pub fn wire_primitive_types_sync(
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) -> support::WireSyncReturn {
    wire_primitive_types_sync_impl(my_i32, my_i64, my_f64, my_bool)
}

#[wasm_bindgen]
pub fn wire_primitive_u32(port_: MessagePort, my_u32: u32) {
    wire_primitive_u32_impl(port_, my_u32)
}

#[wasm_bindgen]
pub fn wire_primitive_u32_sync(my_u32: u32) -> support::WireSyncReturn {
    wire_primitive_u32_sync_impl(my_u32)
}

#[wasm_bindgen]
pub fn wire_handle_string(port_: MessagePort, s: String) {
    wire_handle_string_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_string_sync(s: String) -> support::WireSyncReturn {
    wire_handle_string_sync_impl(s)
}

#[wasm_bindgen]
pub fn wire_handle_return_unit(port_: MessagePort) {
    wire_handle_return_unit_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_return_unit_sync() -> support::WireSyncReturn {
    wire_handle_return_unit_sync_impl()
}

#[wasm_bindgen]
pub fn wire_handle_vec_u8(port_: MessagePort, v: Box<[u8]>) {
    wire_handle_vec_u8_impl(port_, v)
}

#[wasm_bindgen]
pub fn wire_handle_vec_u8_sync(v: Box<[u8]>) -> support::WireSyncReturn {
    wire_handle_vec_u8_sync_impl(v)
}

#[wasm_bindgen]
pub fn wire_handle_vec_of_primitive(port_: MessagePort, n: i32) {
    wire_handle_vec_of_primitive_impl(port_, n)
}

#[wasm_bindgen]
pub fn wire_handle_vec_of_primitive_sync(n: i32) -> support::WireSyncReturn {
    wire_handle_vec_of_primitive_sync_impl(n)
}

#[wasm_bindgen]
pub fn wire_handle_zero_copy_vec_of_primitive(port_: MessagePort, n: i32) {
    wire_handle_zero_copy_vec_of_primitive_impl(port_, n)
}

#[wasm_bindgen]
pub fn wire_handle_zero_copy_vec_of_primitive_sync(n: i32) -> support::WireSyncReturn {
    wire_handle_zero_copy_vec_of_primitive_sync_impl(n)
}

#[wasm_bindgen]
pub fn wire_handle_struct(port_: MessagePort, arg: JsValue, boxed: JsValue) {
    wire_handle_struct_impl(port_, arg, boxed)
}

#[wasm_bindgen]
pub fn wire_handle_struct_sync(arg: JsValue, boxed: JsValue) -> support::WireSyncReturn {
    wire_handle_struct_sync_impl(arg, boxed)
}

#[wasm_bindgen]
pub fn wire_handle_struct_sync_freezed(arg: JsValue, boxed: JsValue) -> support::WireSyncReturn {
    wire_handle_struct_sync_freezed_impl(arg, boxed)
}

#[wasm_bindgen]
pub fn wire_handle_newtype(port_: MessagePort, arg: JsValue) {
    wire_handle_newtype_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_newtype_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_handle_newtype_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_handle_list_of_struct(port_: MessagePort, l: JsValue) {
    wire_handle_list_of_struct_impl(port_, l)
}

#[wasm_bindgen]
pub fn wire_handle_list_of_struct_sync(l: JsValue) -> support::WireSyncReturn {
    wire_handle_list_of_struct_sync_impl(l)
}

#[wasm_bindgen]
pub fn wire_handle_string_list(port_: MessagePort, names: JsValue) {
    wire_handle_string_list_impl(port_, names)
}

#[wasm_bindgen]
pub fn wire_handle_string_list_sync(names: JsValue) -> support::WireSyncReturn {
    wire_handle_string_list_sync_impl(names)
}

#[wasm_bindgen]
pub fn wire_handle_complex_struct(port_: MessagePort, s: JsValue) {
    wire_handle_complex_struct_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_complex_struct_sync(s: JsValue) -> support::WireSyncReturn {
    wire_handle_complex_struct_sync_impl(s)
}

#[wasm_bindgen]
pub fn wire_handle_nested_struct(port_: MessagePort, s: JsValue) {
    wire_handle_nested_struct_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_sync_return(mode: String) -> support::WireSyncReturn {
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
pub fn wire_handle_increment_boxed_optional(port_: MessagePort, opt: JsValue) {
    wire_handle_increment_boxed_optional_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_handle_vec_of_opts(port_: MessagePort, opt: JsValue) {
    wire_handle_vec_of_opts_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_handle_option_box_arguments(
    port_: MessagePort,
    i8box: JsValue,
    u8box: JsValue,
    i32box: JsValue,
    i64box: JsValue,
    f64box: JsValue,
    boolbox: JsValue,
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
pub fn wire_handle_enum_sync_freezed(value: JsValue) -> support::WireSyncReturn {
    wire_handle_enum_sync_freezed_impl(value)
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
pub fn wire_get_fallible_app_settings(port_: MessagePort) {
    wire_get_fallible_app_settings_impl(port_)
}

#[wasm_bindgen]
pub fn wire_is_app_embedded(port_: MessagePort, app_settings: JsValue) {
    wire_is_app_embedded_impl(port_, app_settings)
}

#[wasm_bindgen]
pub fn wire_app_settings_stream(port_: MessagePort) {
    wire_app_settings_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_app_settings_vec_stream(port_: MessagePort) {
    wire_app_settings_vec_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_mirror_struct_stream(port_: MessagePort) {
    wire_mirror_struct_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_mirror_tuple_stream(port_: MessagePort) {
    wire_mirror_tuple_stream_impl(port_)
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
pub fn wire_optional_empty_datetime_utc(port_: MessagePort, d: JsValue) {
    wire_optional_empty_datetime_utc_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_duration(port_: MessagePort, d: i64) {
    wire_duration_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_handle_timestamps(port_: MessagePort, timestamps: Box<[i64]>, epoch: i64) {
    wire_handle_timestamps_impl(port_, timestamps, epoch)
}

#[wasm_bindgen]
pub fn wire_handle_durations(port_: MessagePort, durations: Box<[i64]>, since: i64) {
    wire_handle_durations_impl(port_, durations, since)
}

#[wasm_bindgen]
pub fn wire_test_chrono(port_: MessagePort) {
    wire_test_chrono_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_precise_chrono(port_: MessagePort) {
    wire_test_precise_chrono_impl(port_)
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
pub fn wire_sync_accept_dart_opaque(opaque: JsValue) -> support::WireSyncReturn {
    wire_sync_accept_dart_opaque_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_async_accept_dart_opaque(port_: MessagePort, opaque: JsValue) {
    wire_async_accept_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back(port_: MessagePort, opaque: JsValue) {
    wire_loop_back_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_option(port_: MessagePort, opaque: JsValue) {
    wire_loop_back_option_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_array(port_: MessagePort, opaque: JsValue) {
    wire_loop_back_array_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_vec(port_: MessagePort, opaque: JsValue) {
    wire_loop_back_vec_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_option_get(port_: MessagePort, opaque: JsValue) {
    wire_loop_back_option_get_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_array_get(port_: MessagePort, opaque: JsValue) {
    wire_loop_back_array_get_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_vec_get(port_: MessagePort, opaque: JsValue) {
    wire_loop_back_vec_get_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_unwrap_dart_opaque(opaque: JsValue) -> support::WireSyncReturn {
    wire_unwrap_dart_opaque_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_panic_unwrap_dart_opaque(port_: MessagePort, opaque: JsValue) {
    wire_panic_unwrap_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_create_opaque(port_: MessagePort) {
    wire_create_opaque_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_option_opaque(port_: MessagePort, opaque: JsValue) {
    wire_create_option_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_sync_create_opaque() -> support::WireSyncReturn {
    wire_sync_create_opaque_impl()
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
pub fn wire_sync_create_non_clone() -> support::WireSyncReturn {
    wire_sync_create_non_clone_impl()
}

#[wasm_bindgen]
pub fn wire_run_non_clone(port_: MessagePort, clone: JsValue) {
    wire_run_non_clone_impl(port_, clone)
}

#[wasm_bindgen]
pub fn wire_create_sync_opaque(port_: MessagePort) {
    wire_create_sync_opaque_impl(port_)
}

#[wasm_bindgen]
pub fn wire_sync_create_sync_opaque() -> support::WireSyncReturn {
    wire_sync_create_sync_opaque_impl()
}

#[wasm_bindgen]
pub fn wire_sync_run_opaque(opaque: JsValue) -> support::WireSyncReturn {
    wire_sync_run_opaque_impl(opaque)
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
pub fn wire_sync_loopback(opaque: JsValue) -> support::WireSyncReturn {
    wire_sync_loopback_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_sync_option_loopback(opaque: JsValue) -> support::WireSyncReturn {
    wire_sync_option_loopback_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_sync_option() -> support::WireSyncReturn {
    wire_sync_option_impl()
}

#[wasm_bindgen]
pub fn wire_sync_option_null() -> support::WireSyncReturn {
    wire_sync_option_null_impl()
}

#[wasm_bindgen]
pub fn wire_sync_option_rust_opaque() -> support::WireSyncReturn {
    wire_sync_option_rust_opaque_impl()
}

#[wasm_bindgen]
pub fn wire_sync_option_dart_opaque(opaque: JsValue) -> support::WireSyncReturn {
    wire_sync_option_dart_opaque_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_sync_void() -> support::WireSyncReturn {
    wire_sync_void_impl()
}

#[wasm_bindgen]
pub fn wire_run_nested_opaque(port_: MessagePort, opaque: JsValue) {
    wire_run_nested_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_create_nested_dart_opaque(port_: MessagePort, opaque1: JsValue, opaque2: JsValue) {
    wire_create_nested_dart_opaque_impl(port_, opaque1, opaque2)
}

#[wasm_bindgen]
pub fn wire_get_nested_dart_opaque(port_: MessagePort, opaque: JsValue) {
    wire_get_nested_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_create_enum_dart_opaque(port_: MessagePort, opaque: JsValue) {
    wire_create_enum_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_get_enum_dart_opaque(port_: MessagePort, opaque: JsValue) {
    wire_get_enum_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_set_static_dart_opaque(port_: MessagePort, opaque: JsValue) {
    wire_set_static_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_drop_static_dart_opaque(port_: MessagePort) {
    wire_drop_static_dart_opaque_impl(port_)
}

#[wasm_bindgen]
pub fn wire_unwrap_rust_opaque(port_: MessagePort, opaque: JsValue) {
    wire_unwrap_rust_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_return_non_droppable_dart_opaque(opaque: JsValue) -> support::WireSyncReturn {
    wire_return_non_droppable_dart_opaque_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_frb_generator_test(port_: MessagePort) {
    wire_frb_generator_test_impl(port_)
}

#[wasm_bindgen]
pub fn wire_frb_sync_generator_test() -> support::WireSyncReturn {
    wire_frb_sync_generator_test_impl()
}

#[wasm_bindgen]
pub fn wire_handle_type_alias_id(port_: MessagePort, input: u64) {
    wire_handle_type_alias_id_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_type_nest_alias_id(port_: MessagePort, input: u64) {
    wire_handle_type_nest_alias_id_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_type_alias_model(port_: MessagePort, input: u64) {
    wire_handle_type_alias_model_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_empty_struct(port_: MessagePort, empty: JsValue) {
    wire_empty_struct_impl(port_, empty)
}

#[wasm_bindgen]
pub fn wire_return_dart_dynamic(port_: MessagePort) {
    wire_return_dart_dynamic_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_item_struct(port_: MessagePort) {
    wire_test_raw_string_item_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_more_than_just_one_raw_string_struct(port_: MessagePort) {
    wire_test_more_than_just_one_raw_string_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_mirrored(port_: MessagePort) {
    wire_test_raw_string_mirrored_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_nested_raw_string_mirrored(port_: MessagePort) {
    wire_test_nested_raw_string_mirrored_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_enum_mirrored(port_: MessagePort, nested: bool) {
    wire_test_raw_string_enum_mirrored_impl(port_, nested)
}

#[wasm_bindgen]
pub fn wire_test_list_of_raw_nested_string_mirrored(port_: MessagePort) {
    wire_test_list_of_raw_nested_string_mirrored_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_fallible_of_raw_string_mirrored(port_: MessagePort) {
    wire_test_fallible_of_raw_string_mirrored_impl(port_)
}

#[wasm_bindgen]
pub fn wire_list_of_primitive_enums(port_: MessagePort, weekdays: JsValue) {
    wire_list_of_primitive_enums_impl(port_, weekdays)
}

#[wasm_bindgen]
pub fn wire_test_abc_enum(port_: MessagePort, abc: JsValue) {
    wire_test_abc_enum_impl(port_, abc)
}

#[wasm_bindgen]
pub fn wire_test_contains_mirrored_sub_struct(port_: MessagePort) {
    wire_test_contains_mirrored_sub_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_struct_with_enum(port_: MessagePort, se: JsValue) {
    wire_test_struct_with_enum_impl(port_, se)
}

#[wasm_bindgen]
pub fn wire_test_tuple(port_: MessagePort, value: JsValue) {
    wire_test_tuple_impl(port_, value)
}

#[wasm_bindgen]
pub fn wire_test_tuple_2(port_: MessagePort, value: JsValue) {
    wire_test_tuple_2_impl(port_, value)
}

#[wasm_bindgen]
pub fn wire_sync_return_mirror() -> support::WireSyncReturn {
    wire_sync_return_mirror_impl()
}

#[wasm_bindgen]
pub fn wire_macro_struct(port_: MessagePort) {
    wire_macro_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_err_custom_error(port_: MessagePort) {
    wire_return_err_custom_error_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_ok_custom_error(port_: MessagePort) {
    wire_return_ok_custom_error_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_error_variant(port_: MessagePort, variant: u32) {
    wire_return_error_variant_impl(port_, variant)
}

#[wasm_bindgen]
pub fn wire_return_custom_nested_error_1(port_: MessagePort) {
    wire_return_custom_nested_error_1_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_custom_nested_error_1_variant1(port_: MessagePort) {
    wire_return_custom_nested_error_1_variant1_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_custom_nested_error_2(port_: MessagePort) {
    wire_return_custom_nested_error_2_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_custom_struct_error(port_: MessagePort) {
    wire_return_custom_struct_error_impl(port_)
}

#[wasm_bindgen]
pub fn wire_sync_return_custom_struct_error() -> support::WireSyncReturn {
    wire_sync_return_custom_struct_error_impl()
}

#[wasm_bindgen]
pub fn wire_return_custom_struct_ok(port_: MessagePort) {
    wire_return_custom_struct_ok_impl(port_)
}

#[wasm_bindgen]
pub fn wire_throw_anyhow(port_: MessagePort) {
    wire_throw_anyhow_impl(port_)
}

#[wasm_bindgen]
pub fn wire_panic_with_custom_result(port_: MessagePort) {
    wire_panic_with_custom_result_impl(port_)
}

#[wasm_bindgen]
pub fn wire_stream_sink_throw_anyhow(port_: MessagePort) {
    wire_stream_sink_throw_anyhow_impl(port_)
}

#[wasm_bindgen]
pub fn wire_as_string__method__Event(port_: MessagePort, that: JsValue) {
    wire_as_string__method__Event_impl(port_, that)
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

#[wasm_bindgen]
pub fn wire_new__static_method__SomeStruct(port_: MessagePort, value: u32) {
    wire_new__static_method__SomeStruct_impl(port_, value)
}

#[wasm_bindgen]
pub fn wire_static_return_err_custom_error__static_method__SomeStruct(port_: MessagePort) {
    wire_static_return_err_custom_error__static_method__SomeStruct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_static_return_ok_custom_error__static_method__SomeStruct(port_: MessagePort) {
    wire_static_return_ok_custom_error__static_method__SomeStruct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_non_static_return_err_custom_error__method__SomeStruct(
    port_: MessagePort,
    that: JsValue,
) {
    wire_non_static_return_err_custom_error__method__SomeStruct_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_non_static_return_ok_custom_error__method__SomeStruct(
    port_: MessagePort,
    that: JsValue,
) {
    wire_non_static_return_ok_custom_error__method__SomeStruct_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_new__static_method__CustomStruct(port_: MessagePort, message: String) {
    wire_new__static_method__CustomStruct_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_static_return_custom_struct_error__static_method__CustomStruct(port_: MessagePort) {
    wire_static_return_custom_struct_error__static_method__CustomStruct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_static_return_custom_struct_ok__static_method__CustomStruct(port_: MessagePort) {
    wire_static_return_custom_struct_ok__static_method__CustomStruct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_nonstatic_return_custom_struct_error__method__CustomStruct(
    port_: MessagePort,
    that: JsValue,
) {
    wire_nonstatic_return_custom_struct_error__method__CustomStruct_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_nonstatic_return_custom_struct_ok__method__CustomStruct(
    port_: MessagePort,
    that: JsValue,
) {
    wire_nonstatic_return_custom_struct_ok__method__CustomStruct_impl(port_, that)
}

// Section: allocate functions

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
pub fn drop_opaque_FrbOpaqueReturn(ptr: *const c_void) {
    unsafe {
        Arc::<FrbOpaqueReturn>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_FrbOpaqueReturn(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<FrbOpaqueReturn>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_FrbOpaqueSyncReturn(ptr: *const c_void) {
    unsafe {
        Arc::<FrbOpaqueSyncReturn>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_FrbOpaqueSyncReturn(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<FrbOpaqueSyncReturn>::increment_strong_count(ptr as _);
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
pub fn drop_opaque_NonCloneData(ptr: *const c_void) {
    unsafe {
        Arc::<NonCloneData>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_NonCloneData(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<NonCloneData>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_NonSendHideData(ptr: *const c_void) {
    unsafe {
        Arc::<NonSendHideData>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_NonSendHideData(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<NonSendHideData>::increment_strong_count(ptr as _);
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
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::milliseconds(self)
    }
}
impl Wire2Api<Vec<chrono::Duration>> for Box<[i64]> {
    fn wire2api(self) -> Vec<chrono::Duration> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Vec<chrono::NaiveDateTime>> for Box<[i64]> {
    fn wire2api(self) -> Vec<chrono::NaiveDateTime> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<DartOpaque> for JsValue {
    fn wire2api(self) -> DartOpaque {
        let arr = self.dyn_into::<JsArray>().unwrap();
        unsafe { DartOpaque::new(arr.get(0), arr.get(1)) }
    }
}

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

impl Wire2Api<uuid::Uuid> for Box<[u8]> {
    fn wire2api(self) -> uuid::Uuid {
        let single: Vec<u8> = self.wire2api();
        wire2api_uuid_ref(single.as_slice())
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for Box<[u8]> {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        wire2api_uuids(multiple)
    }
}
impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for Box<[u8]> {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
        ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<(String, i32)> for JsValue {
    fn wire2api(self) -> (String, i32) {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        (self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}
impl Wire2Api<A> for JsValue {
    fn wire2api(self) -> A {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        A {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<Abc> for JsValue {
    fn wire2api(self) -> Abc {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Abc::A(self_.get(1).wire2api()),
            1 => Abc::B(self_.get(1).wire2api()),
            2 => Abc::C(self_.get(1).wire2api()),
            3 => Abc::JustInt(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<ApplicationEnv> for JsValue {
    fn wire2api(self) -> ApplicationEnv {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
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
        assert_eq!(
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
        assert_eq!(
            self_.length(),
            5,
            "Expected 5 elements, got {}",
            self_.length()
        );
        ApplicationSettings {
            name: self_.get(0).wire2api(),
            version: self_.get(1).wire2api(),
            mode: self_.get(2).wire2api(),
            env: self_.get(3).wire2api(),
            env_optional: self_.get(4).wire2api(),
        }
    }
}
impl Wire2Api<Attribute> for JsValue {
    fn wire2api(self) -> Attribute {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
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
impl Wire2Api<B> for JsValue {
    fn wire2api(self) -> B {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        B {
            b: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<Blob> for JsValue {
    fn wire2api(self) -> Blob {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        Blob(self_.get(0).wire2api())
    }
}

impl Wire2Api<Box<[u8; 1600]>> for Box<[u8]> {
    fn wire2api(self) -> Box<[u8; 1600]> {
        Wire2Api::<[u8; 1600]>::wire2api(self).into()
    }
}

impl Wire2Api<C> for JsValue {
    fn wire2api(self) -> C {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        C {
            c: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<ConcatenateWith> for JsValue {
    fn wire2api(self) -> ConcatenateWith {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
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
impl Wire2Api<CustomStruct> for JsValue {
    fn wire2api(self) -> CustomStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        CustomStruct {
            message: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<Customized> for JsValue {
    fn wire2api(self) -> Customized {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
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
impl Wire2Api<DartOpaqueNested> for JsValue {
    fn wire2api(self) -> DartOpaqueNested {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        DartOpaqueNested {
            first: self_.get(0).wire2api(),
            second: self_.get(1).wire2api(),
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
impl Wire2Api<Empty> for JsValue {
    fn wire2api(self) -> Empty {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            0,
            "Expected 0 elements, got {}",
            self_.length()
        );
        Empty {}
    }
}
impl Wire2Api<EnumDartOpaque> for JsValue {
    fn wire2api(self) -> EnumDartOpaque {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumDartOpaque::Primitive(self_.get(1).wire2api()),
            1 => EnumDartOpaque::Opaque(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumOpaque> for JsValue {
    fn wire2api(self) -> EnumOpaque {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumOpaque::Struct(self_.get(1).wire2api()),
            1 => EnumOpaque::Primitive(self_.get(1).wire2api()),
            2 => EnumOpaque::TraitObj(self_.get(1).wire2api()),
            3 => EnumOpaque::Mutex(self_.get(1).wire2api()),
            4 => EnumOpaque::RwLock(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Event> for JsValue {
    fn wire2api(self) -> Event {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        Event {
            address: self_.get(0).wire2api(),
            payload: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<ExoticOptionals> for JsValue {
    fn wire2api(self) -> ExoticOptionals {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
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

impl Wire2Api<[f64; 16]> for Box<[f64]> {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<FeatureChrono> for JsValue {
    fn wire2api(self) -> FeatureChrono {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        FeatureChrono {
            utc: self_.get(0).wire2api(),
            local: self_.get(1).wire2api(),
            duration: self_.get(2).wire2api(),
            naive: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<FeatureUuid> for JsValue {
    fn wire2api(self) -> FeatureUuid {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        FeatureUuid {
            one: self_.get(0).wire2api(),
            many: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<FeedId> for JsValue {
    fn wire2api(self) -> FeedId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        FeedId(self_.get(0).wire2api())
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

impl Wire2Api<[i32; 2]> for Box<[i32]> {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}

impl Wire2Api<Vec<i32>> for Box<[i32]> {
    fn wire2api(self) -> Vec<i32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i64>> for Box<[i64]> {
    fn wire2api(self) -> Vec<i64> {
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
impl Wire2Api<Vec<DartOpaque>> for JsValue {
    fn wire2api(self) -> Vec<DartOpaque> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<RustOpaque<HideData>>> for JsValue {
    fn wire2api(self) -> Vec<RustOpaque<HideData>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<(String, i32)>> for JsValue {
    fn wire2api(self) -> Vec<(String, i32)> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
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
impl Wire2Api<Vec<Option<String>>> for JsValue {
    fn wire2api(self) -> Vec<Option<String>> {
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
impl Wire2Api<Vec<Option<i32>>> for JsValue {
    fn wire2api(self) -> Vec<Option<i32>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<Weekdays>>> for JsValue {
    fn wire2api(self) -> Vec<Option<Weekdays>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<Vec<i32>>>> for JsValue {
    fn wire2api(self) -> Vec<Option<Vec<i32>>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<TestId>> for JsValue {
    fn wire2api(self) -> Vec<TestId> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Weekdays>> for JsValue {
    fn wire2api(self) -> Vec<Weekdays> {
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
impl Wire2Api<MessageId> for JsValue {
    fn wire2api(self) -> MessageId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        MessageId(self_.get(0).wire2api())
    }
}

impl Wire2Api<MyEnumFreezed> for JsValue {
    fn wire2api(self) -> MyEnumFreezed {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => MyEnumFreezed::A(self_.get(1).wire2api()),
            1 => MyEnumFreezed::B(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<MyNestedStruct> for JsValue {
    fn wire2api(self) -> MyNestedStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        MyNestedStruct {
            tree_node: self_.get(0).wire2api(),
            weekday: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<MySize> for JsValue {
    fn wire2api(self) -> MySize {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
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
impl Wire2Api<MySizeFreezed> for JsValue {
    fn wire2api(self) -> MySizeFreezed {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        MySizeFreezed {
            width: self_.get(0).wire2api(),
            height: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<MyStruct> for JsValue {
    fn wire2api(self) -> MyStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
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
        assert_eq!(
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
        assert_eq!(
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
        assert_eq!(
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
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        Numbers(self_.get(0).wire2api())
    }
}
impl Wire2Api<OpaqueNested> for JsValue {
    fn wire2api(self) -> OpaqueNested {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        OpaqueNested {
            first: self_.get(0).wire2api(),
            second: self_.get(1).wire2api(),
        }
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

impl Wire2Api<Option<Vec<u8>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Option<Vec<u8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<OptVecs> for JsValue {
    fn wire2api(self) -> OptVecs {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        OptVecs {
            i32: self_.get(0).wire2api(),
            enums: self_.get(1).wire2api(),
            strings: self_.get(2).wire2api(),
            buffers: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<Sequences> for JsValue {
    fn wire2api(self) -> Sequences {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        Sequences(self_.get(0).wire2api())
    }
}
impl Wire2Api<SomeStruct> for JsValue {
    fn wire2api(self) -> SomeStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        SomeStruct {
            value: self_.get(0).wire2api(),
        }
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
impl Wire2Api<StructWithEnum> for JsValue {
    fn wire2api(self) -> StructWithEnum {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        StructWithEnum {
            abc1: self_.get(0).wire2api(),
            abc2: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<SumWith> for JsValue {
    fn wire2api(self) -> SumWith {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
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
impl Wire2Api<TestId> for JsValue {
    fn wire2api(self) -> TestId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        TestId(self_.get(0).wire2api())
    }
}

impl Wire2Api<[u8; 1600]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
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
        assert_eq!(
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

impl<T> Wire2Api<Option<T>> for JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<RustOpaque<Box<dyn DartDebug>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Box<dyn DartDebug>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<chrono::Duration> for JsValue {
    fn wire2api(self) -> chrono::Duration {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<Vec<chrono::Duration>> for JsValue {
    fn wire2api(self) -> Vec<chrono::Duration> {
        self.unchecked_into::<js_sys::BigInt64Array>()
            .to_vec()
            .into_iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Local>> for JsValue {
    fn wire2api(self) -> chrono::DateTime<chrono::Local> {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<chrono::NaiveDateTime> for JsValue {
    fn wire2api(self) -> chrono::NaiveDateTime {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<Vec<chrono::NaiveDateTime>> for JsValue {
    fn wire2api(self) -> Vec<chrono::NaiveDateTime> {
        self.unchecked_into::<js_sys::BigInt64Array>()
            .to_vec()
            .into_iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Utc>> for JsValue {
    fn wire2api(self) -> chrono::DateTime<chrono::Utc> {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<RustOpaque<HideData>> for JsValue {
    fn wire2api(self) -> RustOpaque<HideData> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<[RustOpaque<HideData>; 2]> for JsValue {
    fn wire2api(self) -> [RustOpaque<HideData>; 2] {
        let vec: Vec<RustOpaque<HideData>> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<RustOpaque<i32>> for JsValue {
    fn wire2api(self) -> RustOpaque<i32> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<HideData>>> for JsValue {
    fn wire2api(self) -> RustOpaque<Mutex<HideData>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<NonCloneData>> for JsValue {
    fn wire2api(self) -> RustOpaque<NonCloneData> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<NonSendHideData>> for JsValue {
    fn wire2api(self) -> RustOpaque<NonSendHideData> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<[DartOpaque; 1]> for JsValue {
    fn wire2api(self) -> [DartOpaque; 1] {
        let vec: Vec<DartOpaque> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<RustOpaque<RwLock<HideData>>> for JsValue {
    fn wire2api(self) -> RustOpaque<RwLock<HideData>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<[TestId; 4]> for JsValue {
    fn wire2api(self) -> [TestId; 4] {
        let vec: Vec<TestId> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<uuid::Uuid> for JsValue {
    fn wire2api(self) -> uuid::Uuid {
        self.unchecked_into::<js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for JsValue {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        self.unchecked_into::<js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
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
impl Wire2Api<Box<Blob>> for JsValue {
    fn wire2api(self) -> Box<Blob> {
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
impl Wire2Api<Box<MySizeFreezed>> for JsValue {
    fn wire2api(self) -> Box<MySizeFreezed> {
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
impl Wire2Api<Box<[u8; 1600]>> for JsValue {
    fn wire2api(self) -> Box<[u8; 1600]> {
        let vec: Vec<u8> = self.wire2api();
        Box::new(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<Box<Weekdays>> for JsValue {
    fn wire2api(self) -> Box<Weekdays> {
        let ptr: Box<i32> = self.wire2api();
        Box::new(ptr.wire2api())
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
impl Wire2Api<[f64; 16]> for JsValue {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        support::from_vec_to_array(vec)
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
impl Wire2Api<[i32; 2]> for JsValue {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        support::from_vec_to_array(vec)
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
impl Wire2Api<Vec<i32>> for JsValue {
    fn wire2api(self) -> Vec<i32> {
        self.unchecked_into::<js_sys::Int32Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<i64>> for JsValue {
    fn wire2api(self) -> Vec<i64> {
        let buf = self.dyn_into::<js_sys::BigInt64Array>().unwrap();
        let buf = js_sys::Uint8Array::new(&buf.buffer());
        support::slice_from_byte_buffer(buf.to_vec()).into()
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
impl Wire2Api<[u8; 1600]> for JsValue {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for JsValue {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for JsValue {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
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
