use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_test_inbuilt_type_in_block_3(port_: i64, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_3_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_test_string_in_block_3(port_: i64, s: *mut wire_uint_8_list, i: u64) {
    wire_test_string_in_block_3_impl(port_, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3(
    port_: i64,
    score: f64,
) {
    wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3_impl(port_, score)
}

#[no_mangle]
pub extern "C" fn wire_test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3(
    port_: i64,
    obj: *mut wire_SharedStructOnlyForSyncTest,
    default_score: f64,
) {
    wire_test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3_impl(
        port_,
        obj,
        default_score,
    )
}

#[no_mangle]
pub extern "C" fn wire_test_all_shared_struct_in_block_3(
    port_: i64,
    custom: *mut wire_SharedStructInAllBlocks,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_all_shared_struct_in_block_3_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_shared_struct_in_block_3_for_2_and_3(
    port_: i64,
    custom: *mut wire_SharedStructInBlock2And3,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_shared_struct_in_block_3_for_2_and_3_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_cross_shared_struct_in_block_3_for_2_and_3(
    port_: i64,
    name: *mut wire_uint_8_list,
) {
    wire_test_cross_shared_struct_in_block_3_for_2_and_3_impl(port_, name)
}

#[no_mangle]
pub extern "C" fn wire_test_cross_shared_struct_in_sync_in_block_3_for_2_and_3(
    name: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_test_cross_shared_struct_in_sync_in_block_3_for_2_and_3_impl(name)
}

#[no_mangle]
pub extern "C" fn wire_test_unique_struct_3(
    port_: i64,
    custom: *mut wire_StructOnlyForBlock3,
    s: *mut wire_uint_8_list,
    i: i64,
) {
    wire_test_unique_struct_3_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_struct_defined_in_block_3(
    port_: i64,
    custom: *mut wire_StructDefinedInBlock3,
) {
    wire_test_struct_defined_in_block_3_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__StructDefinedInBlock3(
    port_: i64,
    that: *mut wire_StructDefinedInBlock3,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__StructDefinedInBlock3_impl(port_, that, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructDefinedInBlock3(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructDefinedInBlock3_impl(port_, message)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_defined_in_block_3() -> *mut wire_StructDefinedInBlock3 {
    support::new_leak_box_ptr(wire_StructDefinedInBlock3::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_only_for_block_3() -> *mut wire_StructOnlyForBlock3 {
    support::new_leak_box_ptr(wire_StructOnlyForBlock3::new_with_null_ptr())
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<StructDefinedInBlock3> for *mut wire_StructDefinedInBlock3 {
    fn wire2api(self) -> StructDefinedInBlock3 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructDefinedInBlock3>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructOnlyForBlock3> for *mut wire_StructOnlyForBlock3 {
    fn wire2api(self) -> StructOnlyForBlock3 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructOnlyForBlock3>::wire2api(*wrap).into()
    }
}

impl Wire2Api<StructDefinedInBlock3> for wire_StructDefinedInBlock3 {
    fn wire2api(self) -> StructDefinedInBlock3 {
        StructDefinedInBlock3 {
            name: bridge_generated_shares::Wire2Api::wire2api(self.name),
        }
    }
}
impl Wire2Api<StructOnlyForBlock3> for wire_StructOnlyForBlock3 {
    fn wire2api(self) -> StructOnlyForBlock3 {
        StructOnlyForBlock3 {
            id: self.id.wire2api(),
            num: bridge_generated_shares::Wire2Api::wire2api(self.num),
            name: bridge_generated_shares::Wire2Api::wire2api(self.name),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StructDefinedInBlock3 {
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_StructOnlyForBlock3 {
    id: i64,
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

impl NewWithNullPtr for wire_StructDefinedInBlock3 {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_StructDefinedInBlock3 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_StructOnlyForBlock3 {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_StructOnlyForBlock3 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility
