use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_test_inbuilt_type_in_block_1(port_: i64, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_1_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_test_string_in_block_1(port_: i64, s: *mut wire_uint_8_list, i: u64) {
    wire_test_string_in_block_1_impl(port_, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_string_in_sync_in_block_1(
    s: *mut wire_uint_8_list,
    i: u64,
) -> support::WireSyncReturn {
    wire_test_string_in_sync_in_block_1_impl(s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1(
    name: *mut wire_uint_8_list,
    score: f64,
) -> support::WireSyncReturn {
    wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1_impl(name, score)
}

#[no_mangle]
pub extern "C" fn wire_test_all_shared_struct_in_block_1(
    port_: i64,
    custom: *mut wire_SharedStructInAllBlocks,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_all_shared_struct_in_block_1_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_shared_struct_in_block_1_for_1_and_2(
    port_: i64,
    custom: *mut wire_SharedStructInBlock1And2,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_shared_struct_in_block_1_for_1_and_2_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_cross_shared_struct_in_block_1_for_1_and_2(
    port_: i64,
    custom: *mut wire_CrossSharedStructInBlock1And2,
) {
    wire_test_cross_shared_struct_in_block_1_for_1_and_2_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_unique_struct_1(
    port_: i64,
    custom: *mut wire_StructOnlyForBlock1,
    s: *mut wire_uint_8_list,
    i: i8,
) {
    wire_test_unique_struct_1_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_struct_defined_in_block_1(
    port_: i64,
    custom: *mut wire_StructDefinedInBlock1,
) {
    wire_test_struct_defined_in_block_1_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__StructDefinedInBlock1(
    port_: i64,
    that: *mut wire_StructDefinedInBlock1,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__StructDefinedInBlock1_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructDefinedInBlock1(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructDefinedInBlock1_impl(port_, message)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_i8(value: i8) -> *mut i8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_defined_in_block_1() -> *mut wire_StructDefinedInBlock1 {
    support::new_leak_box_ptr(wire_StructDefinedInBlock1::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_only_for_block_1() -> *mut wire_StructOnlyForBlock1 {
    support::new_leak_box_ptr(wire_StructOnlyForBlock1::new_with_null_ptr())
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<i8> for *mut i8 {
    fn wire2api(self) -> i8 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<StructDefinedInBlock1> for *mut wire_StructDefinedInBlock1 {
    fn wire2api(self) -> StructDefinedInBlock1 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructDefinedInBlock1>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructOnlyForBlock1> for *mut wire_StructOnlyForBlock1 {
    fn wire2api(self) -> StructOnlyForBlock1 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructOnlyForBlock1>::wire2api(*wrap).into()
    }
}

impl Wire2Api<StructDefinedInBlock1> for wire_StructDefinedInBlock1 {
    fn wire2api(self) -> StructDefinedInBlock1 {
        StructDefinedInBlock1 {
            name: bridge_generated_shares::Wire2Api::wire2api(self.name),
        }
    }
}
impl Wire2Api<StructOnlyForBlock1> for wire_StructOnlyForBlock1 {
    fn wire2api(self) -> StructOnlyForBlock1 {
        StructOnlyForBlock1 {
            id: self.id.wire2api(),
            num: bridge_generated_shares::Wire2Api::wire2api(self.num),
            name: bridge_generated_shares::Wire2Api::wire2api(self.name),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StructDefinedInBlock1 {
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_StructOnlyForBlock1 {
    id: *mut i8,
    num: *mut f64,
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

impl NewWithNullPtr for wire_StructDefinedInBlock1 {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_StructDefinedInBlock1 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_StructOnlyForBlock1 {
    fn new_with_null_ptr() -> Self {
        Self {
            id: core::ptr::null_mut(),
            num: core::ptr::null_mut(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_StructOnlyForBlock1 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility
