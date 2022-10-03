use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_rust_metrics(port_: i64) {
    wire_rust_metrics_impl(port_)
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
pub extern "C" fn wire_handle_uuids_convert_to_strings(port_: i64, ids: *mut wire_uint_8_list) {
    wire_handle_uuids_convert_to_strings_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_handle_nested_uuids(port_: i64, ids: *mut wire_FeatureUuid) {
    wire_handle_nested_uuids_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_handle_strings(port_: i64, strings: *mut wire_StringList) {
    wire_handle_strings_impl(port_, strings)
}

#[no_mangle]
pub extern "C" fn wire_send_i64(port_: i64, value: i64) {
    wire_send_i64_impl(port_, value)
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
pub extern "C" fn new_box_autoadd_feature_uuid_0() -> *mut wire_FeatureUuid {
    support::new_leak_box_ptr(wire_FeatureUuid::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

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
impl Wire2Api<FeatureUuid> for *mut wire_FeatureUuid {
    fn wire2api(self) -> FeatureUuid {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<FeatureUuid>::wire2api(*wrap).into()
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
pub struct wire_FeatureUuid {
    one: *mut wire_uint_8_list,
    many: *mut wire_uint_8_list,
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

impl NewWithNullPtr for wire_FeatureUuid {
    fn new_with_null_ptr() -> Self {
        Self {
            one: core::ptr::null_mut(),
            many: core::ptr::null_mut(),
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
