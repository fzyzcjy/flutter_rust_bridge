use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_test_inbuilt_type_in_block_2(port_: i64, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_2_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_test_string_in_block_2(port_: i64, s: *mut wire_uint_8_list, i: u64) {
    wire_test_string_in_block_2_impl(port_, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_all_shared_struct_in_block_2(
    port_: i64,
    custom: *mut wire_SharedStructInAllBlocks,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_all_shared_struct_in_block_2_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_all_shared_struct_in_sync_in_block_2(
    custom: *mut wire_SharedStructInAllBlocks,
    s: *mut wire_uint_8_list,
    i: i32,
) -> support::WireSyncReturn {
    wire_test_all_shared_struct_in_sync_in_block_2_impl(custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_shared_struct_in_block_2_for_1_and_2(
    port_: i64,
    custom: *mut wire_SharedStructInBlock1And2,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_shared_struct_in_block_2_for_1_and_2_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_cross_shared_struct_in_block_2_for_1_and_2(
    port_: i64,
    name: *mut wire_uint_8_list,
) {
    wire_test_cross_shared_struct_in_block_2_for_1_and_2_impl(port_, name)
}

#[no_mangle]
pub extern "C" fn wire_test_shared_struct_in_block_2_for_2_and_3(
    port_: i64,
    custom: *mut wire_SharedStructInBlock2And3,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_shared_struct_in_block_2_for_2_and_3_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_cross_shared_struct_in_block_2_for_2_and_3(
    port_: i64,
    custom: *mut wire_CrossSharedStructInBlock2And3,
) {
    wire_test_cross_shared_struct_in_block_2_for_2_and_3_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_unique_struct_2(
    port_: i64,
    custom: *mut wire_StructOnlyForBlock2,
    s: *mut wire_uint_8_list,
    i: i16,
) {
    wire_test_unique_struct_2_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_struct_defined_in_block_2(
    port_: i64,
    custom: *mut wire_StructDefinedInBlock2,
) {
    wire_test_struct_defined_in_block_2_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__StructDefinedInBlock2(
    port_: i64,
    that: *mut wire_StructDefinedInBlock2,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__StructDefinedInBlock2_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructDefinedInBlock2(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructDefinedInBlock2_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__StructOnlyForBlock2(
    port_: i64,
    that: *mut wire_StructOnlyForBlock2,
    message: *mut wire_uint_8_list,
    num: u16,
) {
    wire_test_method__method__StructOnlyForBlock2_impl(port_, that, message, num)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructOnlyForBlock2(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructOnlyForBlock2_impl(port_, message)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_defined_in_block_2() -> *mut wire_StructDefinedInBlock2 {
    support::new_leak_box_ptr(wire_StructDefinedInBlock2::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_only_for_block_2() -> *mut wire_StructOnlyForBlock2 {
    support::new_leak_box_ptr(wire_StructOnlyForBlock2::new_with_null_ptr())
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<StructDefinedInBlock2> for *mut wire_StructDefinedInBlock2 {
    fn wire2api(self) -> StructDefinedInBlock2 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructDefinedInBlock2>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructOnlyForBlock2> for *mut wire_StructOnlyForBlock2 {
    fn wire2api(self) -> StructOnlyForBlock2 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructOnlyForBlock2>::wire2api(*wrap).into()
    }
}

impl Wire2Api<StructDefinedInBlock2> for wire_StructDefinedInBlock2 {
    fn wire2api(self) -> StructDefinedInBlock2 {
        StructDefinedInBlock2 {
            name: bridge_generated_shares::Wire2Api::wire2api(self.name),
        }
    }
}
impl Wire2Api<StructOnlyForBlock2> for wire_StructOnlyForBlock2 {
    fn wire2api(self) -> StructOnlyForBlock2 {
        StructOnlyForBlock2 {
            id: self.id.wire2api(),
            num: bridge_generated_shares::Wire2Api::wire2api(self.num),
            name: bridge_generated_shares::Wire2Api::wire2api(self.name),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StructDefinedInBlock2 {
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_StructOnlyForBlock2 {
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

impl NewWithNullPtr for wire_StructDefinedInBlock2 {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_StructDefinedInBlock2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_StructOnlyForBlock2 {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_StructOnlyForBlock2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility
