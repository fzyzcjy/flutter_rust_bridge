use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_test_inbuilt_type_1(port_: i64, a: i32, b: f32) {
    wire_test_inbuilt_type_1_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_test_string_1(port_: i64, s: *mut wire_uint_8_list, i: u64) {
    wire_test_string_1_impl(port_, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_shared_struct_1(
    port_: i64,
    custom: *mut wire_SharedStruct,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_shared_struct_1_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_unique_struct_1(
    port_: i64,
    custom: *mut wire_OnlyForApi1Struct,
    s: *mut wire_uint_8_list,
    i: i16,
) {
    wire_test_unique_struct_1_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_cross_shared_struct_1(port_: i64, custom: *mut wire_CrossSharedStruct) {
    wire_test_cross_shared_struct_1_impl(port_, custom)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_only_for_api_1_struct() -> *mut wire_OnlyForApi1Struct {
    support::new_leak_box_ptr(wire_OnlyForApi1Struct::new_with_null_ptr())
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<OnlyForApi1Struct> for *mut wire_OnlyForApi1Struct {
    fn wire2api(self) -> OnlyForApi1Struct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OnlyForApi1Struct>::wire2api(*wrap).into()
    }
}

impl Wire2Api<OnlyForApi1Struct> for wire_OnlyForApi1Struct {
    fn wire2api(self) -> OnlyForApi1Struct {
        OnlyForApi1Struct {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_OnlyForApi1Struct {
    id: i16,
    num: f64,
    name: *mut wire_uint_8_list,
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

impl NewWithNullPtr for wire_OnlyForApi1Struct {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_OnlyForApi1Struct {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
