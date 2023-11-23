use super::*;

// Section: impl_wire2api

impl Wire2Api<String> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<CustomNestedErrorInnerTwinNormal>
    for *mut wire_custom_nested_error_inner_twin_normal
{
    fn wire2api(self) -> CustomNestedErrorInnerTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CustomNestedErrorInnerTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CustomNestedErrorOuterTwinNormal>
    for *mut wire_custom_nested_error_outer_twin_normal
{
    fn wire2api(self) -> CustomNestedErrorOuterTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CustomNestedErrorOuterTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CustomStructErrorTwinNormal> for *mut wire_custom_struct_error_twin_normal {
    fn wire2api(self) -> CustomStructErrorTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CustomStructErrorTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EnumWithItemMixedTwinNormal> for *mut wire_enum_with_item_mixed_twin_normal {
    fn wire2api(self) -> EnumWithItemMixedTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumWithItemMixedTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EnumWithItemMixedTwinSync> for *mut wire_enum_with_item_mixed_twin_sync {
    fn wire2api(self) -> EnumWithItemMixedTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumWithItemMixedTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EnumWithItemStructTwinNormal> for *mut wire_enum_with_item_struct_twin_normal {
    fn wire2api(self) -> EnumWithItemStructTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumWithItemStructTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EnumWithItemStructTwinSync> for *mut wire_enum_with_item_struct_twin_sync {
    fn wire2api(self) -> EnumWithItemStructTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumWithItemStructTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EnumWithItemTupleTwinNormal> for *mut wire_enum_with_item_tuple_twin_normal {
    fn wire2api(self) -> EnumWithItemTupleTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumWithItemTupleTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EnumWithItemTupleTwinSync> for *mut wire_enum_with_item_tuple_twin_sync {
    fn wire2api(self) -> EnumWithItemTupleTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EnumWithItemTupleTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<f32> for *mut f32 {
    fn wire2api(self) -> f32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i16> for *mut i16 {
    fn wire2api(self) -> i16 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i8> for *mut i8 {
    fn wire2api(self) -> i8 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<MacroStruct> for *mut wire_macro_struct {
    fn wire2api(self) -> MacroStruct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MacroStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithCommentsTwinNormal> for *mut wire_struct_with_comments_twin_normal {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithCommentsTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithCommentsTwinSync> for *mut wire_struct_with_comments_twin_sync {
    fn wire2api(self) -> StructWithCommentsTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithCommentsTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithOneFieldTwinNormal> for *mut wire_struct_with_one_field_twin_normal {
    fn wire2api(self) -> StructWithOneFieldTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithOneFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithOneFieldTwinSync> for *mut wire_struct_with_one_field_twin_sync {
    fn wire2api(self) -> StructWithOneFieldTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithOneFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithTwoFieldTwinNormal> for *mut wire_struct_with_two_field_twin_normal {
    fn wire2api(self) -> StructWithTwoFieldTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithTwoFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithTwoFieldTwinSync> for *mut wire_struct_with_two_field_twin_sync {
    fn wire2api(self) -> StructWithTwoFieldTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithTwoFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithZeroFieldTwinNormal> for *mut wire_struct_with_zero_field_twin_normal {
    fn wire2api(self) -> StructWithZeroFieldTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithZeroFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithZeroFieldTwinSync> for *mut wire_struct_with_zero_field_twin_sync {
    fn wire2api(self) -> StructWithZeroFieldTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithZeroFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<TupleStructWithOneFieldTwinNormal>
    for *mut wire_tuple_struct_with_one_field_twin_normal
{
    fn wire2api(self) -> TupleStructWithOneFieldTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TupleStructWithOneFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<TupleStructWithOneFieldTwinSync> for *mut wire_tuple_struct_with_one_field_twin_sync {
    fn wire2api(self) -> TupleStructWithOneFieldTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TupleStructWithOneFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<TupleStructWithTwoFieldTwinNormal>
    for *mut wire_tuple_struct_with_two_field_twin_normal
{
    fn wire2api(self) -> TupleStructWithTwoFieldTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TupleStructWithTwoFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<TupleStructWithTwoFieldTwinSync> for *mut wire_tuple_struct_with_two_field_twin_sync {
    fn wire2api(self) -> TupleStructWithTwoFieldTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TupleStructWithTwoFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u16> for *mut u16 {
    fn wire2api(self) -> u16 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u64> for *mut u64 {
    fn wire2api(self) -> u64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u8> for *mut u8 {
    fn wire2api(self) -> u8 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<CustomNestedErrorInnerTwinNormal> for wire_custom_nested_error_inner_twin_normal {
    fn wire2api(self) -> CustomNestedErrorInnerTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Three);
                CustomNestedErrorInnerTwinNormal::Three(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Four);
                CustomNestedErrorInnerTwinNormal::Four(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<CustomNestedErrorOuterTwinNormal> for wire_custom_nested_error_outer_twin_normal {
    fn wire2api(self) -> CustomNestedErrorOuterTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.One);
                CustomNestedErrorOuterTwinNormal::One(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Two);
                CustomNestedErrorOuterTwinNormal::Two(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<CustomStructErrorTwinNormal> for wire_custom_struct_error_twin_normal {
    fn wire2api(self) -> CustomStructErrorTwinNormal {
        CustomStructErrorTwinNormal {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<EnumWithItemMixedTwinNormal> for wire_enum_with_item_mixed_twin_normal {
    fn wire2api(self) -> EnumWithItemMixedTwinNormal {
        match self.tag {
            0 => EnumWithItemMixedTwinNormal::A,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                EnumWithItemMixedTwinNormal::B(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.C);
                EnumWithItemMixedTwinNormal::C {
                    c_field: ans.c_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemMixedTwinSync> for wire_enum_with_item_mixed_twin_sync {
    fn wire2api(self) -> EnumWithItemMixedTwinSync {
        match self.tag {
            0 => EnumWithItemMixedTwinSync::A,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                EnumWithItemMixedTwinSync::B(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.C);
                EnumWithItemMixedTwinSync::C {
                    c_field: ans.c_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemStructTwinNormal> for wire_enum_with_item_struct_twin_normal {
    fn wire2api(self) -> EnumWithItemStructTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.A);
                EnumWithItemStructTwinNormal::A {
                    a_field: ans.a_field.wire2api(),
                }
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                EnumWithItemStructTwinNormal::B {
                    b_field: ans.b_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemStructTwinSync> for wire_enum_with_item_struct_twin_sync {
    fn wire2api(self) -> EnumWithItemStructTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.A);
                EnumWithItemStructTwinSync::A {
                    a_field: ans.a_field.wire2api(),
                }
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                EnumWithItemStructTwinSync::B {
                    b_field: ans.b_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemTupleTwinNormal> for wire_enum_with_item_tuple_twin_normal {
    fn wire2api(self) -> EnumWithItemTupleTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.A);
                EnumWithItemTupleTwinNormal::A(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                EnumWithItemTupleTwinNormal::B(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemTupleTwinSync> for wire_enum_with_item_tuple_twin_sync {
    fn wire2api(self) -> EnumWithItemTupleTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.A);
                EnumWithItemTupleTwinSync::A(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                EnumWithItemTupleTwinSync::B(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Vec<bool>> for *mut wire_list_bool {
    fn wire2api(self) -> Vec<bool> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<f32>> for *mut wire_list_prim_f_32 {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<f64>> for *mut wire_list_prim_f_64 {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i16>> for *mut wire_list_prim_i_16 {
    fn wire2api(self) -> Vec<i16> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i32>> for *mut wire_list_prim_i_32 {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i64>> for *mut wire_list_prim_i_64 {
    fn wire2api(self) -> Vec<i64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i8>> for *mut wire_list_prim_i_8 {
    fn wire2api(self) -> Vec<i8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u16>> for *mut wire_list_prim_u_16 {
    fn wire2api(self) -> Vec<u16> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u32>> for *mut wire_list_prim_u_32 {
    fn wire2api(self) -> Vec<u32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u64>> for *mut wire_list_prim_u_64 {
    fn wire2api(self) -> Vec<u64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<MacroStruct> for wire_macro_struct {
    fn wire2api(self) -> MacroStruct {
        MacroStruct {
            data: self.data.wire2api(),
        }
    }
}
impl Wire2Api<StructWithCommentsTwinNormal> for wire_struct_with_comments_twin_normal {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        StructWithCommentsTwinNormal {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<StructWithCommentsTwinSync> for wire_struct_with_comments_twin_sync {
    fn wire2api(self) -> StructWithCommentsTwinSync {
        StructWithCommentsTwinSync {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<StructWithOneFieldTwinNormal> for wire_struct_with_one_field_twin_normal {
    fn wire2api(self) -> StructWithOneFieldTwinNormal {
        StructWithOneFieldTwinNormal {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<StructWithOneFieldTwinSync> for wire_struct_with_one_field_twin_sync {
    fn wire2api(self) -> StructWithOneFieldTwinSync {
        StructWithOneFieldTwinSync {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<StructWithTwoFieldTwinNormal> for wire_struct_with_two_field_twin_normal {
    fn wire2api(self) -> StructWithTwoFieldTwinNormal {
        StructWithTwoFieldTwinNormal {
            a: self.a.wire2api(),
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<StructWithTwoFieldTwinSync> for wire_struct_with_two_field_twin_sync {
    fn wire2api(self) -> StructWithTwoFieldTwinSync {
        StructWithTwoFieldTwinSync {
            a: self.a.wire2api(),
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<StructWithZeroFieldTwinNormal> for wire_struct_with_zero_field_twin_normal {
    fn wire2api(self) -> StructWithZeroFieldTwinNormal {
        StructWithZeroFieldTwinNormal {}
    }
}
impl Wire2Api<StructWithZeroFieldTwinSync> for wire_struct_with_zero_field_twin_sync {
    fn wire2api(self) -> StructWithZeroFieldTwinSync {
        StructWithZeroFieldTwinSync {}
    }
}
impl Wire2Api<TupleStructWithOneFieldTwinNormal> for wire_tuple_struct_with_one_field_twin_normal {
    fn wire2api(self) -> TupleStructWithOneFieldTwinNormal {
        TupleStructWithOneFieldTwinNormal(self.field0.wire2api())
    }
}
impl Wire2Api<TupleStructWithOneFieldTwinSync> for wire_tuple_struct_with_one_field_twin_sync {
    fn wire2api(self) -> TupleStructWithOneFieldTwinSync {
        TupleStructWithOneFieldTwinSync(self.field0.wire2api())
    }
}
impl Wire2Api<TupleStructWithTwoFieldTwinNormal> for wire_tuple_struct_with_two_field_twin_normal {
    fn wire2api(self) -> TupleStructWithTwoFieldTwinNormal {
        TupleStructWithTwoFieldTwinNormal(self.field0.wire2api(), self.field1.wire2api())
    }
}
impl Wire2Api<TupleStructWithTwoFieldTwinSync> for wire_tuple_struct_with_two_field_twin_sync {
    fn wire2api(self) -> TupleStructWithTwoFieldTwinSync {
        TupleStructWithTwoFieldTwinSync(self.field0.wire2api(), self.field1.wire2api())
    }
}

// Section: wire2api_class

#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_nested_error_inner_twin_normal {
    tag: i32,
    kind: *mut CustomNestedErrorInnerTwinNormalKind,
}

#[repr(C)]
pub union CustomNestedErrorInnerTwinNormalKind {
    Three: *mut wire_CustomNestedErrorInnerTwinNormal_Three,
    Four: *mut wire_CustomNestedErrorInnerTwinNormal_Four,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorInnerTwinNormal_Three {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorInnerTwinNormal_Four {
    field0: u32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_nested_error_outer_twin_normal {
    tag: i32,
    kind: *mut CustomNestedErrorOuterTwinNormalKind,
}

#[repr(C)]
pub union CustomNestedErrorOuterTwinNormalKind {
    One: *mut wire_CustomNestedErrorOuterTwinNormal_One,
    Two: *mut wire_CustomNestedErrorOuterTwinNormal_Two,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorOuterTwinNormal_One {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorOuterTwinNormal_Two {
    field0: *mut wire_custom_nested_error_inner_twin_normal,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_struct_error_twin_normal {
    a: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_mixed_twin_normal {
    tag: i32,
    kind: *mut EnumWithItemMixedTwinNormalKind,
}

#[repr(C)]
pub union EnumWithItemMixedTwinNormalKind {
    A: *mut wire_EnumWithItemMixedTwinNormal_A,
    B: *mut wire_EnumWithItemMixedTwinNormal_B,
    C: *mut wire_EnumWithItemMixedTwinNormal_C,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinNormal_A {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinNormal_B {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinNormal_C {
    c_field: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_mixed_twin_sync {
    tag: i32,
    kind: *mut EnumWithItemMixedTwinSyncKind,
}

#[repr(C)]
pub union EnumWithItemMixedTwinSyncKind {
    A: *mut wire_EnumWithItemMixedTwinSync_A,
    B: *mut wire_EnumWithItemMixedTwinSync_B,
    C: *mut wire_EnumWithItemMixedTwinSync_C,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinSync_A {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinSync_B {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinSync_C {
    c_field: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_struct_twin_normal {
    tag: i32,
    kind: *mut EnumWithItemStructTwinNormalKind,
}

#[repr(C)]
pub union EnumWithItemStructTwinNormalKind {
    A: *mut wire_EnumWithItemStructTwinNormal_A,
    B: *mut wire_EnumWithItemStructTwinNormal_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStructTwinNormal_A {
    a_field: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStructTwinNormal_B {
    b_field: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_struct_twin_sync {
    tag: i32,
    kind: *mut EnumWithItemStructTwinSyncKind,
}

#[repr(C)]
pub union EnumWithItemStructTwinSyncKind {
    A: *mut wire_EnumWithItemStructTwinSync_A,
    B: *mut wire_EnumWithItemStructTwinSync_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStructTwinSync_A {
    a_field: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStructTwinSync_B {
    b_field: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_tuple_twin_normal {
    tag: i32,
    kind: *mut EnumWithItemTupleTwinNormalKind,
}

#[repr(C)]
pub union EnumWithItemTupleTwinNormalKind {
    A: *mut wire_EnumWithItemTupleTwinNormal_A,
    B: *mut wire_EnumWithItemTupleTwinNormal_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTupleTwinNormal_A {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTupleTwinNormal_B {
    field0: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_tuple_twin_sync {
    tag: i32,
    kind: *mut EnumWithItemTupleTwinSyncKind,
}

#[repr(C)]
pub union EnumWithItemTupleTwinSyncKind {
    A: *mut wire_EnumWithItemTupleTwinSync_A,
    B: *mut wire_EnumWithItemTupleTwinSync_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTupleTwinSync_A {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTupleTwinSync_B {
    field0: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_list_bool {
    ptr: *mut bool,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_f_32 {
    ptr: *mut f32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_f_64 {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_16 {
    ptr: *mut i16,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_32 {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_64 {
    ptr: *mut i64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_8 {
    ptr: *mut i8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_16 {
    ptr: *mut u16,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_32 {
    ptr: *mut u32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_64 {
    ptr: *mut u64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_8 {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_macro_struct {
    data: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments_twin_normal {
    field_with_comments: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments_twin_sync {
    field_with_comments: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_one_field_twin_normal {
    a: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_one_field_twin_sync {
    a: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_two_field_twin_normal {
    a: i32,
    b: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_two_field_twin_sync {
    a: i32,
    b: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_zero_field_twin_normal {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_zero_field_twin_sync {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_one_field_twin_normal {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_one_field_twin_sync {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_two_field_twin_normal {
    field0: i32,
    field1: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_two_field_twin_sync {
    field0: i32,
    field1: i32,
}

// Section: impl_new_with_nullptr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}
impl NewWithNullPtr for wire_custom_nested_error_inner_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_nested_error_inner_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_nested_error_outer_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_nested_error_outer_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_struct_error_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_struct_error_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_mixed_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_mixed_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_mixed_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_mixed_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_struct_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_struct_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_struct_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_struct_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_tuple_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_tuple_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_tuple_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_tuple_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_macro_struct {
    fn new_with_null_ptr() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
impl Default for wire_macro_struct {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_comments_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field_with_comments: Default::default(),
        }
    }
}
impl Default for wire_struct_with_comments_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_comments_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field_with_comments: Default::default(),
        }
    }
}
impl Default for wire_struct_with_comments_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_one_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
        }
    }
}
impl Default for wire_struct_with_one_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_one_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
        }
    }
}
impl Default for wire_struct_with_one_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_two_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
        }
    }
}
impl Default for wire_struct_with_two_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_two_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
        }
    }
}
impl Default for wire_struct_with_two_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_zero_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {}
    }
}
impl Default for wire_struct_with_zero_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_zero_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {}
    }
}
impl Default for wire_struct_with_zero_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_one_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_one_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_one_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_one_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_two_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_two_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_two_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_two_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
    port_: i64,
    that: *mut wire_struct_with_comments_twin_normal,
) {
    wire_StructWithCommentsTwinNormal_instance_method_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_static_method_twin_normal(port_: i64) {
    wire_StructWithCommentsTwinNormal_static_method_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_slash_star_star_twin_normal(port_: i64) {
    wire_function_with_comments_slash_star_star_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line_twin_normal(port_: i64) {
    wire_function_with_comments_triple_slash_multi_line_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line_twin_normal(port_: i64) {
    wire_function_with_comments_triple_slash_single_line_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_simple_twin_normal(port_: i64, arg: i32) {
    wire_func_enum_simple_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_mixed_twin_normal(
    port_: i64,
    arg: *mut wire_enum_with_item_mixed_twin_normal,
) {
    wire_func_enum_with_item_mixed_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_struct_twin_normal(
    port_: i64,
    arg: *mut wire_enum_with_item_struct_twin_normal,
) {
    wire_func_enum_with_item_struct_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_tuple_twin_normal(
    port_: i64,
    arg: *mut wire_enum_with_item_tuple_twin_normal,
) {
    wire_func_enum_with_item_tuple_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_panic_twin_normal(port_: i64) {
    wire_custom_enum_error_panic_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_error_twin_normal(port_: i64) {
    wire_custom_enum_error_return_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_ok_twin_normal(port_: i64) {
    wire_custom_enum_error_return_ok_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_custom_nested_error_return_error_twin_normal(
    port_: i64,
    arg: *mut wire_custom_nested_error_outer_twin_normal,
) {
    wire_custom_nested_error_return_error_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_struct_error_return_error_twin_normal(
    port_: i64,
    arg: *mut wire_custom_struct_error_twin_normal,
) {
    wire_custom_struct_error_return_error_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_return_error_twin_normal(port_: i64) {
    wire_func_return_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_type_fallible_panic_twin_normal(port_: i64) {
    wire_func_type_fallible_panic_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_type_infallible_panic_twin_normal(port_: i64) {
    wire_func_type_infallible_panic_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_macro_struct(port_: i64, arg: *mut wire_macro_struct) {
    wire_func_macro_struct_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_return_unit_twin_normal(port_: i64) {
    wire_func_return_unit_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_string_twin_normal(port_: i64, arg: *mut wire_list_prim_u_8) {
    wire_func_string_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    that: *mut wire_struct_with_comments_twin_sync,
) -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_instance_method_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_static_method_twin_sync(
) -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_static_method_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_slash_star_star_twin_sync() -> support::WireSyncReturn
{
    wire_function_with_comments_slash_star_star_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line_twin_sync(
) -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_multi_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line_twin_sync(
) -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_single_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_enum_simple_twin_sync(arg: i32) -> support::WireSyncReturn {
    wire_func_enum_simple_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_mixed_twin_sync(
    arg: *mut wire_enum_with_item_mixed_twin_sync,
) -> support::WireSyncReturn {
    wire_func_enum_with_item_mixed_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_struct_twin_sync(
    arg: *mut wire_enum_with_item_struct_twin_sync,
) -> support::WireSyncReturn {
    wire_func_enum_with_item_struct_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_tuple_twin_sync(
    arg: *mut wire_enum_with_item_tuple_twin_sync,
) -> support::WireSyncReturn {
    wire_func_enum_with_item_tuple_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_return_error_twin_sync() -> support::WireSyncReturn {
    wire_func_return_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_return_panic_twin_sync() -> support::WireSyncReturn {
    wire_func_return_panic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_return_unit_twin_sync() -> support::WireSyncReturn {
    wire_func_return_unit_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_string_twin_sync(
    arg: *mut wire_list_prim_u_8,
) -> support::WireSyncReturn {
    wire_func_string_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_bool_twin_normal(
    port_: i64,
    arg: *mut bool,
) {
    wire_example_optional_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32_twin_normal(port_: i64, arg: *mut f32) {
    wire_example_optional_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64_twin_normal(port_: i64, arg: *mut f64) {
    wire_example_optional_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16_twin_normal(port_: i64, arg: *mut i16) {
    wire_example_optional_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32_twin_normal(port_: i64, arg: *mut i32) {
    wire_example_optional_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64_twin_normal(port_: i64, arg: *mut i64) {
    wire_example_optional_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8_twin_normal(port_: i64, arg: *mut i8) {
    wire_example_optional_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16_twin_normal(port_: i64, arg: *mut u16) {
    wire_example_optional_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32_twin_normal(port_: i64, arg: *mut u32) {
    wire_example_optional_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64_twin_normal(port_: i64, arg: *mut u64) {
    wire_example_optional_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8_twin_normal(port_: i64, arg: *mut u8) {
    wire_example_optional_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_bool_twin_sync(
    arg: *mut bool,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32_twin_sync(
    arg: *mut f32,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64_twin_sync(
    arg: *mut f64,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16_twin_sync(
    arg: *mut i16,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32_twin_sync(
    arg: *mut i32,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64_twin_sync(
    arg: *mut i64,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8_twin_sync(
    arg: *mut i8,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16_twin_sync(
    arg: *mut u16,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32_twin_sync(
    arg: *mut u32,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64_twin_sync(
    arg: *mut u64,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8_twin_sync(
    arg: *mut u8,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_bool_twin_normal(port_: i64, arg: bool) {
    wire_example_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32_twin_normal(port_: i64, arg: f32) {
    wire_example_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64_twin_normal(port_: i64, arg: f64) {
    wire_example_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16_twin_normal(port_: i64, arg: i16) {
    wire_example_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32_twin_normal(port_: i64, arg: i32) {
    wire_example_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64_twin_normal(port_: i64, arg: i64) {
    wire_example_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8_twin_normal(port_: i64, arg: i8) {
    wire_example_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16_twin_normal(port_: i64, arg: u16) {
    wire_example_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32_twin_normal(port_: i64, arg: u32) {
    wire_example_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64_twin_normal(port_: i64, arg: u64) {
    wire_example_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8_twin_normal(port_: i64, arg: u8) {
    wire_example_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_bool_twin_normal(
    port_: i64,
    arg: *mut wire_list_bool,
) {
    wire_example_primitive_list_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_f_32,
) {
    wire_example_primitive_list_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_f_64,
) {
    wire_example_primitive_list_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i16_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_16,
) {
    wire_example_primitive_list_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_32,
) {
    wire_example_primitive_list_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_64,
) {
    wire_example_primitive_list_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i8_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_8,
) {
    wire_example_primitive_list_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u16_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_16,
) {
    wire_example_primitive_list_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_32,
) {
    wire_example_primitive_list_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_64,
) {
    wire_example_primitive_list_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u8_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_8,
) {
    wire_example_primitive_list_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_bool_twin_sync(
    arg: *mut wire_list_bool,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f32_twin_sync(
    arg: *mut wire_list_prim_f_32,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f64_twin_sync(
    arg: *mut wire_list_prim_f_64,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i16_twin_sync(
    arg: *mut wire_list_prim_i_16,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i32_twin_sync(
    arg: *mut wire_list_prim_i_32,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i64_twin_sync(
    arg: *mut wire_list_prim_i_64,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i8_twin_sync(
    arg: *mut wire_list_prim_i_8,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u16_twin_sync(
    arg: *mut wire_list_prim_u_16,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u32_twin_sync(
    arg: *mut wire_list_prim_u_32,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u64_twin_sync(
    arg: *mut wire_list_prim_u_64,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u8_twin_sync(
    arg: *mut wire_list_prim_u_8,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_bool_twin_sync(arg: bool) -> support::WireSyncReturn {
    wire_example_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32_twin_sync(arg: f32) -> support::WireSyncReturn {
    wire_example_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64_twin_sync(arg: f64) -> support::WireSyncReturn {
    wire_example_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16_twin_sync(arg: i16) -> support::WireSyncReturn {
    wire_example_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32_twin_sync(arg: i32) -> support::WireSyncReturn {
    wire_example_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64_twin_sync(arg: i64) -> support::WireSyncReturn {
    wire_example_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8_twin_sync(arg: i8) -> support::WireSyncReturn {
    wire_example_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16_twin_sync(arg: u16) -> support::WireSyncReturn {
    wire_example_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32_twin_sync(arg: u32) -> support::WireSyncReturn {
    wire_example_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64_twin_sync(arg: u64) -> support::WireSyncReturn {
    wire_example_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8_twin_sync(arg: u8) -> support::WireSyncReturn {
    wire_example_primitive_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_twin_sync(a: i32, b: i32) -> support::WireSyncReturn {
    wire_simple_adder_twin_sync_impl(a, b)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_one_field_twin_sync(
    arg: *mut wire_struct_with_one_field_twin_sync,
) -> support::WireSyncReturn {
    wire_func_struct_with_one_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_two_field_twin_sync(
    arg: *mut wire_struct_with_two_field_twin_sync,
) -> support::WireSyncReturn {
    wire_func_struct_with_two_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_zero_field_twin_sync(
    arg: *mut wire_struct_with_zero_field_twin_sync,
) -> support::WireSyncReturn {
    wire_func_struct_with_zero_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_one_field_twin_sync(
    arg: *mut wire_tuple_struct_with_one_field_twin_sync,
) -> support::WireSyncReturn {
    wire_func_tuple_struct_with_one_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_two_field_twin_sync(
    arg: *mut wire_tuple_struct_with_two_field_twin_sync,
) -> support::WireSyncReturn {
    wire_func_tuple_struct_with_two_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_twin_normal(port_: i64, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_func_stream_realistic_twin_normal(port_: i64, arg: *mut wire_list_prim_u_8) {
    wire_func_stream_realistic_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_stream_return_error_twin_normal(port_: i64) {
    wire_func_stream_return_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_stream_return_panic_twin_normal(port_: i64) {
    wire_func_stream_return_panic_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_stream_sink_arg_position_twin_normal(port_: i64, a: u32, b: u32) {
    wire_func_stream_sink_arg_position_twin_normal_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_one_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_one_field_twin_normal,
) {
    wire_func_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_two_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_two_field_twin_normal,
) {
    wire_func_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_zero_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_zero_field_twin_normal,
) {
    wire_func_struct_with_zero_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_one_field_twin_normal(
    port_: i64,
    arg: *mut wire_tuple_struct_with_one_field_twin_normal,
) {
    wire_func_tuple_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_two_field_twin_normal(
    port_: i64,
    arg: *mut wire_tuple_struct_with_two_field_twin_normal,
) {
    wire_func_tuple_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_inner_twin_normal(
) -> *mut wire_custom_nested_error_inner_twin_normal {
    support::new_leak_box_ptr(wire_custom_nested_error_inner_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_outer_twin_normal(
) -> *mut wire_custom_nested_error_outer_twin_normal {
    support::new_leak_box_ptr(wire_custom_nested_error_outer_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_struct_error_twin_normal(
) -> *mut wire_custom_struct_error_twin_normal {
    support::new_leak_box_ptr(wire_custom_struct_error_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_mixed_twin_normal(
) -> *mut wire_enum_with_item_mixed_twin_normal {
    support::new_leak_box_ptr(wire_enum_with_item_mixed_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_mixed_twin_sync(
) -> *mut wire_enum_with_item_mixed_twin_sync {
    support::new_leak_box_ptr(wire_enum_with_item_mixed_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_struct_twin_normal(
) -> *mut wire_enum_with_item_struct_twin_normal {
    support::new_leak_box_ptr(wire_enum_with_item_struct_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_struct_twin_sync(
) -> *mut wire_enum_with_item_struct_twin_sync {
    support::new_leak_box_ptr(wire_enum_with_item_struct_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_tuple_twin_normal(
) -> *mut wire_enum_with_item_tuple_twin_normal {
    support::new_leak_box_ptr(wire_enum_with_item_tuple_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_tuple_twin_sync(
) -> *mut wire_enum_with_item_tuple_twin_sync {
    support::new_leak_box_ptr(wire_enum_with_item_tuple_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_32(value: f32) -> *mut f32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_64(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_16(value: i16) -> *mut i16 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_32(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_64(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_8(value: i8) -> *mut i8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_macro_struct() -> *mut wire_macro_struct {
    support::new_leak_box_ptr(wire_macro_struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_normal(
) -> *mut wire_struct_with_comments_twin_normal {
    support::new_leak_box_ptr(wire_struct_with_comments_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_sync(
) -> *mut wire_struct_with_comments_twin_sync {
    support::new_leak_box_ptr(wire_struct_with_comments_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_one_field_twin_normal(
) -> *mut wire_struct_with_one_field_twin_normal {
    support::new_leak_box_ptr(wire_struct_with_one_field_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_one_field_twin_sync(
) -> *mut wire_struct_with_one_field_twin_sync {
    support::new_leak_box_ptr(wire_struct_with_one_field_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_two_field_twin_normal(
) -> *mut wire_struct_with_two_field_twin_normal {
    support::new_leak_box_ptr(wire_struct_with_two_field_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_two_field_twin_sync(
) -> *mut wire_struct_with_two_field_twin_sync {
    support::new_leak_box_ptr(wire_struct_with_two_field_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_zero_field_twin_normal(
) -> *mut wire_struct_with_zero_field_twin_normal {
    support::new_leak_box_ptr(wire_struct_with_zero_field_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_zero_field_twin_sync(
) -> *mut wire_struct_with_zero_field_twin_sync {
    support::new_leak_box_ptr(wire_struct_with_zero_field_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_one_field_twin_normal(
) -> *mut wire_tuple_struct_with_one_field_twin_normal {
    support::new_leak_box_ptr(wire_tuple_struct_with_one_field_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_one_field_twin_sync(
) -> *mut wire_tuple_struct_with_one_field_twin_sync {
    support::new_leak_box_ptr(wire_tuple_struct_with_one_field_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_two_field_twin_normal(
) -> *mut wire_tuple_struct_with_two_field_twin_normal {
    support::new_leak_box_ptr(wire_tuple_struct_with_two_field_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_two_field_twin_sync(
) -> *mut wire_tuple_struct_with_two_field_twin_sync {
    support::new_leak_box_ptr(wire_tuple_struct_with_two_field_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_16(value: u16) -> *mut u16 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_32(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_64(value: u64) -> *mut u64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_8(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_list_bool(len: i32) -> *mut wire_list_bool {
    let wrap = wire_list_bool {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_32(len: i32) -> *mut wire_list_prim_f_32 {
    let ans = wire_list_prim_f_32 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_64(len: i32) -> *mut wire_list_prim_f_64 {
    let ans = wire_list_prim_f_64 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_16(len: i32) -> *mut wire_list_prim_i_16 {
    let ans = wire_list_prim_i_16 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_32(len: i32) -> *mut wire_list_prim_i_32 {
    let ans = wire_list_prim_i_32 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_64(len: i32) -> *mut wire_list_prim_i_64 {
    let ans = wire_list_prim_i_64 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_8(len: i32) -> *mut wire_list_prim_i_8 {
    let ans = wire_list_prim_i_8 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_16(len: i32) -> *mut wire_list_prim_u_16 {
    let ans = wire_list_prim_u_16 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_32(len: i32) -> *mut wire_list_prim_u_32 {
    let ans = wire_list_prim_u_32 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_64(len: i32) -> *mut wire_list_prim_u_64 {
    let ans = wire_list_prim_u_64 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_8(len: i32) -> *mut wire_list_prim_u_8 {
    let ans = wire_list_prim_u_8 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinNormal_Three(
) -> *mut CustomNestedErrorInnerTwinNormalKind {
    support::new_leak_box_ptr(CustomNestedErrorInnerTwinNormalKind {
        Three: support::new_leak_box_ptr(wire_CustomNestedErrorInnerTwinNormal_Three {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinNormal_Four(
) -> *mut CustomNestedErrorInnerTwinNormalKind {
    support::new_leak_box_ptr(CustomNestedErrorInnerTwinNormalKind {
        Four: support::new_leak_box_ptr(wire_CustomNestedErrorInnerTwinNormal_Four {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinNormal_One(
) -> *mut CustomNestedErrorOuterTwinNormalKind {
    support::new_leak_box_ptr(CustomNestedErrorOuterTwinNormalKind {
        One: support::new_leak_box_ptr(wire_CustomNestedErrorOuterTwinNormal_One {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinNormal_Two(
) -> *mut CustomNestedErrorOuterTwinNormalKind {
    support::new_leak_box_ptr(CustomNestedErrorOuterTwinNormalKind {
        Two: support::new_leak_box_ptr(wire_CustomNestedErrorOuterTwinNormal_Two {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinNormal_B() -> *mut EnumWithItemMixedTwinNormalKind {
    support::new_leak_box_ptr(EnumWithItemMixedTwinNormalKind {
        B: support::new_leak_box_ptr(wire_EnumWithItemMixedTwinNormal_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinNormal_C() -> *mut EnumWithItemMixedTwinNormalKind {
    support::new_leak_box_ptr(EnumWithItemMixedTwinNormalKind {
        C: support::new_leak_box_ptr(wire_EnumWithItemMixedTwinNormal_C {
            c_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinSync_B() -> *mut EnumWithItemMixedTwinSyncKind {
    support::new_leak_box_ptr(EnumWithItemMixedTwinSyncKind {
        B: support::new_leak_box_ptr(wire_EnumWithItemMixedTwinSync_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinSync_C() -> *mut EnumWithItemMixedTwinSyncKind {
    support::new_leak_box_ptr(EnumWithItemMixedTwinSyncKind {
        C: support::new_leak_box_ptr(wire_EnumWithItemMixedTwinSync_C {
            c_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinNormal_A() -> *mut EnumWithItemStructTwinNormalKind
{
    support::new_leak_box_ptr(EnumWithItemStructTwinNormalKind {
        A: support::new_leak_box_ptr(wire_EnumWithItemStructTwinNormal_A {
            a_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinNormal_B() -> *mut EnumWithItemStructTwinNormalKind
{
    support::new_leak_box_ptr(EnumWithItemStructTwinNormalKind {
        B: support::new_leak_box_ptr(wire_EnumWithItemStructTwinNormal_B {
            b_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinSync_A() -> *mut EnumWithItemStructTwinSyncKind {
    support::new_leak_box_ptr(EnumWithItemStructTwinSyncKind {
        A: support::new_leak_box_ptr(wire_EnumWithItemStructTwinSync_A {
            a_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinSync_B() -> *mut EnumWithItemStructTwinSyncKind {
    support::new_leak_box_ptr(EnumWithItemStructTwinSyncKind {
        B: support::new_leak_box_ptr(wire_EnumWithItemStructTwinSync_B {
            b_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinNormal_A() -> *mut EnumWithItemTupleTwinNormalKind {
    support::new_leak_box_ptr(EnumWithItemTupleTwinNormalKind {
        A: support::new_leak_box_ptr(wire_EnumWithItemTupleTwinNormal_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinNormal_B() -> *mut EnumWithItemTupleTwinNormalKind {
    support::new_leak_box_ptr(EnumWithItemTupleTwinNormalKind {
        B: support::new_leak_box_ptr(wire_EnumWithItemTupleTwinNormal_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinSync_A() -> *mut EnumWithItemTupleTwinSyncKind {
    support::new_leak_box_ptr(EnumWithItemTupleTwinSyncKind {
        A: support::new_leak_box_ptr(wire_EnumWithItemTupleTwinSync_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinSync_B() -> *mut EnumWithItemTupleTwinSyncKind {
    support::new_leak_box_ptr(EnumWithItemTupleTwinSyncKind {
        B: support::new_leak_box_ptr(wire_EnumWithItemTupleTwinSync_B {
            field0: core::ptr::null_mut(),
        }),
    })
}
