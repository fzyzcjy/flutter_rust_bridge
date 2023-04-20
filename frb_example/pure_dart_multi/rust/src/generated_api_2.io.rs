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

#[no_mangle]
pub extern "C" fn wire_test_struct_defined_in_api_2(
    port_: i64,
    custom: *mut wire_StructDefinedInApi2,
) {
    wire_test_struct_defined_in_api_2_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__StructDefinedInApi2(
    port_: i64,
    that: *mut wire_StructDefinedInApi2,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__StructDefinedInApi2_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructDefinedInApi2(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructDefinedInApi2_impl(port_, message)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_only_for_api_2_struct() -> *mut wire_OnlyForApi2Struct {
    support::new_leak_box_ptr(wire_OnlyForApi2Struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_defined_in_api_2() -> *mut wire_StructDefinedInApi2 {
    support::new_leak_box_ptr(wire_StructDefinedInApi2::new_with_null_ptr())
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<OnlyForApi2Struct> for *mut wire_OnlyForApi2Struct {
    fn wire2api(self) -> OnlyForApi2Struct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OnlyForApi2Struct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructDefinedInApi2> for *mut wire_StructDefinedInApi2 {
    fn wire2api(self) -> StructDefinedInApi2 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructDefinedInApi2>::wire2api(*wrap).into()
    }
}

impl Wire2Api<OnlyForApi2Struct> for wire_OnlyForApi2Struct {
    fn wire2api(self) -> OnlyForApi2Struct {
        OnlyForApi2Struct {
            id: self.id.wire2api(),
            num: bridge_generated_shares::Wire2Api::wire2api(self.num),
            name: bridge_generated_shares::Wire2Api::wire2api(self.name),
        }
    }
}
impl Wire2Api<StructDefinedInApi2> for wire_StructDefinedInApi2 {
    fn wire2api(self) -> StructDefinedInApi2 {
        StructDefinedInApi2 {
            name: bridge_generated_shares::Wire2Api::wire2api(self.name),
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
pub struct wire_StructDefinedInApi2 {
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

impl NewWithNullPtr for wire_StructDefinedInApi2 {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_StructDefinedInApi2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility
