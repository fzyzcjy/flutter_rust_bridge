use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_test_inbuilt_type_2(port_: i64, a: i32, b: f32) {
    wire_test_inbuilt_type_2_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_test_string_2(port_: i64, s: *mut wire_uint_8_list, i: u64) {
    wire_test_string_2_impl(port_, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_shared_struct_2(
    port_: i64,
    custom: *mut wire_SharedStruct,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_shared_struct_2_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_unique_struct_2(
    port_: i64,
    custom: *mut wire_OnlyForApi2Struct,
    s: *mut wire_uint_8_list,
    i: i64,
) {
    wire_test_unique_struct_2_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_cross_shared_struct_2(port_: i64, name: *mut wire_uint_8_list) {
    wire_test_cross_shared_struct_2_impl(port_, name)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_only_for_api_2_struct_1() -> *mut wire_OnlyForApi2Struct {
    support::new_leak_box_ptr(wire_OnlyForApi2Struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct_1() -> *mut wire_SharedStruct {
    support::new_leak_box_ptr(wire_SharedStruct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_1(len: i32) -> *mut wire_uint_8_list {
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
impl Wire2Api<OnlyForApi2Struct> for *mut wire_OnlyForApi2Struct {
    fn wire2api(self) -> OnlyForApi2Struct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OnlyForApi2Struct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SharedStruct> for *mut wire_SharedStruct {
    fn wire2api(self) -> SharedStruct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStruct>::wire2api(*wrap).into()
    }
}

impl Wire2Api<OnlyForApi2Struct> for wire_OnlyForApi2Struct {
    fn wire2api(self) -> OnlyForApi2Struct {
        OnlyForApi2Struct {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
        }
    }
}
impl Wire2Api<SharedStruct> for wire_SharedStruct {
    fn wire2api(self) -> SharedStruct {
        SharedStruct {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
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
pub struct wire_OnlyForApi2Struct {
    id: i64,
    num: f64,
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStruct {
    id: i32,
    num: f64,
    name: *mut wire_uint_8_list,
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

impl NewWithNullPtr for wire_OnlyForApi2Struct {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_OnlyForApi2Struct {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SharedStruct {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SharedStruct {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
