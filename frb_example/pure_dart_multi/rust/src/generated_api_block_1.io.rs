use super::*;
// Section: wire functions

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
pub extern "C" fn wire_test_cross_shared_struct_in_block_1_for_1_and_2(
    port_: i64,
    custom: *mut wire_CrossSharedStructInBlock1And2,
) {
    wire_test_cross_shared_struct_in_block_1_for_1_and_2_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_enum_defined_in_block_1(
    port_: i64,
    custom: *mut wire_EnumDefinedInBlock1,
) {
    wire_test_enum_defined_in_block_1_impl(port_, custom)
}

#[no_mangle]
pub extern "C" fn wire_test_inbuilt_type_in_block_1(port_: i64, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_1_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_test_list_in_block_1(
    port_: i64,
    shared_structs: *mut wire_list_shared_struct_in_all_blocks,
    strings: *mut wire_StringList,
    nums: *mut wire_int_32_list,
    weekdays: *mut wire_list_shared_weekdays_enum_in_all_blocks,
    struct_list: *mut wire_list_struct_defined_in_block_1,
    enum_list: *mut wire_list_enum_defined_in_block_1,
) {
    wire_test_list_in_block_1_impl(
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
pub extern "C" fn wire_test_method__method__EnumDefinedInBlock1(
    port_: i64,
    that: *mut wire_EnumDefinedInBlock1,
    message: *mut wire_uint_8_list,
) {
    wire_test_method__method__EnumDefinedInBlock1_impl(port_, that, message)
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
pub extern "C" fn wire_test_method__method__StructOnlyForBlock1(
    port_: i64,
    that: *mut wire_StructOnlyForBlock1,
    message: *mut wire_uint_8_list,
    num: u16,
) {
    wire_test_method__method__StructOnlyForBlock1_impl(port_, that, message, num)
}

#[no_mangle]
pub extern "C" fn wire_test_optional_string_in_block_1(
    port_: i64,
    s: *mut wire_uint_8_list,
    i: i32,
) {
    wire_test_optional_string_in_block_1_impl(port_, s, i)
}

#[no_mangle]
pub extern "C" fn wire_test_optional_string_in_sync_in_block_1(
    s: *mut wire_uint_8_list,
    i: i32,
) -> support::WireSyncReturn {
    wire_test_optional_string_in_sync_in_block_1_impl(s, i)
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
pub extern "C" fn wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1(
    name: *mut wire_uint_8_list,
    score: f64,
) -> support::WireSyncReturn {
    wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1_impl(name, score)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__EnumDefinedInBlock1(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__EnumDefinedInBlock1_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructDefinedInBlock1(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructDefinedInBlock1_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_test_static_method__static_method__StructOnlyForBlock1(
    port_: i64,
    message: *mut wire_uint_8_list,
) {
    wire_test_static_method__static_method__StructOnlyForBlock1_impl(port_, message)
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
pub extern "C" fn wire_test_struct_defined_in_block_1(
    port_: i64,
    custom: *mut wire_StructDefinedInBlock1,
) {
    wire_test_struct_defined_in_block_1_impl(port_, custom)
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

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_defined_in_block_1() -> *mut wire_EnumDefinedInBlock1 {
    support::new_leak_box_ptr(wire_EnumDefinedInBlock1::new_with_null_ptr())
}

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

#[no_mangle]
pub extern "C" fn new_list_enum_defined_in_block_1(
    len: i32,
) -> *mut wire_list_enum_defined_in_block_1 {
    let wrap = wire_list_enum_defined_in_block_1 {
        ptr: support::new_leak_vec_ptr(<wire_EnumDefinedInBlock1>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_struct_defined_in_block_1(
    len: i32,
) -> *mut wire_list_struct_defined_in_block_1 {
    let wrap = wire_list_struct_defined_in_block_1 {
        ptr: support::new_leak_vec_ptr(<wire_StructDefinedInBlock1>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<EnumDefinedInBlock1> for *mut wire_EnumDefinedInBlock1 {
    fn wire2api(self) -> EnumDefinedInBlock1 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumDefinedInBlock1>::wire2api(*wrap).into()
    }
}
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
impl Wire2Api<EnumDefinedInBlock1> for wire_EnumDefinedInBlock1 {
    fn wire2api(self) -> EnumDefinedInBlock1 {
        match self.tag {
            0 => EnumDefinedInBlock1::Quit,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Move);
                EnumDefinedInBlock1::Move {
                    x: bridge_generated_shared::Wire2Api::wire2api(ans.x),
                    y: bridge_generated_shared::Wire2Api::wire2api(ans.y),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Write);
                EnumDefinedInBlock1::Write(bridge_generated_shared::Wire2Api::wire2api(ans.field0))
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.ChangeColor);
                EnumDefinedInBlock1::ChangeColor(
                    bridge_generated_shared::Wire2Api::wire2api(ans.field0),
                    bridge_generated_shared::Wire2Api::wire2api(ans.field1),
                    bridge_generated_shared::Wire2Api::wire2api(ans.field2),
                )
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Vec<EnumDefinedInBlock1>> for *mut wire_list_enum_defined_in_block_1 {
    fn wire2api(self) -> Vec<EnumDefinedInBlock1> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<StructDefinedInBlock1>> for *mut wire_list_struct_defined_in_block_1 {
    fn wire2api(self) -> Vec<StructDefinedInBlock1> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<StructDefinedInBlock1> for wire_StructDefinedInBlock1 {
    fn wire2api(self) -> StructDefinedInBlock1 {
        StructDefinedInBlock1 {
            name: bridge_generated_shared::Wire2Api::wire2api(self.name),
        }
    }
}
impl Wire2Api<StructOnlyForBlock1> for wire_StructOnlyForBlock1 {
    fn wire2api(self) -> StructOnlyForBlock1 {
        StructOnlyForBlock1 {
            id: self.id.wire2api(),
            num: bridge_generated_shared::Wire2Api::wire2api(self.num),
            name: bridge_generated_shared::Wire2Api::wire2api(self.name),
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

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_enum_defined_in_block_1 {
    ptr: *mut wire_EnumDefinedInBlock1,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_struct_defined_in_block_1 {
    ptr: *mut wire_StructDefinedInBlock1,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock1 {
    tag: i32,
    kind: *mut EnumDefinedInBlock1Kind,
}

#[repr(C)]
pub union EnumDefinedInBlock1Kind {
    Quit: *mut wire_EnumDefinedInBlock1_Quit,
    Move: *mut wire_EnumDefinedInBlock1_Move,
    Write: *mut wire_EnumDefinedInBlock1_Write,
    ChangeColor: *mut wire_EnumDefinedInBlock1_ChangeColor,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock1_Quit {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock1_Move {
    x: i32,
    y: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock1_Write {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDefinedInBlock1_ChangeColor {
    field0: i32,
    field1: i32,
    field2: i32,
}

// Section: impl NewWithNullPtr

impl Default for wire_EnumDefinedInBlock1 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_EnumDefinedInBlock1 {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_EnumDefinedInBlock1_Move() -> *mut EnumDefinedInBlock1Kind {
    support::new_leak_box_ptr(EnumDefinedInBlock1Kind {
        Move: support::new_leak_box_ptr(wire_EnumDefinedInBlock1_Move {
            x: Default::default(),
            y: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDefinedInBlock1_Write() -> *mut EnumDefinedInBlock1Kind {
    support::new_leak_box_ptr(EnumDefinedInBlock1Kind {
        Write: support::new_leak_box_ptr(wire_EnumDefinedInBlock1_Write {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDefinedInBlock1_ChangeColor() -> *mut EnumDefinedInBlock1Kind {
    support::new_leak_box_ptr(EnumDefinedInBlock1Kind {
        ChangeColor: support::new_leak_box_ptr(wire_EnumDefinedInBlock1_ChangeColor {
            field0: Default::default(),
            field1: Default::default(),
            field2: Default::default(),
        }),
    })
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
