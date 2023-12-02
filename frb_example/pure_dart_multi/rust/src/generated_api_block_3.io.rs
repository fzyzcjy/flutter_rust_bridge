use super::*;
// Section: wire functions

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
pub extern "C" fn wire_test_enum_defined_in_block_3(
    port_: i64,
    custom: *mut wire_EnumDefinedInBlock3,
) {
    wire_test_enum_defined_in_block_3_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_inbuilt_type_in_block_3(port_: i64, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_3_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_test_list_in_block_3(
    port_: i64,
    shared_structs: *mut wire_list_shared_struct_in_all_blocks,
    strings: *mut wire_StringList,
    nums: *mut wire_int_32_list,
    weekdays: *mut wire_list_shared_weekdays_enum_in_all_blocks,
    struct_list: *mut wire_list_struct_defined_in_block_3,
    enum_list: *mut wire_list_enum_defined_in_block_3,
) {
    wire_test_list_in_block_3_impl(
        port_,
        shared_structs,
        strings,
        nums,
        weekdays,
        struct_list,
        enum_list,
    )
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__EnumDefinedInBlock3(
    port_: i64,
    that: *mut wire_EnumDefinedInBlock3,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__EnumDefinedInBlock3_impl(port_, that, message)
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
pub extern "C" fn wire_test_method__method__StructOnlyForBlock3(
    port_: i64,
    that: *mut wire_StructOnlyForBlock3,
    message: *mut wire_uint_8_list,
    num: u16,
) {
    wire_test_method__method__StructOnlyForBlock3_impl(port_, that, message, num)
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
pub extern "C" fn wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3(
    port_: i64,
    name: *mut wire_uint_8_list,
    score: f64,
) {
    wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3_impl(port_, name, score)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__EnumDefinedInBlock3(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__EnumDefinedInBlock3_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructDefinedInBlock3(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructDefinedInBlock3_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructOnlyForBlock3(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructOnlyForBlock3_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_string_in_block_3(port_: i64, s: *mut wire_uint_8_list, i: u64) {
    wire_test_string_in_block_3_impl(port_, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_struct_defined_in_block_3(
    port_: i64,
    custom: *mut wire_StructDefinedInBlock3,
) {
    wire_test_struct_defined_in_block_3_impl(port_, custom)
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

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_defined_in_block_3() -> *mut wire_EnumDefinedInBlock3 {
    support::new_leak_box_ptr(wire_EnumDefinedInBlock3::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_defined_in_block_3() -> *mut wire_StructDefinedInBlock3 {
    support::new_leak_box_ptr(wire_StructDefinedInBlock3::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_only_for_block_3() -> *mut wire_StructOnlyForBlock3 {
    support::new_leak_box_ptr(wire_StructOnlyForBlock3::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_enum_defined_in_block_3(
    len: i32,
) -> *mut wire_list_enum_defined_in_block_3 {
    let wrap = wire_list_enum_defined_in_block_3 {
        ptr: support::new_leak_vec_ptr(<wire_EnumDefinedInBlock3>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_struct_defined_in_block_3(
    len: i32,
) -> *mut wire_list_struct_defined_in_block_3 {
    let wrap = wire_list_struct_defined_in_block_3 {
        ptr: support::new_leak_vec_ptr(<wire_StructDefinedInBlock3>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<EnumDefinedInBlock3> for *mut wire_EnumDefinedInBlock3 {
    fn wire2api(self) -> EnumDefinedInBlock3 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumDefinedInBlock3>::wire2api(*wrap).into()
    }
}
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
impl Wire2Api<EnumDefinedInBlock3> for wire_EnumDefinedInBlock3 {
    fn wire2api(self) -> EnumDefinedInBlock3 {
        match self.tag {
            0 => EnumDefinedInBlock3::Quit,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Move);
                EnumDefinedInBlock3::Move {
                    x: bridge_generated_shared::Wire2Api::wire2api(ans.x),
                    y: bridge_generated_shared::Wire2Api::wire2api(ans.y),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Write);
                EnumDefinedInBlock3::Write(bridge_generated_shared::Wire2Api::wire2api(ans.field0))
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.ChangeColor);
                EnumDefinedInBlock3::ChangeColor(
                    bridge_generated_shared::Wire2Api::wire2api(ans.field0),
                    bridge_generated_shared::Wire2Api::wire2api(ans.field1),
                    bridge_generated_shared::Wire2Api::wire2api(ans.field2),
                )
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Vec<EnumDefinedInBlock3>> for *mut wire_list_enum_defined_in_block_3 {
    fn wire2api(self) -> Vec<EnumDefinedInBlock3> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<StructDefinedInBlock3>> for *mut wire_list_struct_defined_in_block_3 {
    fn wire2api(self) -> Vec<StructDefinedInBlock3> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<StructDefinedInBlock3> for wire_StructDefinedInBlock3 {
    fn wire2api(self) -> StructDefinedInBlock3 {
        StructDefinedInBlock3 {
            name: bridge_generated_shared::Wire2Api::wire2api(self.name),
        }
    }
}
impl Wire2Api<StructOnlyForBlock3> for wire_StructOnlyForBlock3 {
    fn wire2api(self) -> StructOnlyForBlock3 {
        StructOnlyForBlock3 {
            id: self.id.wire2api(),
            num: bridge_generated_shared::Wire2Api::wire2api(self.num),
            name: bridge_generated_shared::Wire2Api::wire2api(self.name),
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

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_enum_defined_in_block_3 {
    ptr: *mut wire_EnumDefinedInBlock3,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_struct_defined_in_block_3 {
    ptr: *mut wire_StructDefinedInBlock3,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock3 {
    tag: i32,
    kind: *mut EnumDefinedInBlock3Kind,
}

#[repr(C)]
pub union EnumDefinedInBlock3Kind {
    Quit: *mut wire_EnumDefinedInBlock3_Quit,
    Move: *mut wire_EnumDefinedInBlock3_Move,
    Write: *mut wire_EnumDefinedInBlock3_Write,
    ChangeColor: *mut wire_EnumDefinedInBlock3_ChangeColor,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock3_Quit {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock3_Move {
    x: i32,
    y: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock3_Write {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock3_ChangeColor {
    field0: i32,
    field1: i32,
    field2: i32,
}

// Section: impl NewWithNullPtr

impl Default for wire_EnumDefinedInBlock3 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_EnumDefinedInBlock3 {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_EnumDefinedInBlock3_Move() -> *mut EnumDefinedInBlock3Kind {
    support::new_leak_box_ptr(EnumDefinedInBlock3Kind {
        Move: support::new_leak_box_ptr(wire_EnumDefinedInBlock3_Move {
            x: Default::default(),
            y: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDefinedInBlock3_Write() -> *mut EnumDefinedInBlock3Kind {
    support::new_leak_box_ptr(EnumDefinedInBlock3Kind {
        Write: support::new_leak_box_ptr(wire_EnumDefinedInBlock3_Write {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDefinedInBlock3_ChangeColor() -> *mut EnumDefinedInBlock3Kind {
    support::new_leak_box_ptr(EnumDefinedInBlock3Kind {
        ChangeColor: support::new_leak_box_ptr(wire_EnumDefinedInBlock3_ChangeColor {
            field0: Default::default(),
            field1: Default::default(),
            field2: Default::default(),
        }),
    })
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
