use super::*;
// Section: wire functions

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
pub extern "C" fn wire_test_cross_shared_struct_in_block_2_for_1_and_2(
    port_: i64,
    name: *mut wire_uint_8_list,
) {
    wire_test_cross_shared_struct_in_block_2_for_1_and_2_impl(port_, name)
}

#[no_mangle]
pub extern "C" fn wire_test_cross_shared_struct_in_block_2_for_2_and_3(
    port_: i64,
    custom: *mut wire_CrossSharedStructInBlock2And3,
) {
    wire_test_cross_shared_struct_in_block_2_for_2_and_3_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_enum_defined_in_block_2(
    port_: i64,
    custom: *mut wire_EnumDefinedInBlock2,
) {
    wire_test_enum_defined_in_block_2_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_inbuilt_type_in_block_2(port_: i64, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_2_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_test_list_in_block_2(
    port_: i64,
    shared_structs: *mut wire_list_shared_struct_in_all_blocks,
    strings: *mut wire_StringList,
    nums: *mut wire_int_32_list,
    weekdays: *mut wire_list_shared_weekdays_enum_in_all_blocks,
    struct_list: *mut wire_list_struct_defined_in_block_2,
    enum_list: *mut wire_list_enum_defined_in_block_2,
) {
    wire_test_list_in_block_2_impl(
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
pub extern "C" fn wire_test_method__method__EnumDefinedInBlock2(
    port_: i64,
    that: *mut wire_EnumDefinedInBlock2,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__EnumDefinedInBlock2_impl(port_, that, message)
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
pub extern "C" fn wire_test_method__method__StructOnlyForBlock2(
    port_: i64,
    that: *mut wire_StructOnlyForBlock2,
    message: *mut wire_uint_8_list,
    num: u16,
) {
    wire_test_method__method__StructOnlyForBlock2_impl(port_, that, message, num)
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
pub extern "C" fn wire_test_shared_struct_in_block_2_for_2_and_3(
    port_: i64,
    custom: *mut wire_SharedStructInBlock2And3,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_shared_struct_in_block_2_for_2_and_3_impl(port_, custom, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__EnumDefinedInBlock2(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__EnumDefinedInBlock2_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructDefinedInBlock2(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructDefinedInBlock2_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructOnlyForBlock2(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructOnlyForBlock2_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_string_in_block_2(port_: i64, s: *mut wire_uint_8_list, i: u64) {
    wire_test_string_in_block_2_impl(port_, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_struct_defined_in_block_2(
    port_: i64,
    custom: *mut wire_StructDefinedInBlock2,
) {
    wire_test_struct_defined_in_block_2_impl(port_, custom)
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

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_defined_in_block_2() -> *mut wire_EnumDefinedInBlock2 {
    support::new_leak_box_ptr(wire_EnumDefinedInBlock2::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_defined_in_block_2() -> *mut wire_StructDefinedInBlock2 {
    support::new_leak_box_ptr(wire_StructDefinedInBlock2::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_only_for_block_2() -> *mut wire_StructOnlyForBlock2 {
    support::new_leak_box_ptr(wire_StructOnlyForBlock2::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_enum_defined_in_block_2(
    len: i32,
) -> *mut wire_list_enum_defined_in_block_2 {
    let wrap = wire_list_enum_defined_in_block_2 {
        ptr: support::new_leak_vec_ptr(<wire_EnumDefinedInBlock2>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_struct_defined_in_block_2(
    len: i32,
) -> *mut wire_list_struct_defined_in_block_2 {
    let wrap = wire_list_struct_defined_in_block_2 {
        ptr: support::new_leak_vec_ptr(<wire_StructDefinedInBlock2>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<EnumDefinedInBlock2> for *mut wire_EnumDefinedInBlock2 {
    fn wire2api(self) -> EnumDefinedInBlock2 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumDefinedInBlock2>::wire2api(*wrap).into()
    }
}
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
impl Wire2Api<EnumDefinedInBlock2> for wire_EnumDefinedInBlock2 {
    fn wire2api(self) -> EnumDefinedInBlock2 {
        match self.tag {
            0 => EnumDefinedInBlock2::Quit,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Move);
                EnumDefinedInBlock2::Move {
                    x: bridge_generated_shared::Wire2Api::wire2api(ans.x),
                    y: bridge_generated_shared::Wire2Api::wire2api(ans.y),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Write);
                EnumDefinedInBlock2::Write(bridge_generated_shared::Wire2Api::wire2api(ans.field0))
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.ChangeColor);
                EnumDefinedInBlock2::ChangeColor(
                    bridge_generated_shared::Wire2Api::wire2api(ans.field0),
                    bridge_generated_shared::Wire2Api::wire2api(ans.field1),
                    bridge_generated_shared::Wire2Api::wire2api(ans.field2),
                )
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Vec<EnumDefinedInBlock2>> for *mut wire_list_enum_defined_in_block_2 {
    fn wire2api(self) -> Vec<EnumDefinedInBlock2> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<StructDefinedInBlock2>> for *mut wire_list_struct_defined_in_block_2 {
    fn wire2api(self) -> Vec<StructDefinedInBlock2> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<StructDefinedInBlock2> for wire_StructDefinedInBlock2 {
    fn wire2api(self) -> StructDefinedInBlock2 {
        StructDefinedInBlock2 {
            name: bridge_generated_shared::Wire2Api::wire2api(self.name),
        }
    }
}
impl Wire2Api<StructOnlyForBlock2> for wire_StructOnlyForBlock2 {
    fn wire2api(self) -> StructOnlyForBlock2 {
        StructOnlyForBlock2 {
            id: self.id.wire2api(),
            num: bridge_generated_shared::Wire2Api::wire2api(self.num),
            name: bridge_generated_shared::Wire2Api::wire2api(self.name),
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

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_enum_defined_in_block_2 {
    ptr: *mut wire_EnumDefinedInBlock2,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_struct_defined_in_block_2 {
    ptr: *mut wire_StructDefinedInBlock2,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock2 {
    tag: i32,
    kind: *mut EnumDefinedInBlock2Kind,
}

#[repr(C)]
pub union EnumDefinedInBlock2Kind {
    Quit: *mut wire_EnumDefinedInBlock2_Quit,
    Move: *mut wire_EnumDefinedInBlock2_Move,
    Write: *mut wire_EnumDefinedInBlock2_Write,
    ChangeColor: *mut wire_EnumDefinedInBlock2_ChangeColor,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock2_Quit {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock2_Move {
    x: i32,
    y: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock2_Write {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock2_ChangeColor {
    field0: i32,
    field1: i32,
    field2: i32,
}

// Section: impl NewWithNullPtr

impl Default for wire_EnumDefinedInBlock2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_EnumDefinedInBlock2 {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_EnumDefinedInBlock2_Move() -> *mut EnumDefinedInBlock2Kind {
    support::new_leak_box_ptr(EnumDefinedInBlock2Kind {
        Move: support::new_leak_box_ptr(wire_EnumDefinedInBlock2_Move {
            x: Default::default(),
            y: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDefinedInBlock2_Write() -> *mut EnumDefinedInBlock2Kind {
    support::new_leak_box_ptr(EnumDefinedInBlock2Kind {
        Write: support::new_leak_box_ptr(wire_EnumDefinedInBlock2_Write {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDefinedInBlock2_ChangeColor() -> *mut EnumDefinedInBlock2Kind {
    support::new_leak_box_ptr(EnumDefinedInBlock2Kind {
        ChangeColor: support::new_leak_box_ptr(wire_EnumDefinedInBlock2_ChangeColor {
            field0: Default::default(),
            field1: Default::default(),
            field2: Default::default(),
        }),
    })
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
