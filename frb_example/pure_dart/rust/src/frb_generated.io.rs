use super::*;

// Section: impl_wire2api

impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::microseconds(self)
    }
}
impl Wire2Api<Vec<chrono::Duration>> for *mut wire_list_prim_i_64 {
    fn wire2api(self) -> Vec<chrono::Duration> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<chrono::NaiveDateTime>> for *mut wire_list_prim_i_64 {
    fn wire2api(self) -> Vec<chrono::NaiveDateTime> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<String> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<uuid::Uuid> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> uuid::Uuid {
        let single: Vec<u8> = self.wire2api();
        wire2api_uuid_ref(single.as_slice())
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        wire2api_uuids(multiple)
    }
}
impl Wire2Api<A> for wire_a {
    fn wire2api(self) -> A {
        A {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<Abc> for wire_abc {
    fn wire2api(self) -> Abc {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.A);
                Abc::A(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.B);
                Abc::B(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.C);
                Abc::C(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.JustInt);
                Abc::JustInt(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<B> for wire_b {
    fn wire2api(self) -> B {
        B {
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<Blob> for wire_blob {
    fn wire2api(self) -> Blob {
        Blob(self.field0.wire2api())
    }
}
impl Wire2Api<chrono::DateTime<chrono::Utc>> for *mut i64 {
    fn wire2api(self) -> chrono::DateTime<chrono::Utc> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<chrono::DateTime<chrono::Utc>>::wire2api(*wrap).into()
    }
}
impl Wire2Api<A> for *mut wire_a {
    fn wire2api(self) -> A {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<A>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Abc> for *mut wire_abc {
    fn wire2api(self) -> Abc {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Abc>::wire2api(*wrap).into()
    }
}
impl Wire2Api<B> for *mut wire_b {
    fn wire2api(self) -> B {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<B>::wire2api(*wrap).into()
    }
}
impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<C> for *mut wire_c {
    fn wire2api(self) -> C {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<C>::wire2api(*wrap).into()
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
impl Wire2Api<CustomNestedErrorInnerTwinSync> for *mut wire_custom_nested_error_inner_twin_sync {
    fn wire2api(self) -> CustomNestedErrorInnerTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CustomNestedErrorInnerTwinSync>::wire2api(*wrap).into()
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
impl Wire2Api<CustomNestedErrorOuterTwinSync> for *mut wire_custom_nested_error_outer_twin_sync {
    fn wire2api(self) -> CustomNestedErrorOuterTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CustomNestedErrorOuterTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CustomStructErrorTwinNormal> for *mut wire_custom_struct_error_twin_normal {
    fn wire2api(self) -> CustomStructErrorTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CustomStructErrorTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CustomStructErrorTwinSync> for *mut wire_custom_struct_error_twin_sync {
    fn wire2api(self) -> CustomStructErrorTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CustomStructErrorTwinSync>::wire2api(*wrap).into()
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
impl Wire2Api<FeatureChrono> for *mut wire_feature_chrono {
    fn wire2api(self) -> FeatureChrono {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<FeatureChrono>::wire2api(*wrap).into()
    }
}
impl Wire2Api<FeatureUuid> for *mut wire_feature_uuid {
    fn wire2api(self) -> FeatureUuid {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<FeatureUuid>::wire2api(*wrap).into()
    }
}
impl Wire2Api<FeedId> for *mut wire_feed_id {
    fn wire2api(self) -> FeedId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<FeedId>::wire2api(*wrap).into()
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
impl Wire2Api<Measure> for *mut wire_measure {
    fn wire2api(self) -> Measure {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Measure>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MessageId> for *mut wire_message_id {
    fn wire2api(self) -> MessageId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MessageId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MyNestedStruct> for *mut wire_my_nested_struct {
    fn wire2api(self) -> MyNestedStruct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MyNestedStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<MyTreeNode> for *mut wire_my_tree_node {
    fn wire2api(self) -> MyTreeNode {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<MyTreeNode>::wire2api(*wrap).into()
    }
}
impl Wire2Api<NewTypeInt> for *mut wire_new_type_int {
    fn wire2api(self) -> NewTypeInt {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<NewTypeInt>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Note> for *mut wire_note {
    fn wire2api(self) -> Note {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Note>::wire2api(*wrap).into()
    }
}
impl Wire2Api<(String, i32)> for *mut wire_record_string_i_32 {
    fn wire2api(self) -> (String, i32) {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<(String, i32)>::wire2api(*wrap).into()
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
impl Wire2Api<StructWithEnum> for *mut wire_struct_with_enum {
    fn wire2api(self) -> StructWithEnum {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithEnum>::wire2api(*wrap).into()
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
impl Wire2Api<TestId> for *mut wire_test_id {
    fn wire2api(self) -> TestId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TestId>::wire2api(*wrap).into()
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
impl Wire2Api<Box<Blob>> for *mut wire_blob {
    fn wire2api(self) -> Box<Blob> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Blob>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<Distance>> for *mut wire_distance {
    fn wire2api(self) -> Box<Distance> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Distance>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<Speed>> for *mut wire_speed {
    fn wire2api(self) -> Box<Speed> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Speed>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<[u8; 1600]>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Box<[u8; 1600]> {
        Wire2Api::<[u8; 1600]>::wire2api(self).into()
    }
}
impl Wire2Api<Box<Weekdays>> for *mut i32 {
    fn wire2api(self) -> Box<Weekdays> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Weekdays>::wire2api(*wrap).into()
    }
}
impl Wire2Api<C> for wire_c {
    fn wire2api(self) -> C {
        C {
            c: self.c.wire2api(),
        }
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
impl Wire2Api<CustomNestedErrorInnerTwinSync> for wire_custom_nested_error_inner_twin_sync {
    fn wire2api(self) -> CustomNestedErrorInnerTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Three);
                CustomNestedErrorInnerTwinSync::Three(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Four);
                CustomNestedErrorInnerTwinSync::Four(ans.field0.wire2api())
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
impl Wire2Api<CustomNestedErrorOuterTwinSync> for wire_custom_nested_error_outer_twin_sync {
    fn wire2api(self) -> CustomNestedErrorOuterTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.One);
                CustomNestedErrorOuterTwinSync::One(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Two);
                CustomNestedErrorOuterTwinSync::Two(ans.field0.wire2api())
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
impl Wire2Api<CustomStructErrorTwinSync> for wire_custom_struct_error_twin_sync {
    fn wire2api(self) -> CustomStructErrorTwinSync {
        CustomStructErrorTwinSync {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<Distance> for wire_distance {
    fn wire2api(self) -> Distance {
        match self.tag {
            0 => Distance::Unknown,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Map);
                Distance::Map(ans.field0.wire2api())
            },
            _ => unreachable!(),
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
impl Wire2Api<[f64; 16]> for *mut wire_list_prim_f_64 {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<FeatureChrono> for wire_feature_chrono {
    fn wire2api(self) -> FeatureChrono {
        FeatureChrono {
            utc: self.utc.wire2api(),
            local: self.local.wire2api(),
            duration: self.duration.wire2api(),
            naive: self.naive.wire2api(),
        }
    }
}
impl Wire2Api<FeatureUuid> for wire_feature_uuid {
    fn wire2api(self) -> FeatureUuid {
        FeatureUuid {
            one: self.one.wire2api(),
            many: self.many.wire2api(),
        }
    }
}
impl Wire2Api<FeedId> for wire_feed_id {
    fn wire2api(self) -> FeedId {
        FeedId(self.field0.wire2api())
    }
}
impl Wire2Api<[i32; 2]> for *mut wire_list_prim_i_32 {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        support::from_vec_to_array(vec)
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
impl Wire2Api<Vec<MySize>> for *mut wire_list_my_size {
    fn wire2api(self) -> Vec<MySize> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<MyTreeNode>> for *mut wire_list_my_tree_node {
    fn wire2api(self) -> Vec<MyTreeNode> {
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
impl Wire2Api<Vec<(String, i32)>> for *mut wire_list_record_string_i_32 {
    fn wire2api(self) -> Vec<(String, i32)> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<TestId>> for *mut wire_list_test_id {
    fn wire2api(self) -> Vec<TestId> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Weekdays>> for *mut wire_list_weekdays {
    fn wire2api(self) -> Vec<Weekdays> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<MacroStruct> for wire_macro_struct {
    fn wire2api(self) -> MacroStruct {
        MacroStruct {
            data: self.data.wire2api(),
        }
    }
}
impl Wire2Api<Measure> for wire_measure {
    fn wire2api(self) -> Measure {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Speed);
                Measure::Speed(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Distance);
                Measure::Distance(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<MessageId> for wire_message_id {
    fn wire2api(self) -> MessageId {
        MessageId(self.field0.wire2api())
    }
}
impl Wire2Api<MyNestedStruct> for wire_my_nested_struct {
    fn wire2api(self) -> MyNestedStruct {
        MyNestedStruct {
            tree_node: self.tree_node.wire2api(),
            weekday: self.weekday.wire2api(),
        }
    }
}
impl Wire2Api<MySize> for wire_my_size {
    fn wire2api(self) -> MySize {
        MySize {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}
impl Wire2Api<MyTreeNode> for wire_my_tree_node {
    fn wire2api(self) -> MyTreeNode {
        MyTreeNode {
            value_i32: self.value_i32.wire2api(),
            value_vec_u8: self.value_vec_u8.wire2api(),
            value_boolean: self.value_boolean.wire2api(),
            children: self.children.wire2api(),
        }
    }
}
impl Wire2Api<NewTypeInt> for wire_new_type_int {
    fn wire2api(self) -> NewTypeInt {
        NewTypeInt(self.field0.wire2api())
    }
}
impl Wire2Api<Note> for wire_note {
    fn wire2api(self) -> Note {
        Note {
            day: self.day.wire2api(),
            body: self.body.wire2api(),
        }
    }
}
impl Wire2Api<(String, i32)> for wire_record_string_i_32 {
    fn wire2api(self) -> (String, i32) {
        (self.field0.wire2api(), self.field1.wire2api())
    }
}
impl Wire2Api<Speed> for wire_speed {
    fn wire2api(self) -> Speed {
        match self.tag {
            0 => Speed::Unknown,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.GPS);
                Speed::GPS(ans.field0.wire2api())
            },
            _ => unreachable!(),
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
impl Wire2Api<StructWithEnum> for wire_struct_with_enum {
    fn wire2api(self) -> StructWithEnum {
        StructWithEnum {
            abc1: self.abc1.wire2api(),
            abc2: self.abc2.wire2api(),
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
impl Wire2Api<TestId> for wire_test_id {
    fn wire2api(self) -> TestId {
        TestId(self.field0.wire2api())
    }
}
impl Wire2Api<[TestId; 4]> for *mut wire_list_test_id {
    fn wire2api(self) -> [TestId; 4] {
        let vec: Vec<TestId> = self.wire2api();
        support::from_vec_to_array(vec)
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
impl Wire2Api<[u8; 1600]> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}

// Section: wire2api_class

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_list_prim_u_8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_a {
    a: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_abc {
    tag: i32,
    kind: *mut AbcKind,
}

#[repr(C)]
pub union AbcKind {
    A: *mut wire_Abc_A,
    B: *mut wire_Abc_B,
    C: *mut wire_Abc_C,
    JustInt: *mut wire_Abc_JustInt,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Abc_A {
    field0: *mut wire_a,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Abc_B {
    field0: *mut wire_b,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Abc_C {
    field0: *mut wire_c,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Abc_JustInt {
    field0: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_b {
    b: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_blob {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_c {
    c: bool,
}
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
pub struct wire_custom_nested_error_inner_twin_sync {
    tag: i32,
    kind: *mut CustomNestedErrorInnerTwinSyncKind,
}

#[repr(C)]
pub union CustomNestedErrorInnerTwinSyncKind {
    Three: *mut wire_CustomNestedErrorInnerTwinSync_Three,
    Four: *mut wire_CustomNestedErrorInnerTwinSync_Four,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorInnerTwinSync_Three {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorInnerTwinSync_Four {
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
pub struct wire_custom_nested_error_outer_twin_sync {
    tag: i32,
    kind: *mut CustomNestedErrorOuterTwinSyncKind,
}

#[repr(C)]
pub union CustomNestedErrorOuterTwinSyncKind {
    One: *mut wire_CustomNestedErrorOuterTwinSync_One,
    Two: *mut wire_CustomNestedErrorOuterTwinSync_Two,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorOuterTwinSync_One {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorOuterTwinSync_Two {
    field0: *mut wire_custom_nested_error_inner_twin_sync,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_struct_error_twin_normal {
    a: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_struct_error_twin_sync {
    a: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_distance {
    tag: i32,
    kind: *mut DistanceKind,
}

#[repr(C)]
pub union DistanceKind {
    Unknown: *mut wire_Distance_Unknown,
    Map: *mut wire_Distance_Map,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Distance_Unknown {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Distance_Map {
    field0: f64,
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
pub struct wire_feature_chrono {
    utc: i64,
    local: i64,
    duration: i64,
    naive: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_feature_uuid {
    one: *mut wire_list_prim_u_8,
    many: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_feed_id {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_bool {
    ptr: *mut bool,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_size {
    ptr: *mut wire_my_size,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_tree_node {
    ptr: *mut wire_my_tree_node,
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
pub struct wire_list_record_string_i_32 {
    ptr: *mut wire_record_string_i_32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_test_id {
    ptr: *mut wire_test_id,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_weekdays {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_macro_struct {
    data: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_measure {
    tag: i32,
    kind: *mut MeasureKind,
}

#[repr(C)]
pub union MeasureKind {
    Speed: *mut wire_Measure_Speed,
    Distance: *mut wire_Measure_Distance,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Measure_Speed {
    field0: *mut wire_speed,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Measure_Distance {
    field0: *mut wire_distance,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_message_id {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_nested_struct {
    tree_node: wire_my_tree_node,
    weekday: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_size {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_tree_node {
    value_i32: i32,
    value_vec_u8: *mut wire_list_prim_u_8,
    value_boolean: bool,
    children: *mut wire_list_my_tree_node,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_new_type_int {
    field0: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_note {
    day: *mut i32,
    body: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_record_string_i_32 {
    field0: *mut wire_list_prim_u_8,
    field1: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_speed {
    tag: i32,
    kind: *mut SpeedKind,
}

#[repr(C)]
pub union SpeedKind {
    Unknown: *mut wire_Speed_Unknown,
    GPS: *mut wire_Speed_GPS,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Speed_Unknown {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Speed_GPS {
    field0: f64,
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
pub struct wire_struct_with_enum {
    abc1: wire_abc,
    abc2: wire_abc,
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
pub struct wire_test_id {
    field0: *mut wire_list_prim_i_32,
}

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
impl NewWithNullPtr for wire_a {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_a {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_abc {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_abc {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_b {
    fn new_with_null_ptr() -> Self {
        Self {
            b: Default::default(),
        }
    }
}
impl Default for wire_b {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_blob {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_blob {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_c {
    fn new_with_null_ptr() -> Self {
        Self {
            c: Default::default(),
        }
    }
}
impl Default for wire_c {
    fn default() -> Self {
        Self::new_with_null_ptr()
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
impl NewWithNullPtr for wire_custom_nested_error_inner_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_nested_error_inner_twin_sync {
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
impl NewWithNullPtr for wire_custom_nested_error_outer_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_nested_error_outer_twin_sync {
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
impl NewWithNullPtr for wire_custom_struct_error_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_struct_error_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_distance {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_distance {
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
impl NewWithNullPtr for wire_feature_chrono {
    fn new_with_null_ptr() -> Self {
        Self {
            utc: Default::default(),
            local: Default::default(),
            duration: Default::default(),
            naive: Default::default(),
        }
    }
}
impl Default for wire_feature_chrono {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_feature_uuid {
    fn new_with_null_ptr() -> Self {
        Self {
            one: core::ptr::null_mut(),
            many: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_feature_uuid {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_feed_id {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_feed_id {
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
impl NewWithNullPtr for wire_measure {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_measure {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_message_id {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_message_id {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_nested_struct {
    fn new_with_null_ptr() -> Self {
        Self {
            tree_node: Default::default(),
            weekday: Default::default(),
        }
    }
}
impl Default for wire_my_nested_struct {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_size {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}
impl Default for wire_my_size {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_tree_node {
    fn new_with_null_ptr() -> Self {
        Self {
            value_i32: Default::default(),
            value_vec_u8: core::ptr::null_mut(),
            value_boolean: Default::default(),
            children: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_my_tree_node {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_new_type_int {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_new_type_int {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_note {
    fn new_with_null_ptr() -> Self {
        Self {
            day: core::ptr::null_mut(),
            body: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_note {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_record_string_i_32 {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }
    }
}
impl Default for wire_record_string_i_32 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_speed {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_speed {
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
impl NewWithNullPtr for wire_struct_with_enum {
    fn new_with_null_ptr() -> Self {
        Self {
            abc1: Default::default(),
            abc2: Default::default(),
        }
    }
}
impl Default for wire_struct_with_enum {
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
impl NewWithNullPtr for wire_test_id {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_test_id {
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
pub extern "C" fn wire_boxed_blob(port_: i64, blob: *mut wire_list_prim_u_8) {
    wire_boxed_blob_impl(port_, blob)
}

#[no_mangle]
pub extern "C" fn wire_func_test_id(port_: i64, id: *mut wire_test_id) {
    wire_func_test_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_get_array(port_: i64) {
    wire_get_array_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_complex_array(port_: i64) {
    wire_get_complex_array_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_last_number(port_: i64, array: *mut wire_list_prim_f_64) {
    wire_last_number_impl(port_, array)
}

#[no_mangle]
pub extern "C" fn wire_nested_id(port_: i64, id: *mut wire_list_test_id) {
    wire_nested_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_new_msgid(port_: i64, id: *mut wire_list_prim_u_8) {
    wire_new_msgid_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_return_boxed_feed_id(port_: i64, id: *mut wire_list_prim_u_8) {
    wire_return_boxed_feed_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_return_boxed_raw_feed_id(port_: i64, id: *mut wire_feed_id) {
    wire_return_boxed_raw_feed_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_use_boxed_blob(port_: i64, blob: *mut wire_blob) {
    wire_use_boxed_blob_impl(port_, blob)
}

#[no_mangle]
pub extern "C" fn wire_use_msgid(port_: i64, id: *mut wire_message_id) {
    wire_use_msgid_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_datetime_local(port_: i64, d: i64) {
    wire_datetime_local_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_datetime_utc(port_: i64, d: i64) {
    wire_datetime_utc_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_duration(port_: i64, d: i64) {
    wire_duration_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_handle_durations(
    port_: i64,
    durations: *mut wire_list_prim_i_64,
    since: i64,
) {
    wire_handle_durations_impl(port_, durations, since)
}

#[no_mangle]
pub extern "C" fn wire_handle_timestamps(
    port_: i64,
    timestamps: *mut wire_list_prim_i_64,
    epoch: i64,
) {
    wire_handle_timestamps_impl(port_, timestamps, epoch)
}

#[no_mangle]
pub extern "C" fn wire_how_long_does_it_take(port_: i64, mine: *mut wire_feature_chrono) {
    wire_how_long_does_it_take_impl(port_, mine)
}

#[no_mangle]
pub extern "C" fn wire_naivedatetime(port_: i64, d: i64) {
    wire_naivedatetime_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_optional_empty_datetime_utc(port_: i64, d: *mut i64) {
    wire_optional_empty_datetime_utc_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_test_chrono(port_: i64) {
    wire_test_chrono_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_precise_chrono(port_: i64) {
    wire_test_precise_chrono_impl(port_)
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
pub extern "C" fn wire_return_dart_dynamic(port_: i64) {
    wire_return_dart_dynamic_impl(port_)
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
pub extern "C" fn wire_handle_enum_parameter(port_: i64, weekday: i32) {
    wire_handle_enum_parameter_impl(port_, weekday)
}

#[no_mangle]
pub extern "C" fn wire_handle_return_enum(port_: i64, input: *mut wire_list_prim_u_8) {
    wire_handle_return_enum_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_multiply_by_ten(port_: i64, measure: *mut wire_measure) {
    wire_multiply_by_ten_impl(port_, measure)
}

#[no_mangle]
pub extern "C" fn wire_print_note(port_: i64, note: *mut wire_note) {
    wire_print_note_impl(port_, note)
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
pub extern "C" fn wire_custom_enum_error_return_ok_twin_normal(port_: i64, arg: u32) {
    wire_custom_enum_error_return_ok_twin_normal_impl(port_, arg)
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
pub extern "C" fn wire_handle_big_buffers(port_: i64) {
    wire_handle_big_buffers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_complex_struct(port_: i64, s: *mut wire_my_tree_node) {
    wire_handle_complex_struct_impl(port_, s)
}

#[no_mangle]
pub extern "C" fn wire_handle_nested_struct(port_: i64, s: *mut wire_my_nested_struct) {
    wire_handle_nested_struct_impl(port_, s)
}

#[no_mangle]
pub extern "C" fn wire_list_of_primitive_enums(port_: i64, weekdays: *mut wire_list_weekdays) {
    wire_list_of_primitive_enums_impl(port_, weekdays)
}

#[no_mangle]
pub extern "C" fn wire_test_abc_enum(port_: i64, abc: *mut wire_abc) {
    wire_test_abc_enum_impl(port_, abc)
}

#[no_mangle]
pub extern "C" fn wire_test_struct_with_enum(port_: i64, se: *mut wire_struct_with_enum) {
    wire_test_struct_with_enum_impl(port_, se)
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
pub extern "C" fn wire_handle_list_of_struct(port_: i64, l: *mut wire_list_my_size) {
    wire_handle_list_of_struct_impl(port_, l)
}

#[no_mangle]
pub extern "C" fn wire_handle_string_list(port_: i64, names: *mut wire_StringList) {
    wire_handle_string_list_impl(port_, names)
}

#[no_mangle]
pub extern "C" fn wire_handle_newtype(port_: i64, arg: *mut wire_new_type_int) {
    wire_handle_newtype_impl(port_, arg)
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
pub extern "C" fn wire_custom_enum_error_panic_twin_sync() -> support::WireSyncReturn {
    wire_custom_enum_error_panic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_error_twin_sync() -> support::WireSyncReturn {
    wire_custom_enum_error_return_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_ok_twin_sync(arg: u32) -> support::WireSyncReturn {
    wire_custom_enum_error_return_ok_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_nested_error_return_error_twin_sync(
    arg: *mut wire_custom_nested_error_outer_twin_sync,
) -> support::WireSyncReturn {
    wire_custom_nested_error_return_error_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_struct_error_return_error_twin_sync(
    arg: *mut wire_custom_struct_error_twin_sync,
) -> support::WireSyncReturn {
    wire_custom_struct_error_return_error_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_return_error_twin_sync() -> support::WireSyncReturn {
    wire_func_return_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_type_fallible_panic_twin_sync() -> support::WireSyncReturn {
    wire_func_type_fallible_panic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_type_infallible_panic_twin_sync() -> support::WireSyncReturn {
    wire_func_type_infallible_panic_twin_sync_impl()
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
pub extern "C" fn wire_test_more_than_just_one_raw_string_struct(port_: i64) {
    wire_test_more_than_just_one_raw_string_struct_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_raw_string_item_struct(port_: i64) {
    wire_test_raw_string_item_struct_impl(port_)
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
pub extern "C" fn wire_handle_stream_of_struct(port_: i64) {
    wire_handle_stream_of_struct_impl(port_)
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
pub extern "C" fn wire_test_tuple(port_: i64, value: *mut wire_record_string_i_32) {
    wire_test_tuple_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_test_tuple_2(port_: i64, value: *mut wire_list_record_string_i_32) {
    wire_test_tuple_2_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_handle_nested_uuids(port_: i64, ids: *mut wire_feature_uuid) {
    wire_handle_nested_uuids_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_handle_uuid(port_: i64, id: *mut wire_list_prim_u_8) {
    wire_handle_uuid_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_handle_uuids(port_: i64, ids: *mut wire_list_prim_u_8) {
    wire_handle_uuids_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn new_StringList(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_list_prim_u_8>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_Chrono_Utc(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_a() -> *mut wire_a {
    support::new_leak_box_ptr(wire_a::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_abc() -> *mut wire_abc {
    support::new_leak_box_ptr(wire_abc::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_b() -> *mut wire_b {
    support::new_leak_box_ptr(wire_b::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_c() -> *mut wire_c {
    support::new_leak_box_ptr(wire_c::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_inner_twin_normal(
) -> *mut wire_custom_nested_error_inner_twin_normal {
    support::new_leak_box_ptr(wire_custom_nested_error_inner_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_inner_twin_sync(
) -> *mut wire_custom_nested_error_inner_twin_sync {
    support::new_leak_box_ptr(wire_custom_nested_error_inner_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_outer_twin_normal(
) -> *mut wire_custom_nested_error_outer_twin_normal {
    support::new_leak_box_ptr(wire_custom_nested_error_outer_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_outer_twin_sync(
) -> *mut wire_custom_nested_error_outer_twin_sync {
    support::new_leak_box_ptr(wire_custom_nested_error_outer_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_struct_error_twin_normal(
) -> *mut wire_custom_struct_error_twin_normal {
    support::new_leak_box_ptr(wire_custom_struct_error_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_struct_error_twin_sync(
) -> *mut wire_custom_struct_error_twin_sync {
    support::new_leak_box_ptr(wire_custom_struct_error_twin_sync::new_with_null_ptr())
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
pub extern "C" fn new_box_autoadd_feature_chrono() -> *mut wire_feature_chrono {
    support::new_leak_box_ptr(wire_feature_chrono::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feature_uuid() -> *mut wire_feature_uuid {
    support::new_leak_box_ptr(wire_feature_uuid::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feed_id() -> *mut wire_feed_id {
    support::new_leak_box_ptr(wire_feed_id::new_with_null_ptr())
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
pub extern "C" fn new_box_autoadd_measure() -> *mut wire_measure {
    support::new_leak_box_ptr(wire_measure::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_message_id() -> *mut wire_message_id {
    support::new_leak_box_ptr(wire_message_id::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_nested_struct() -> *mut wire_my_nested_struct {
    support::new_leak_box_ptr(wire_my_nested_struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_tree_node() -> *mut wire_my_tree_node {
    support::new_leak_box_ptr(wire_my_tree_node::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_new_type_int() -> *mut wire_new_type_int {
    support::new_leak_box_ptr(wire_new_type_int::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_note() -> *mut wire_note {
    support::new_leak_box_ptr(wire_note::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_record_string_i_32() -> *mut wire_record_string_i_32 {
    support::new_leak_box_ptr(wire_record_string_i_32::new_with_null_ptr())
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
pub extern "C" fn new_box_autoadd_struct_with_enum() -> *mut wire_struct_with_enum {
    support::new_leak_box_ptr(wire_struct_with_enum::new_with_null_ptr())
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
pub extern "C" fn new_box_autoadd_test_id() -> *mut wire_test_id {
    support::new_leak_box_ptr(wire_test_id::new_with_null_ptr())
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
pub extern "C" fn new_box_blob() -> *mut wire_blob {
    support::new_leak_box_ptr(wire_blob::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_distance() -> *mut wire_distance {
    support::new_leak_box_ptr(wire_distance::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_speed() -> *mut wire_speed {
    support::new_leak_box_ptr(wire_speed::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_weekdays(value: i32) -> *mut i32 {
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
pub extern "C" fn new_list_my_size(len: i32) -> *mut wire_list_my_size {
    let wrap = wire_list_my_size {
        ptr: support::new_leak_vec_ptr(<wire_my_size>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_tree_node(len: i32) -> *mut wire_list_my_tree_node {
    let wrap = wire_list_my_tree_node {
        ptr: support::new_leak_vec_ptr(<wire_my_tree_node>::new_with_null_ptr(), len),
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
pub extern "C" fn new_list_record_string_i_32(len: i32) -> *mut wire_list_record_string_i_32 {
    let wrap = wire_list_record_string_i_32 {
        ptr: support::new_leak_vec_ptr(<wire_record_string_i_32>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_test_id(len: i32) -> *mut wire_list_test_id {
    let wrap = wire_list_test_id {
        ptr: support::new_leak_vec_ptr(<wire_test_id>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_weekdays(len: i32) -> *mut wire_list_weekdays {
    let wrap = wire_list_weekdays {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn inflate_Abc_A() -> *mut AbcKind {
    support::new_leak_box_ptr(AbcKind {
        A: support::new_leak_box_ptr(wire_Abc_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Abc_B() -> *mut AbcKind {
    support::new_leak_box_ptr(AbcKind {
        B: support::new_leak_box_ptr(wire_Abc_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Abc_C() -> *mut AbcKind {
    support::new_leak_box_ptr(AbcKind {
        C: support::new_leak_box_ptr(wire_Abc_C {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Abc_JustInt() -> *mut AbcKind {
    support::new_leak_box_ptr(AbcKind {
        JustInt: support::new_leak_box_ptr(wire_Abc_JustInt {
            field0: Default::default(),
        }),
    })
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
pub extern "C" fn inflate_CustomNestedErrorInnerTwinSync_Three(
) -> *mut CustomNestedErrorInnerTwinSyncKind {
    support::new_leak_box_ptr(CustomNestedErrorInnerTwinSyncKind {
        Three: support::new_leak_box_ptr(wire_CustomNestedErrorInnerTwinSync_Three {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinSync_Four(
) -> *mut CustomNestedErrorInnerTwinSyncKind {
    support::new_leak_box_ptr(CustomNestedErrorInnerTwinSyncKind {
        Four: support::new_leak_box_ptr(wire_CustomNestedErrorInnerTwinSync_Four {
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
pub extern "C" fn inflate_CustomNestedErrorOuterTwinSync_One(
) -> *mut CustomNestedErrorOuterTwinSyncKind {
    support::new_leak_box_ptr(CustomNestedErrorOuterTwinSyncKind {
        One: support::new_leak_box_ptr(wire_CustomNestedErrorOuterTwinSync_One {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinSync_Two(
) -> *mut CustomNestedErrorOuterTwinSyncKind {
    support::new_leak_box_ptr(CustomNestedErrorOuterTwinSyncKind {
        Two: support::new_leak_box_ptr(wire_CustomNestedErrorOuterTwinSync_Two {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Distance_Map() -> *mut DistanceKind {
    support::new_leak_box_ptr(DistanceKind {
        Map: support::new_leak_box_ptr(wire_Distance_Map {
            field0: Default::default(),
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

#[no_mangle]
pub extern "C" fn inflate_Measure_Speed() -> *mut MeasureKind {
    support::new_leak_box_ptr(MeasureKind {
        Speed: support::new_leak_box_ptr(wire_Measure_Speed {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Measure_Distance() -> *mut MeasureKind {
    support::new_leak_box_ptr(MeasureKind {
        Distance: support::new_leak_box_ptr(wire_Measure_Distance {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Speed_GPS() -> *mut SpeedKind {
    support::new_leak_box_ptr(SpeedKind {
        GPS: support::new_leak_box_ptr(wire_Speed_GPS {
            field0: Default::default(),
        }),
    })
}
