use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_simple_adder(port_: i64, a: i32, b: i32) {
    wire_simple_adder_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_primitive_types(
    port_: i64,
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) {
    wire_primitive_types_impl(port_, my_i32, my_i64, my_f64, my_bool)
}

#[no_mangle]
pub extern "C" fn wire_primitive_u32(port_: i64, my_u32: u32) {
    wire_primitive_u32_impl(port_, my_u32)
}

#[no_mangle]
pub extern "C" fn wire_handle_string(port_: i64, s: *mut wire_uint_8_list) {
    wire_handle_string_impl(port_, s)
}

#[no_mangle]
pub extern "C" fn wire_handle_return_unit(port_: i64) {
    wire_handle_return_unit_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_u8(port_: i64, v: *mut wire_uint_8_list) {
    wire_handle_vec_u8_impl(port_, v)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_of_primitive(port_: i64, n: i32) {
    wire_handle_vec_of_primitive_impl(port_, n)
}

#[no_mangle]
pub extern "C" fn wire_handle_zero_copy_vec_of_primitive(port_: i64, n: i32) {
    wire_handle_zero_copy_vec_of_primitive_impl(port_, n)
}

#[no_mangle]
pub extern "C" fn wire_handle_struct(port_: i64, arg: *mut wire_MySize, boxed: *mut wire_MySize) {
    wire_handle_struct_impl(port_, arg, boxed)
}

#[no_mangle]
pub extern "C" fn wire_handle_newtype(port_: i64, arg: *mut wire_NewTypeInt) {
    wire_handle_newtype_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_list_of_struct(port_: i64, l: *mut wire_list_my_size) {
    wire_handle_list_of_struct_impl(port_, l)
}

#[no_mangle]
pub extern "C" fn wire_handle_string_list(port_: i64, names: *mut wire_StringList) {
    wire_handle_string_list_impl(port_, names)
}

#[no_mangle]
pub extern "C" fn wire_handle_complex_struct(port_: i64, s: *mut wire_MyTreeNode) {
    wire_handle_complex_struct_impl(port_, s)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_return(
    mode: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_handle_sync_return_impl(mode)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_bool(input: bool) -> support::WireSyncReturnStruct {
    wire_handle_sync_bool_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_u8(input: u8) -> support::WireSyncReturnStruct {
    wire_handle_sync_u8_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_u16(input: u16) -> support::WireSyncReturnStruct {
    wire_handle_sync_u16_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_u32(input: u32) -> support::WireSyncReturnStruct {
    wire_handle_sync_u32_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_u64(input: u64) -> support::WireSyncReturnStruct {
    wire_handle_sync_u64_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_i8(input: i8) -> support::WireSyncReturnStruct {
    wire_handle_sync_i8_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_i16(input: i16) -> support::WireSyncReturnStruct {
    wire_handle_sync_i16_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_i32(input: i32) -> support::WireSyncReturnStruct {
    wire_handle_sync_i32_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_i64(input: i64) -> support::WireSyncReturnStruct {
    wire_handle_sync_i64_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_f32(input: f32) -> support::WireSyncReturnStruct {
    wire_handle_sync_f32_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_f64(input: f64) -> support::WireSyncReturnStruct {
    wire_handle_sync_f64_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_string(
    input: *mut wire_uint_8_list,
) -> support::WireSyncReturnStruct {
    wire_handle_sync_string_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream(port_: i64, arg: *mut wire_uint_8_list) {
    wire_handle_stream_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_of_struct(port_: i64) {
    wire_handle_stream_of_struct_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_err(port_: i64) {
    wire_return_err_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_panic(port_: i64) {
    wire_return_panic_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_return(port_: i64, left: f64, right: f64) {
    wire_handle_optional_return_impl(port_, left, right)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_struct(port_: i64, document: *mut wire_uint_8_list) {
    wire_handle_optional_struct_impl(port_, document)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_increment(port_: i64, opt: *mut wire_ExoticOptionals) {
    wire_handle_optional_increment_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_increment_boxed_optional(port_: i64, opt: *mut f64) {
    wire_handle_increment_boxed_optional_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_option_box_arguments(
    port_: i64,
    i8box: *mut i8,
    u8box: *mut u8,
    i32box: *mut i32,
    i64box: *mut i64,
    f64box: *mut f64,
    boolbox: *mut bool,
    structbox: *mut wire_ExoticOptionals,
) {
    wire_handle_option_box_arguments_impl(
        port_, i8box, u8box, i32box, i64box, f64box, boolbox, structbox,
    )
}

#[no_mangle]
pub extern "C" fn wire_print_note(port_: i64, note: *mut wire_Note) {
    wire_print_note_impl(port_, note)
}

#[no_mangle]
pub extern "C" fn wire_handle_return_enum(port_: i64, input: *mut wire_uint_8_list) {
    wire_handle_return_enum_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_enum_parameter(port_: i64, weekday: i32) {
    wire_handle_enum_parameter_impl(port_, weekday)
}

#[no_mangle]
pub extern "C" fn wire_handle_customized_struct(port_: i64, val: *mut wire_Customized) {
    wire_handle_customized_struct_impl(port_, val)
}

#[no_mangle]
pub extern "C" fn wire_handle_enum_struct(port_: i64, val: *mut wire_KitchenSink) {
    wire_handle_enum_struct_impl(port_, val)
}

#[no_mangle]
pub extern "C" fn wire_use_imported_struct(port_: i64, my_struct: *mut wire_MyStruct) {
    wire_use_imported_struct_impl(port_, my_struct)
}

#[no_mangle]
pub extern "C" fn wire_use_imported_enum(port_: i64, my_enum: i32) {
    wire_use_imported_enum_impl(port_, my_enum)
}

#[no_mangle]
pub extern "C" fn wire_get_app_settings(port_: i64) {
    wire_get_app_settings_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_app_embedded(port_: i64, app_settings: *mut wire_ApplicationSettings) {
    wire_is_app_embedded_impl(port_, app_settings)
}

#[no_mangle]
pub extern "C" fn wire_get_message(port_: i64) {
    wire_get_message_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_repeat_number(port_: i64, num: i32, times: usize) {
    wire_repeat_number_impl(port_, num, times)
}

#[no_mangle]
pub extern "C" fn wire_repeat_sequence(port_: i64, seq: i32, times: usize) {
    wire_repeat_sequence_impl(port_, seq, times)
}

#[no_mangle]
pub extern "C" fn wire_first_number(port_: i64, nums: *mut wire_Numbers) {
    wire_first_number_impl(port_, nums)
}

#[no_mangle]
pub extern "C" fn wire_first_sequence(port_: i64, seqs: *mut wire_Sequences) {
    wire_first_sequence_impl(port_, seqs)
}

#[no_mangle]
pub extern "C" fn wire_get_array(port_: i64) {
    wire_get_array_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_complex_array(port_: i64) {
    wire_get_complex_array_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_usize(port_: i64, u: usize) {
    wire_get_usize_impl(port_, u)
}

#[no_mangle]
pub extern "C" fn wire_next_user_id(port_: i64, user_id: *mut wire_UserId) {
    wire_next_user_id_impl(port_, user_id)
}

#[no_mangle]
pub extern "C" fn wire_register_event_listener(port_: i64) {
    wire_register_event_listener_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_close_event_listener(port_: i64) {
    wire_close_event_listener_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_event(
    port_: i64,
    address: *mut wire_uint_8_list,
    payload: *mut wire_uint_8_list,
) {
    wire_create_event_impl(port_, address, payload)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_sink_at_1(port_: i64, key: u32, max: u32) {
    wire_handle_stream_sink_at_1_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_sink_at_2(port_: i64, key: u32, max: u32) {
    wire_handle_stream_sink_at_2_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_sink_at_3(port_: i64, key: u32, max: u32) {
    wire_handle_stream_sink_at_3_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_get_sum_struct(port_: i64) {
    wire_get_sum_struct_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_sum_array(port_: i64, a: u32, b: u32, c: u32) {
    wire_get_sum_array_impl(port_, a, b, c)
}

#[no_mangle]
pub extern "C" fn wire_multiply_by_ten(port_: i64, measure: *mut wire_Measure) {
    wire_multiply_by_ten_impl(port_, measure)
}

#[no_mangle]
pub extern "C" fn wire_call_old_module_system(port_: i64) {
    wire_call_old_module_system_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_call_new_module_system(port_: i64) {
    wire_call_new_module_system_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_big_buffers(port_: i64) {
    wire_handle_big_buffers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_datetime_utc(port_: i64, d: i64) {
    wire_datetime_utc_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_datetime_local(port_: i64, d: i64) {
    wire_datetime_local_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_naivedatetime(port_: i64, d: i64) {
    wire_naivedatetime_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_duration(port_: i64, d: i64) {
    wire_duration_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_how_long_does_it_take(port_: i64, mine: *mut wire_FeatureChrono) {
    wire_how_long_does_it_take_impl(port_, mine)
}

#[no_mangle]
pub extern "C" fn wire_handle_uuid(port_: i64, id: *mut wire_uint_8_list) {
    wire_handle_uuid_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_handle_uuids(port_: i64, ids: *mut wire_uint_8_list) {
    wire_handle_uuids_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_handle_nested_uuids(port_: i64, ids: *mut wire_FeatureUuid) {
    wire_handle_nested_uuids_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_new_msgid(port_: i64, id: *mut wire_uint_8_list) {
    wire_new_msgid_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_use_msgid(port_: i64, id: *mut wire_MessageId) {
    wire_use_msgid_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_boxed_blob(port_: i64, blob: *mut wire_uint_8_list) {
    wire_boxed_blob_impl(port_, blob)
}

#[no_mangle]
pub extern "C" fn wire_use_boxed_blob(port_: i64, blob: *mut wire_Blob) {
    wire_use_boxed_blob_impl(port_, blob)
}

#[no_mangle]
pub extern "C" fn wire_return_boxed_feed_id(port_: i64, id: *mut wire_uint_8_list) {
    wire_return_boxed_feed_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_return_boxed_raw_feed_id(port_: i64, id: *mut wire_FeedId) {
    wire_return_boxed_raw_feed_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_test_id(port_: i64, id: *mut wire_TestId) {
    wire_test_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_last_number(port_: i64, array: *mut wire_float_64_list) {
    wire_last_number_impl(port_, array)
}

#[no_mangle]
pub extern "C" fn wire_nested_id(port_: i64, id: *mut wire_list_test_id) {
    wire_nested_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_create_opaque(port_: i64) {
    wire_create_opaque_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_array_opaque_enum(port_: i64) {
    wire_create_array_opaque_enum_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_run_enum_opaque(port_: i64, opaque: *mut wire_EnumOpaque) {
    wire_run_enum_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_opaque(port_: i64, opaque: *mut wire_OpaqueStruct) {
    wire_run_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_opaque_with_delay(port_: i64, opaque: *mut wire_OpaqueStruct) {
    wire_run_opaque_with_delay_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_opaque_array(port_: i64) {
    wire_opaque_array_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_nested_opaque(port_: i64) {
    wire_create_nested_opaque_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_run_nested_opaque(port_: i64, opaque: *mut wire_OpaqueNested) {
    wire_run_nested_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_sync_option() -> support::WireSyncReturnStruct {
    wire_sync_option_impl()
}

#[no_mangle]
pub extern "C" fn wire_sync_option_null() -> support::WireSyncReturnStruct {
    wire_sync_option_null_impl()
}

#[no_mangle]
pub extern "C" fn wire_sum__method__SumWith(port_: i64, that: *mut wire_SumWith, y: u32, z: u32) {
    wire_sum__method__SumWith_impl(port_, that, y, z)
}

#[no_mangle]
pub extern "C" fn wire_new__static_method__ConcatenateWith(port_: i64, a: *mut wire_uint_8_list) {
    wire_new__static_method__ConcatenateWith_impl(port_, a)
}

#[no_mangle]
pub extern "C" fn wire_concatenate__method__ConcatenateWith(
    port_: i64,
    that: *mut wire_ConcatenateWith,
    b: *mut wire_uint_8_list,
) {
    wire_concatenate__method__ConcatenateWith_impl(port_, that, b)
}

#[no_mangle]
pub extern "C" fn wire_concatenate_static__static_method__ConcatenateWith(
    port_: i64,
    a: *mut wire_uint_8_list,
    b: *mut wire_uint_8_list,
) {
    wire_concatenate_static__static_method__ConcatenateWith_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_handle_some_stream_sink__method__ConcatenateWith(
    port_: i64,
    that: *mut wire_ConcatenateWith,
    key: u32,
    max: u32,
) {
    wire_handle_some_stream_sink__method__ConcatenateWith_impl(port_, that, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_some_stream_sink_at_1__method__ConcatenateWith(
    port_: i64,
    that: *mut wire_ConcatenateWith,
) {
    wire_handle_some_stream_sink_at_1__method__ConcatenateWith_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_handle_some_static_stream_sink__static_method__ConcatenateWith(
    port_: i64,
    key: u32,
    max: u32,
) {
    wire_handle_some_static_stream_sink__static_method__ConcatenateWith_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_some_static_stream_sink_single_arg__static_method__ConcatenateWith(
    port_: i64,
) {
    wire_handle_some_static_stream_sink_single_arg__static_method__ConcatenateWith_impl(port_)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_BoxDartDebug() -> *mut wire_BoxDartDebug {
    support::new_leak_box_ptr(wire_BoxDartDebug::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_I32() -> *mut wire_I32 {
    support::new_leak_box_ptr(wire_I32::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_MutexOpaqueStruct() -> *mut wire_MutexOpaqueStruct {
    support::new_leak_box_ptr(wire_MutexOpaqueStruct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_OpaqueStruct() -> *mut wire_OpaqueStruct {
    support::new_leak_box_ptr(wire_OpaqueStruct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RwLockOpaqueStruct() -> *mut wire_RwLockOpaqueStruct {
    support::new_leak_box_ptr(wire_RwLockOpaqueStruct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_application_env_0() -> *mut wire_ApplicationEnv {
    support::new_leak_box_ptr(wire_ApplicationEnv::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_settings_0() -> *mut wire_ApplicationSettings {
    support::new_leak_box_ptr(wire_ApplicationSettings::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_attribute_0() -> *mut wire_Attribute {
    support::new_leak_box_ptr(wire_Attribute::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_concatenate_with_0() -> *mut wire_ConcatenateWith {
    support::new_leak_box_ptr(wire_ConcatenateWith::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_customized_0() -> *mut wire_Customized {
    support::new_leak_box_ptr(wire_Customized::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_opaque_0() -> *mut wire_EnumOpaque {
    support::new_leak_box_ptr(wire_EnumOpaque::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_exotic_optionals_0() -> *mut wire_ExoticOptionals {
    support::new_leak_box_ptr(wire_ExoticOptionals::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f64_0(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feature_chrono_0() -> *mut wire_FeatureChrono {
    support::new_leak_box_ptr(wire_FeatureChrono::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feature_uuid_0() -> *mut wire_FeatureUuid {
    support::new_leak_box_ptr(wire_FeatureUuid::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feed_id_0() -> *mut wire_FeedId {
    support::new_leak_box_ptr(wire_FeedId::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i64_0(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_kitchen_sink_0() -> *mut wire_KitchenSink {
    support::new_leak_box_ptr(wire_KitchenSink::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_measure_0() -> *mut wire_Measure {
    support::new_leak_box_ptr(wire_Measure::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_message_id_0() -> *mut wire_MessageId {
    support::new_leak_box_ptr(wire_MessageId::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_size_0() -> *mut wire_MySize {
    support::new_leak_box_ptr(wire_MySize::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_struct_0() -> *mut wire_MyStruct {
    support::new_leak_box_ptr(wire_MyStruct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_tree_node_0() -> *mut wire_MyTreeNode {
    support::new_leak_box_ptr(wire_MyTreeNode::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_new_type_int_0() -> *mut wire_NewTypeInt {
    support::new_leak_box_ptr(wire_NewTypeInt::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_note_0() -> *mut wire_Note {
    support::new_leak_box_ptr(wire_Note::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_numbers_0() -> *mut wire_Numbers {
    support::new_leak_box_ptr(wire_Numbers::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_opaque_nested_0() -> *mut wire_OpaqueNested {
    support::new_leak_box_ptr(wire_OpaqueNested::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sequences_0() -> *mut wire_Sequences {
    support::new_leak_box_ptr(wire_Sequences::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sum_with_0() -> *mut wire_SumWith {
    support::new_leak_box_ptr(wire_SumWith::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_test_id_0() -> *mut wire_TestId {
    support::new_leak_box_ptr(wire_TestId::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_user_id_0() -> *mut wire_UserId {
    support::new_leak_box_ptr(wire_UserId::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_blob_0() -> *mut wire_Blob {
    support::new_leak_box_ptr(wire_Blob::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_distance_0() -> *mut wire_Distance {
    support::new_leak_box_ptr(wire_Distance::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_exotic_optionals_0() -> *mut wire_ExoticOptionals {
    support::new_leak_box_ptr(wire_ExoticOptionals::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_f64_0(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i64_0(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i8_0(value: i8) -> *mut i8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_kitchen_sink_0() -> *mut wire_KitchenSink {
    support::new_leak_box_ptr(wire_KitchenSink::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_my_size_0() -> *mut wire_MySize {
    support::new_leak_box_ptr(wire_MySize::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_speed_0() -> *mut wire_Speed {
    support::new_leak_box_ptr(wire_Speed::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_u8_0(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_weekdays_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_float_32_list_0(len: i32) -> *mut wire_float_32_list {
    let ans = wire_float_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_float_64_list_0(len: i32) -> *mut wire_float_64_list {
    let ans = wire_float_64_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_32_list_0(len: i32) -> *mut wire_int_32_list {
    let ans = wire_int_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_8_list_0(len: i32) -> *mut wire_int_8_list {
    let ans = wire_int_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_application_env_var_0(len: i32) -> *mut wire_list_application_env_var {
    let wrap = wire_list_application_env_var {
        ptr: support::new_leak_vec_ptr(<wire_ApplicationEnvVar>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_attribute_0(len: i32) -> *mut wire_list_attribute {
    let wrap = wire_list_attribute {
        ptr: support::new_leak_vec_ptr(<wire_Attribute>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_size_0(len: i32) -> *mut wire_list_my_size {
    let wrap = wire_list_my_size {
        ptr: support::new_leak_vec_ptr(<wire_MySize>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_tree_node_0(len: i32) -> *mut wire_list_my_tree_node {
    let wrap = wire_list_my_tree_node {
        ptr: support::new_leak_vec_ptr(<wire_MyTreeNode>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_attribute_0(
    len: i32,
) -> *mut wire_list_opt_box_autoadd_attribute {
    let wrap = wire_list_opt_box_autoadd_attribute {
        ptr: support::new_leak_vec_ptr(<*mut wire_Attribute>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_test_id_0(len: i32) -> *mut wire_list_test_id {
    let wrap = wire_list_test_id {
        ptr: support::new_leak_vec_ptr(<wire_TestId>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: deallocate functions

#[no_mangle]
pub extern "C" fn drop_box_application_env_0(raw: *mut wire_ApplicationEnv) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_application_settings_0(raw: *mut wire_ApplicationSettings) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_attribute_0(raw: *mut wire_Attribute) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_bool_0(raw: *mut bool) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_concatenate_with_0(raw: *mut wire_ConcatenateWith) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_customized_0(raw: *mut wire_Customized) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_enum_opaque_0(raw: *mut wire_EnumOpaque) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_exotic_optionals_0(raw: *mut wire_ExoticOptionals) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_f64_0(raw: *mut f64) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_feature_chrono_0(raw: *mut wire_FeatureChrono) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_feature_uuid_0(raw: *mut wire_FeatureUuid) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_feed_id_0(raw: *mut wire_FeedId) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_i32_0(raw: *mut i32) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_i64_0(raw: *mut i64) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_kitchen_sink_0(raw: *mut wire_KitchenSink) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_measure_0(raw: *mut wire_Measure) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_message_id_0(raw: *mut wire_MessageId) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_my_size_0(raw: *mut wire_MySize) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_my_struct_0(raw: *mut wire_MyStruct) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_my_tree_node_0(raw: *mut wire_MyTreeNode) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_new_type_int_0(raw: *mut wire_NewTypeInt) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_note_0(raw: *mut wire_Note) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_numbers_0(raw: *mut wire_Numbers) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_opaque_nested_0(raw: *mut wire_OpaqueNested) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_sequences_0(raw: *mut wire_Sequences) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_sum_with_0(raw: *mut wire_SumWith) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_test_id_0(raw: *mut wire_TestId) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_autoadd_user_id_0(raw: *mut wire_UserId) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_blob_0(raw: *mut wire_Blob) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_bool_0(raw: *mut bool) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_distance_0(raw: *mut wire_Distance) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_exotic_optionals_0(raw: *mut wire_ExoticOptionals) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_f64_0(raw: *mut f64) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_i32_0(raw: *mut i32) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_i64_0(raw: *mut i64) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_i8_0(raw: *mut i8) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_kitchen_sink_0(raw: *mut wire_KitchenSink) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_my_size_0(raw: *mut wire_MySize) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_speed_0(raw: *mut wire_Speed) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_u8_0(raw: *mut u8) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

#[no_mangle]
pub extern "C" fn drop_box_weekdays_0(raw: *mut i32) {
    unsafe {
        {
            support::box_from_leak_ptr(raw);
        }
    }
}

// Section: impl Wire2Api

impl Wire2Api<Opaque<Box<dyn DartDebug>>> for *mut wire_BoxDartDebug {
    fn wire2api(self) -> Opaque<Box<dyn DartDebug>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::microseconds(self)
    }
}
impl Wire2Api<chrono::DateTime<chrono::Local>> for i64 {
    fn wire2api(self) -> chrono::DateTime<chrono::Local> {
        let Timestamp { s, ns } = wire2api_timestamp(self);
        chrono::DateTime::<chrono::Local>::from(chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(s, ns),
            chrono::Utc,
        ))
    }
}
impl Wire2Api<chrono::NaiveDateTime> for i64 {
    fn wire2api(self) -> chrono::NaiveDateTime {
        let Timestamp { s, ns } = wire2api_timestamp(self);
        chrono::NaiveDateTime::from_timestamp(s, ns)
    }
}
impl Wire2Api<chrono::DateTime<chrono::Utc>> for i64 {
    fn wire2api(self) -> chrono::DateTime<chrono::Utc> {
        let Timestamp { s, ns } = wire2api_timestamp(self);
        chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(s, ns),
            chrono::Utc,
        )
    }
}
impl Wire2Api<Opaque<i32>> for *mut wire_I32 {
    fn wire2api(self) -> Opaque<i32> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<Mutex<OpaqueStruct>>> for *mut wire_MutexOpaqueStruct {
    fn wire2api(self) -> Opaque<Mutex<OpaqueStruct>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<OpaqueStruct>> for *mut wire_OpaqueStruct {
    fn wire2api(self) -> Opaque<OpaqueStruct> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RwLock<OpaqueStruct>>> for *mut wire_RwLockOpaqueStruct {
    fn wire2api(self) -> Opaque<RwLock<OpaqueStruct>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<[TestId; 4]> for *mut wire_list_test_id {
    fn wire2api(self) -> [TestId; 4] {
        let vec: Vec<TestId> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<uuid::Uuid> for *mut wire_uint_8_list {
    fn wire2api(self) -> uuid::Uuid {
        let single: Vec<u8> = self.wire2api();
        wire2api_uuid_ref(single.as_slice())
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        wire2api_uuids(multiple)
    }
}
impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for *mut wire_uint_8_list {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
        ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<ApplicationEnv> for wire_ApplicationEnv {
    fn wire2api(self) -> ApplicationEnv {
        ApplicationEnv {
            vars: self.vars.wire2api(),
        }
    }
}
impl Wire2Api<ApplicationEnvVar> for wire_ApplicationEnvVar {
    fn wire2api(self) -> ApplicationEnvVar {
        ApplicationEnvVar(self.field0.wire2api(), self.field1.wire2api())
    }
}

impl Wire2Api<ApplicationSettings> for wire_ApplicationSettings {
    fn wire2api(self) -> ApplicationSettings {
        ApplicationSettings {
            name: self.name.wire2api(),
            version: self.version.wire2api(),
            mode: self.mode.wire2api(),
            env: self.env.wire2api(),
        }
    }
}
impl Wire2Api<Attribute> for wire_Attribute {
    fn wire2api(self) -> Attribute {
        Attribute {
            key: self.key.wire2api(),
            value: self.value.wire2api(),
        }
    }
}
impl Wire2Api<Blob> for wire_Blob {
    fn wire2api(self) -> Blob {
        Blob(self.field0.wire2api())
    }
}

impl Wire2Api<Box<ApplicationEnv>> for *mut wire_ApplicationEnv {
    fn wire2api(self) -> Box<ApplicationEnv> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApplicationEnv>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ApplicationSettings> for *mut wire_ApplicationSettings {
    fn wire2api(self) -> ApplicationSettings {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApplicationSettings>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Attribute> for *mut wire_Attribute {
    fn wire2api(self) -> Attribute {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Attribute>::wire2api(*wrap).into()
    }
}

impl Wire2Api<ConcatenateWith> for *mut wire_ConcatenateWith {
    fn wire2api(self) -> ConcatenateWith {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ConcatenateWith>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Customized> for *mut wire_Customized {
    fn wire2api(self) -> Customized {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Customized>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EnumOpaque> for *mut wire_EnumOpaque {
    fn wire2api(self) -> EnumOpaque {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumOpaque>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ExoticOptionals> for *mut wire_ExoticOptionals {
    fn wire2api(self) -> ExoticOptionals {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ExoticOptionals>::wire2api(*wrap).into()
    }
}

impl Wire2Api<FeatureChrono> for *mut wire_FeatureChrono {
    fn wire2api(self) -> FeatureChrono {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<FeatureChrono>::wire2api(*wrap).into()
    }
}
impl Wire2Api<FeatureUuid> for *mut wire_FeatureUuid {
    fn wire2api(self) -> FeatureUuid {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<FeatureUuid>::wire2api(*wrap).into()
    }
}
impl Wire2Api<FeedId> for *mut wire_FeedId {
    fn wire2api(self) -> FeedId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<FeedId>::wire2api(*wrap).into()
    }
}

impl Wire2Api<KitchenSink> for *mut wire_KitchenSink {
    fn wire2api(self) -> KitchenSink {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KitchenSink>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Measure> for *mut wire_Measure {
    fn wire2api(self) -> Measure {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Measure>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MessageId> for *mut wire_MessageId {
    fn wire2api(self) -> MessageId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MessageId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MySize> for *mut wire_MySize {
    fn wire2api(self) -> MySize {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MySize>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MyStruct> for *mut wire_MyStruct {
    fn wire2api(self) -> MyStruct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MyStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MyTreeNode> for *mut wire_MyTreeNode {
    fn wire2api(self) -> MyTreeNode {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MyTreeNode>::wire2api(*wrap).into()
    }
}
impl Wire2Api<NewTypeInt> for *mut wire_NewTypeInt {
    fn wire2api(self) -> NewTypeInt {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<NewTypeInt>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Note> for *mut wire_Note {
    fn wire2api(self) -> Note {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Note>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Numbers> for *mut wire_Numbers {
    fn wire2api(self) -> Numbers {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Numbers>::wire2api(*wrap).into()
    }
}
impl Wire2Api<OpaqueNested> for *mut wire_OpaqueNested {
    fn wire2api(self) -> OpaqueNested {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OpaqueNested>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Sequences> for *mut wire_Sequences {
    fn wire2api(self) -> Sequences {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Sequences>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SumWith> for *mut wire_SumWith {
    fn wire2api(self) -> SumWith {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SumWith>::wire2api(*wrap).into()
    }
}
impl Wire2Api<TestId> for *mut wire_TestId {
    fn wire2api(self) -> TestId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TestId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<UserId> for *mut wire_UserId {
    fn wire2api(self) -> UserId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<UserId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<Blob>> for *mut wire_Blob {
    fn wire2api(self) -> Box<Blob> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Blob>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Box<Distance>> for *mut wire_Distance {
    fn wire2api(self) -> Box<Distance> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Distance>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<ExoticOptionals>> for *mut wire_ExoticOptionals {
    fn wire2api(self) -> Box<ExoticOptionals> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ExoticOptionals>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Box<KitchenSink>> for *mut wire_KitchenSink {
    fn wire2api(self) -> Box<KitchenSink> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<KitchenSink>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<MySize>> for *mut wire_MySize {
    fn wire2api(self) -> Box<MySize> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MySize>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<Speed>> for *mut wire_Speed {
    fn wire2api(self) -> Box<Speed> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Speed>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Box<[u8; 1600]>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Box<[u8; 1600]> {
        Wire2Api::<[u8; 1600]>::wire2api(self).into()
    }
}
impl Wire2Api<Box<Weekdays>> for *mut i32 {
    fn wire2api(self) -> Box<Weekdays> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Weekdays>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ConcatenateWith> for wire_ConcatenateWith {
    fn wire2api(self) -> ConcatenateWith {
        ConcatenateWith {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<Customized> for wire_Customized {
    fn wire2api(self) -> Customized {
        Customized {
            final_field: self.final_field.wire2api(),
            non_final_field: self.non_final_field.wire2api(),
        }
    }
}
impl Wire2Api<Distance> for wire_Distance {
    fn wire2api(self) -> Distance {
        match self.tag {
            0 => Distance::Unknown,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Map);
                Distance::Map(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumOpaque> for wire_EnumOpaque {
    fn wire2api(self) -> EnumOpaque {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Struct);
                EnumOpaque::Struct(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Primitive);
                EnumOpaque::Primitive(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.TraitObj);
                EnumOpaque::TraitObj(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Mutex);
                EnumOpaque::Mutex(ans.field0.wire2api())
            },
            4 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.RwLock);
                EnumOpaque::RwLock(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<ExoticOptionals> for wire_ExoticOptionals {
    fn wire2api(self) -> ExoticOptionals {
        ExoticOptionals {
            int32: self.int32.wire2api(),
            int64: self.int64.wire2api(),
            float64: self.float64.wire2api(),
            boolean: self.boolean.wire2api(),
            zerocopy: self.zerocopy.wire2api(),
            int8list: self.int8list.wire2api(),
            uint8list: self.uint8list.wire2api(),
            int32list: self.int32list.wire2api(),
            float32list: self.float32list.wire2api(),
            float64list: self.float64list.wire2api(),
            attributes: self.attributes.wire2api(),
            attributes_nullable: self.attributes_nullable.wire2api(),
            nullable_attributes: self.nullable_attributes.wire2api(),
            newtypeint: self.newtypeint.wire2api(),
        }
    }
}

impl Wire2Api<[f64; 16]> for *mut wire_float_64_list {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<FeatureChrono> for wire_FeatureChrono {
    fn wire2api(self) -> FeatureChrono {
        FeatureChrono {
            utc: self.utc.wire2api(),
            local: self.local.wire2api(),
            duration: self.duration.wire2api(),
            naive: self.naive.wire2api(),
        }
    }
}
impl Wire2Api<FeatureUuid> for wire_FeatureUuid {
    fn wire2api(self) -> FeatureUuid {
        FeatureUuid {
            one: self.one.wire2api(),
            many: self.many.wire2api(),
        }
    }
}
impl Wire2Api<FeedId> for wire_FeedId {
    fn wire2api(self) -> FeedId {
        FeedId(self.field0.wire2api())
    }
}
impl Wire2Api<Vec<f32>> for *mut wire_float_32_list {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<f64>> for *mut wire_float_64_list {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<[i32; 2]> for *mut wire_int_32_list {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}

impl Wire2Api<Vec<i32>> for *mut wire_int_32_list {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i8>> for *mut wire_int_8_list {
    fn wire2api(self) -> Vec<i8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<KitchenSink> for wire_KitchenSink {
    fn wire2api(self) -> KitchenSink {
        match self.tag {
            0 => KitchenSink::Empty,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Primitives);
                KitchenSink::Primitives {
                    int32: ans.int32.wire2api(),
                    float64: ans.float64.wire2api(),
                    boolean: ans.boolean.wire2api(),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Nested);
                KitchenSink::Nested(ans.field0.wire2api(), ans.field1.wire2api())
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Optional);
                KitchenSink::Optional(ans.field0.wire2api(), ans.field1.wire2api())
            },
            4 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Buffer);
                KitchenSink::Buffer(ans.field0.wire2api())
            },
            5 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Enums);
                KitchenSink::Enums(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Vec<ApplicationEnvVar>> for *mut wire_list_application_env_var {
    fn wire2api(self) -> Vec<ApplicationEnvVar> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Attribute>> for *mut wire_list_attribute {
    fn wire2api(self) -> Vec<Attribute> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<MySize>> for *mut wire_list_my_size {
    fn wire2api(self) -> Vec<MySize> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<MyTreeNode>> for *mut wire_list_my_tree_node {
    fn wire2api(self) -> Vec<MyTreeNode> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<Attribute>>> for *mut wire_list_opt_box_autoadd_attribute {
    fn wire2api(self) -> Vec<Option<Attribute>> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<TestId>> for *mut wire_list_test_id {
    fn wire2api(self) -> Vec<TestId> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Measure> for wire_Measure {
    fn wire2api(self) -> Measure {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Speed);
                Measure::Speed(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Distance);
                Measure::Distance(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<MessageId> for wire_MessageId {
    fn wire2api(self) -> MessageId {
        MessageId(self.field0.wire2api())
    }
}

impl Wire2Api<MySize> for wire_MySize {
    fn wire2api(self) -> MySize {
        MySize {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}
impl Wire2Api<MyStruct> for wire_MyStruct {
    fn wire2api(self) -> MyStruct {
        MyStruct {
            content: self.content.wire2api(),
        }
    }
}
impl Wire2Api<MyTreeNode> for wire_MyTreeNode {
    fn wire2api(self) -> MyTreeNode {
        MyTreeNode {
            value_i32: self.value_i32.wire2api(),
            value_vec_u8: self.value_vec_u8.wire2api(),
            value_boolean: self.value_boolean.wire2api(),
            children: self.children.wire2api(),
        }
    }
}
impl Wire2Api<NewTypeInt> for wire_NewTypeInt {
    fn wire2api(self) -> NewTypeInt {
        NewTypeInt(self.field0.wire2api())
    }
}
impl Wire2Api<Note> for wire_Note {
    fn wire2api(self) -> Note {
        Note {
            day: self.day.wire2api(),
            body: self.body.wire2api(),
        }
    }
}
impl Wire2Api<Numbers> for wire_Numbers {
    fn wire2api(self) -> Numbers {
        Numbers(self.field0.wire2api())
    }
}
impl Wire2Api<OpaqueNested> for wire_OpaqueNested {
    fn wire2api(self) -> OpaqueNested {
        OpaqueNested {
            first: self.first.wire2api(),
            second: self.second.wire2api(),
        }
    }
}

impl Wire2Api<Sequences> for wire_Sequences {
    fn wire2api(self) -> Sequences {
        Sequences(self.field0.wire2api())
    }
}
impl Wire2Api<Speed> for wire_Speed {
    fn wire2api(self) -> Speed {
        match self.tag {
            0 => Speed::Unknown,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.GPS);
                Speed::GPS(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<SumWith> for wire_SumWith {
    fn wire2api(self) -> SumWith {
        SumWith {
            x: self.x.wire2api(),
        }
    }
}
impl Wire2Api<TestId> for wire_TestId {
    fn wire2api(self) -> TestId {
        TestId(self.field0.wire2api())
    }
}

impl Wire2Api<[u8; 1600]> for *mut wire_uint_8_list {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for *mut wire_uint_8_list {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for *mut wire_uint_8_list {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<UserId> for wire_UserId {
    fn wire2api(self) -> UserId {
        UserId {
            value: self.value.wire2api(),
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_BoxDartDebug {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_I32 {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexOpaqueStruct {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OpaqueStruct {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RwLockOpaqueStruct {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApplicationEnv {
    vars: *mut wire_list_application_env_var,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApplicationEnvVar {
    field0: *mut wire_uint_8_list,
    field1: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApplicationSettings {
    name: *mut wire_uint_8_list,
    version: *mut wire_uint_8_list,
    mode: i32,
    env: *mut wire_ApplicationEnv,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Attribute {
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Blob {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ConcatenateWith {
    a: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Customized {
    final_field: *mut wire_uint_8_list,
    non_final_field: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ExoticOptionals {
    int32: *mut i32,
    int64: *mut i64,
    float64: *mut f64,
    boolean: *mut bool,
    zerocopy: *mut wire_uint_8_list,
    int8list: *mut wire_int_8_list,
    uint8list: *mut wire_uint_8_list,
    int32list: *mut wire_int_32_list,
    float32list: *mut wire_float_32_list,
    float64list: *mut wire_float_64_list,
    attributes: *mut wire_list_attribute,
    attributes_nullable: *mut wire_list_opt_box_autoadd_attribute,
    nullable_attributes: *mut wire_list_opt_box_autoadd_attribute,
    newtypeint: *mut wire_NewTypeInt,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_FeatureChrono {
    utc: i64,
    local: i64,
    duration: i64,
    naive: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_FeatureUuid {
    one: *mut wire_uint_8_list,
    many: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_FeedId {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_32_list {
    ptr: *mut f32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_64_list {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_32_list {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_8_list {
    ptr: *mut i8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_application_env_var {
    ptr: *mut wire_ApplicationEnvVar,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_attribute {
    ptr: *mut wire_Attribute,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_size {
    ptr: *mut wire_MySize,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_tree_node {
    ptr: *mut wire_MyTreeNode,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_attribute {
    ptr: *mut *mut wire_Attribute,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_test_id {
    ptr: *mut wire_TestId,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MessageId {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MySize {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MyStruct {
    content: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MyTreeNode {
    value_i32: i32,
    value_vec_u8: *mut wire_uint_8_list,
    value_boolean: bool,
    children: *mut wire_list_my_tree_node,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NewTypeInt {
    field0: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Note {
    day: *mut i32,
    body: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Numbers {
    field0: *mut wire_int_32_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OpaqueNested {
    first: *mut wire_OpaqueStruct,
    second: *mut wire_OpaqueStruct,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Sequences {
    field0: *mut wire_int_32_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SumWith {
    x: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_TestId {
    field0: *mut wire_int_32_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_UserId {
    value: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Distance {
    tag: i32,
    kind: *mut DistanceKind,
}

#[repr(C)]
pub union DistanceKind {
    Unknown: *mut wire_Distance_Unknown,
    Map: *mut wire_Distance_Map,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Distance_Unknown {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Distance_Map {
    field0: f64,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque {
    tag: i32,
    kind: *mut EnumOpaqueKind,
}

#[repr(C)]
pub union EnumOpaqueKind {
    Struct: *mut wire_EnumOpaque_Struct,
    Primitive: *mut wire_EnumOpaque_Primitive,
    TraitObj: *mut wire_EnumOpaque_TraitObj,
    Mutex: *mut wire_EnumOpaque_Mutex,
    RwLock: *mut wire_EnumOpaque_RwLock,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_Struct {
    field0: *mut wire_OpaqueStruct,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_Primitive {
    field0: *mut wire_I32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_TraitObj {
    field0: *mut wire_BoxDartDebug,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_Mutex {
    field0: *mut wire_MutexOpaqueStruct,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_RwLock {
    field0: *mut wire_RwLockOpaqueStruct,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSink {
    tag: i32,
    kind: *mut KitchenSinkKind,
}

#[repr(C)]
pub union KitchenSinkKind {
    Empty: *mut wire_KitchenSink_Empty,
    Primitives: *mut wire_KitchenSink_Primitives,
    Nested: *mut wire_KitchenSink_Nested,
    Optional: *mut wire_KitchenSink_Optional,
    Buffer: *mut wire_KitchenSink_Buffer,
    Enums: *mut wire_KitchenSink_Enums,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSink_Empty {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSink_Primitives {
    int32: i32,
    float64: f64,
    boolean: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSink_Nested {
    field0: *mut wire_KitchenSink,
    field1: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSink_Optional {
    field0: *mut i32,
    field1: *mut i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSink_Buffer {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSink_Enums {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Measure {
    tag: i32,
    kind: *mut MeasureKind,
}

#[repr(C)]
pub union MeasureKind {
    Speed: *mut wire_Measure_Speed,
    Distance: *mut wire_Measure_Distance,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Measure_Speed {
    field0: *mut wire_Speed,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Measure_Distance {
    field0: *mut wire_Distance,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Speed {
    tag: i32,
    kind: *mut SpeedKind,
}

#[repr(C)]
pub union SpeedKind {
    Unknown: *mut wire_Speed_Unknown,
    GPS: *mut wire_Speed_GPS,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Speed_Unknown {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Speed_GPS {
    field0: f64,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_BoxDartDebug {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_I32 {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MutexOpaqueStruct {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_OpaqueStruct {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RwLockOpaqueStruct {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_ApplicationEnv {
    fn new_with_null_ptr() -> Self {
        Self {
            vars: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_ApplicationEnvVar {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_ApplicationSettings {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            version: core::ptr::null_mut(),
            mode: Default::default(),
            env: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Attribute {
    fn new_with_null_ptr() -> Self {
        Self {
            key: core::ptr::null_mut(),
            value: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Blob {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_ConcatenateWith {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Customized {
    fn new_with_null_ptr() -> Self {
        Self {
            final_field: core::ptr::null_mut(),
            non_final_field: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Distance {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_Distance_Map() -> *mut DistanceKind {
    support::new_leak_box_ptr(DistanceKind {
        Map: support::new_leak_box_ptr(wire_Distance_Map {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_EnumOpaque {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_Struct() -> *mut EnumOpaqueKind {
    support::new_leak_box_ptr(EnumOpaqueKind {
        Struct: support::new_leak_box_ptr(wire_EnumOpaque_Struct {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_Primitive() -> *mut EnumOpaqueKind {
    support::new_leak_box_ptr(EnumOpaqueKind {
        Primitive: support::new_leak_box_ptr(wire_EnumOpaque_Primitive {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_TraitObj() -> *mut EnumOpaqueKind {
    support::new_leak_box_ptr(EnumOpaqueKind {
        TraitObj: support::new_leak_box_ptr(wire_EnumOpaque_TraitObj {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_Mutex() -> *mut EnumOpaqueKind {
    support::new_leak_box_ptr(EnumOpaqueKind {
        Mutex: support::new_leak_box_ptr(wire_EnumOpaque_Mutex {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_RwLock() -> *mut EnumOpaqueKind {
    support::new_leak_box_ptr(EnumOpaqueKind {
        RwLock: support::new_leak_box_ptr(wire_EnumOpaque_RwLock {
            field0: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_ExoticOptionals {
    fn new_with_null_ptr() -> Self {
        Self {
            int32: core::ptr::null_mut(),
            int64: core::ptr::null_mut(),
            float64: core::ptr::null_mut(),
            boolean: core::ptr::null_mut(),
            zerocopy: core::ptr::null_mut(),
            int8list: core::ptr::null_mut(),
            uint8list: core::ptr::null_mut(),
            int32list: core::ptr::null_mut(),
            float32list: core::ptr::null_mut(),
            float64list: core::ptr::null_mut(),
            attributes: core::ptr::null_mut(),
            attributes_nullable: core::ptr::null_mut(),
            nullable_attributes: core::ptr::null_mut(),
            newtypeint: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_FeatureChrono {
    fn new_with_null_ptr() -> Self {
        Self {
            utc: Default::default(),
            local: Default::default(),
            duration: Default::default(),
            naive: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_FeatureUuid {
    fn new_with_null_ptr() -> Self {
        Self {
            one: core::ptr::null_mut(),
            many: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_FeedId {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_KitchenSink {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Primitives() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Primitives: support::new_leak_box_ptr(wire_KitchenSink_Primitives {
            int32: Default::default(),
            float64: Default::default(),
            boolean: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Nested() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Nested: support::new_leak_box_ptr(wire_KitchenSink_Nested {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Optional() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Optional: support::new_leak_box_ptr(wire_KitchenSink_Optional {
            field0: core::ptr::null_mut(),
            field1: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Buffer() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Buffer: support::new_leak_box_ptr(wire_KitchenSink_Buffer {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSink_Enums() -> *mut KitchenSinkKind {
    support::new_leak_box_ptr(KitchenSinkKind {
        Enums: support::new_leak_box_ptr(wire_KitchenSink_Enums {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_Measure {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_Measure_Speed() -> *mut MeasureKind {
    support::new_leak_box_ptr(MeasureKind {
        Speed: support::new_leak_box_ptr(wire_Measure_Speed {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Measure_Distance() -> *mut MeasureKind {
    support::new_leak_box_ptr(MeasureKind {
        Distance: support::new_leak_box_ptr(wire_Measure_Distance {
            field0: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_MessageId {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_MySize {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_MyStruct {
    fn new_with_null_ptr() -> Self {
        Self {
            content: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_MyTreeNode {
    fn new_with_null_ptr() -> Self {
        Self {
            value_i32: Default::default(),
            value_vec_u8: core::ptr::null_mut(),
            value_boolean: Default::default(),
            children: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_NewTypeInt {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_Note {
    fn new_with_null_ptr() -> Self {
        Self {
            day: core::ptr::null_mut(),
            body: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Numbers {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_OpaqueNested {
    fn new_with_null_ptr() -> Self {
        Self {
            first: core::ptr::null_mut(),
            second: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Sequences {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Speed {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_Speed_GPS() -> *mut SpeedKind {
    support::new_leak_box_ptr(SpeedKind {
        GPS: support::new_leak_box_ptr(wire_Speed_GPS {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_SumWith {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_TestId {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_UserId {
    fn new_with_null_ptr() -> Self {
        Self {
            value: Default::default(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
