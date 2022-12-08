use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_handle_uuids(port_: i64, ids: *mut wire_uint_8_list) {
    wire_handle_uuids_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_handle_uuids_convert_to_strings(port_: i64, ids: *mut wire_uint_8_list) {
    wire_handle_uuids_convert_to_strings_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_handle_strings(port_: i64, strings: *mut wire_StringList) {
    wire_handle_strings_impl(port_, strings)
}

#[no_mangle]
pub extern "C" fn wire_handle_bool(port_: i64, input: bool) {
    wire_handle_bool_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_u32(port_: i64, input: u32) {
    wire_handle_u32_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_u64(port_: i64, input: u64) {
    wire_handle_u64_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_i8(port_: i64, input: i8) {
    wire_handle_i8_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_i16(port_: i64, input: i16) {
    wire_handle_i16_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_i32(port_: i64, input: i32) {
    wire_handle_i32_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_i64(port_: i64, input: i64) {
    wire_handle_i64_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_f32(port_: i64, input: f32) {
    wire_handle_f32_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_f64(port_: i64, input: f64) {
    wire_handle_f64_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_string(port_: i64, input: *mut wire_uint_8_list) {
    wire_handle_string_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_sync_bool(input: bool) -> support::WireSyncReturnStruct {
    wire_handle_sync_bool_impl(input)
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
pub extern "C" fn wire_dummy(port_: i64, unit: i32) {
    wire_dummy_impl(port_, unit)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
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

// Section: related functions

// Section: impl Wire2Api

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
impl Wire2Api<Vec<uuid::Uuid>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        wire2api_uuids(multiple)
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

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
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

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
