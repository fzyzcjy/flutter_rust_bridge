use super::*;

// Section: impl_wire2api

impl<T> Wire2Api<Option<T>> for JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<Vec<String>> for JsValue {
    fn wire2api(self) -> Vec<String> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<A> for JsValue {
    fn wire2api(self) -> A {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        A {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<Abc> for JsValue {
    fn wire2api(self) -> Abc {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Abc::A(self_.get(1).wire2api()),
            1 => Abc::B(self_.get(1).wire2api()),
            2 => Abc::C(self_.get(1).wire2api()),
            3 => Abc::JustInt(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<B> for JsValue {
    fn wire2api(self) -> B {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        B {
            b: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<Blob> for JsValue {
    fn wire2api(self) -> Blob {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        Blob(self_.get(0).wire2api())
    }
}
impl Wire2Api<Box<[u8; 1600]>> for Box<[u8]> {
    fn wire2api(self) -> Box<[u8; 1600]> {
        Wire2Api::<[u8; 1600]>::wire2api(self).into()
    }
}
impl Wire2Api<C> for JsValue {
    fn wire2api(self) -> C {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        C {
            c: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<CustomNestedErrorInnerTwinNormal> for JsValue {
    fn wire2api(self) -> CustomNestedErrorInnerTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => CustomNestedErrorInnerTwinNormal::Three(self_.get(1).wire2api()),
            1 => CustomNestedErrorInnerTwinNormal::Four(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<CustomNestedErrorInnerTwinSync> for JsValue {
    fn wire2api(self) -> CustomNestedErrorInnerTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => CustomNestedErrorInnerTwinSync::Three(self_.get(1).wire2api()),
            1 => CustomNestedErrorInnerTwinSync::Four(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<CustomNestedErrorOuterTwinNormal> for JsValue {
    fn wire2api(self) -> CustomNestedErrorOuterTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => CustomNestedErrorOuterTwinNormal::One(self_.get(1).wire2api()),
            1 => CustomNestedErrorOuterTwinNormal::Two(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<CustomNestedErrorOuterTwinSync> for JsValue {
    fn wire2api(self) -> CustomNestedErrorOuterTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => CustomNestedErrorOuterTwinSync::One(self_.get(1).wire2api()),
            1 => CustomNestedErrorOuterTwinSync::Two(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<CustomStructErrorTwinNormal> for JsValue {
    fn wire2api(self) -> CustomStructErrorTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        CustomStructErrorTwinNormal {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<CustomStructErrorTwinSync> for JsValue {
    fn wire2api(self) -> CustomStructErrorTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        CustomStructErrorTwinSync {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<Distance> for JsValue {
    fn wire2api(self) -> Distance {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Distance::Unknown,
            1 => Distance::Map(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemMixedTwinNormal> for JsValue {
    fn wire2api(self) -> EnumWithItemMixedTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumWithItemMixedTwinNormal::A,
            1 => EnumWithItemMixedTwinNormal::B(self_.get(1).wire2api()),
            2 => EnumWithItemMixedTwinNormal::C {
                c_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemMixedTwinSync> for JsValue {
    fn wire2api(self) -> EnumWithItemMixedTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumWithItemMixedTwinSync::A,
            1 => EnumWithItemMixedTwinSync::B(self_.get(1).wire2api()),
            2 => EnumWithItemMixedTwinSync::C {
                c_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemStructTwinNormal> for JsValue {
    fn wire2api(self) -> EnumWithItemStructTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumWithItemStructTwinNormal::A {
                a_field: self_.get(1).wire2api(),
            },
            1 => EnumWithItemStructTwinNormal::B {
                b_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemStructTwinSync> for JsValue {
    fn wire2api(self) -> EnumWithItemStructTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumWithItemStructTwinSync::A {
                a_field: self_.get(1).wire2api(),
            },
            1 => EnumWithItemStructTwinSync::B {
                b_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemTupleTwinNormal> for JsValue {
    fn wire2api(self) -> EnumWithItemTupleTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumWithItemTupleTwinNormal::A(self_.get(1).wire2api()),
            1 => EnumWithItemTupleTwinNormal::B(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<EnumWithItemTupleTwinSync> for JsValue {
    fn wire2api(self) -> EnumWithItemTupleTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumWithItemTupleTwinSync::A(self_.get(1).wire2api()),
            1 => EnumWithItemTupleTwinSync::B(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<[f64; 16]> for Box<[f64]> {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<FeedId> for JsValue {
    fn wire2api(self) -> FeedId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        FeedId(self_.get(0).wire2api())
    }
}
impl Wire2Api<[i32; 2]> for Box<[i32]> {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Vec<bool>> for JsValue {
    fn wire2api(self) -> Vec<bool> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<MySize>> for JsValue {
    fn wire2api(self) -> Vec<MySize> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<MyTreeNode>> for JsValue {
    fn wire2api(self) -> Vec<MyTreeNode> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<f32>> for Box<[f32]> {
    fn wire2api(self) -> Vec<f32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Vec<f64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i16>> for Box<[i16]> {
    fn wire2api(self) -> Vec<i16> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i32>> for Box<[i32]> {
    fn wire2api(self) -> Vec<i32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i64>> for Box<[i64]> {
    fn wire2api(self) -> Vec<i64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i8>> for Box<[i8]> {
    fn wire2api(self) -> Vec<i8> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u16>> for Box<[u16]> {
    fn wire2api(self) -> Vec<u16> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u32>> for Box<[u32]> {
    fn wire2api(self) -> Vec<u32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u64>> for Box<[u64]> {
    fn wire2api(self) -> Vec<u64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<TestId>> for JsValue {
    fn wire2api(self) -> Vec<TestId> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Weekdays>> for JsValue {
    fn wire2api(self) -> Vec<Weekdays> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<MacroStruct> for JsValue {
    fn wire2api(self) -> MacroStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        MacroStruct {
            data: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<Measure> for JsValue {
    fn wire2api(self) -> Measure {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Measure::Speed(self_.get(1).wire2api()),
            1 => Measure::Distance(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<MessageId> for JsValue {
    fn wire2api(self) -> MessageId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        MessageId(self_.get(0).wire2api())
    }
}
impl Wire2Api<MyNestedStruct> for JsValue {
    fn wire2api(self) -> MyNestedStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        MyNestedStruct {
            tree_node: self_.get(0).wire2api(),
            weekday: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<MySize> for JsValue {
    fn wire2api(self) -> MySize {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        MySize {
            width: self_.get(0).wire2api(),
            height: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<MyTreeNode> for JsValue {
    fn wire2api(self) -> MyTreeNode {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        MyTreeNode {
            value_i32: self_.get(0).wire2api(),
            value_vec_u8: self_.get(1).wire2api(),
            value_boolean: self_.get(2).wire2api(),
            children: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<NewTypeInt> for JsValue {
    fn wire2api(self) -> NewTypeInt {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        NewTypeInt(self_.get(0).wire2api())
    }
}
impl Wire2Api<Note> for JsValue {
    fn wire2api(self) -> Note {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        Note {
            day: self_.get(0).wire2api(),
            body: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<Speed> for JsValue {
    fn wire2api(self) -> Speed {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Speed::Unknown,
            1 => Speed::GPS(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<StructWithCommentsTwinNormal> for JsValue {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructWithCommentsTwinNormal {
            field_with_comments: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<StructWithCommentsTwinSync> for JsValue {
    fn wire2api(self) -> StructWithCommentsTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructWithCommentsTwinSync {
            field_with_comments: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<StructWithEnum> for JsValue {
    fn wire2api(self) -> StructWithEnum {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        StructWithEnum {
            abc1: self_.get(0).wire2api(),
            abc2: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<StructWithOneFieldTwinNormal> for JsValue {
    fn wire2api(self) -> StructWithOneFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructWithOneFieldTwinNormal {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<StructWithOneFieldTwinSync> for JsValue {
    fn wire2api(self) -> StructWithOneFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructWithOneFieldTwinSync {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<StructWithTwoFieldTwinNormal> for JsValue {
    fn wire2api(self) -> StructWithTwoFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        StructWithTwoFieldTwinNormal {
            a: self_.get(0).wire2api(),
            b: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<StructWithTwoFieldTwinSync> for JsValue {
    fn wire2api(self) -> StructWithTwoFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        StructWithTwoFieldTwinSync {
            a: self_.get(0).wire2api(),
            b: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<StructWithZeroFieldTwinNormal> for JsValue {
    fn wire2api(self) -> StructWithZeroFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            0,
            "Expected 0 elements, got {}",
            self_.length()
        );
        StructWithZeroFieldTwinNormal {}
    }
}
impl Wire2Api<StructWithZeroFieldTwinSync> for JsValue {
    fn wire2api(self) -> StructWithZeroFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            0,
            "Expected 0 elements, got {}",
            self_.length()
        );
        StructWithZeroFieldTwinSync {}
    }
}
impl Wire2Api<TestId> for JsValue {
    fn wire2api(self) -> TestId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        TestId(self_.get(0).wire2api())
    }
}
impl Wire2Api<TupleStructWithOneFieldTwinNormal> for JsValue {
    fn wire2api(self) -> TupleStructWithOneFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        TupleStructWithOneFieldTwinNormal(self_.get(0).wire2api())
    }
}
impl Wire2Api<TupleStructWithOneFieldTwinSync> for JsValue {
    fn wire2api(self) -> TupleStructWithOneFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        TupleStructWithOneFieldTwinSync(self_.get(0).wire2api())
    }
}
impl Wire2Api<TupleStructWithTwoFieldTwinNormal> for JsValue {
    fn wire2api(self) -> TupleStructWithTwoFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        TupleStructWithTwoFieldTwinNormal(self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}
impl Wire2Api<TupleStructWithTwoFieldTwinSync> for JsValue {
    fn wire2api(self) -> TupleStructWithTwoFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        TupleStructWithTwoFieldTwinSync(self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}
impl Wire2Api<[u8; 1600]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<Box<Blob>> for JsValue {
    fn wire2api(self) -> Box<Blob> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<Distance>> for JsValue {
    fn wire2api(self) -> Box<Distance> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<Speed>> for JsValue {
    fn wire2api(self) -> Box<Speed> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<[u8; 1600]>> for JsValue {
    fn wire2api(self) -> Box<[u8; 1600]> {
        let vec: Vec<u8> = self.wire2api();
        Box::new(support::from_vec_to_array(vec))
    }
}
impl Wire2Api<Box<Weekdays>> for JsValue {
    fn wire2api(self) -> Box<Weekdays> {
        let ptr: Box<i32> = self.wire2api();
        Box::new(ptr.wire2api())
    }
}
impl Wire2Api<EnumSimpleTwinNormal> for JsValue {
    fn wire2api(self) -> EnumSimpleTwinNormal {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<EnumSimpleTwinSync> for JsValue {
    fn wire2api(self) -> EnumSimpleTwinSync {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<f32> for JsValue {
    fn wire2api(self) -> f32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<[f64; 16]> for JsValue {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<i16> for JsValue {
    fn wire2api(self) -> i16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<[i32; 2]> for JsValue {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<f32>> for JsValue {
    fn wire2api(self) -> Vec<f32> {
        self.unchecked_into::<js_sys::Float32Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<f64>> for JsValue {
    fn wire2api(self) -> Vec<f64> {
        self.unchecked_into::<js_sys::Float64Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<i16>> for JsValue {
    fn wire2api(self) -> Vec<i16> {
        self.unchecked_into::<js_sys::Int16Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<i32>> for JsValue {
    fn wire2api(self) -> Vec<i32> {
        self.unchecked_into::<js_sys::Int32Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<i64>> for JsValue {
    fn wire2api(self) -> Vec<i64> {
        let buf = self.dyn_into::<js_sys::BigInt64Array>().unwrap();
        let buf = js_sys::Uint8Array::new(&buf.buffer());
        support::slice_from_byte_buffer(buf.to_vec()).into()
    }
}
impl Wire2Api<Vec<i8>> for JsValue {
    fn wire2api(self) -> Vec<i8> {
        self.unchecked_into::<js_sys::Int8Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<u16>> for JsValue {
    fn wire2api(self) -> Vec<u16> {
        self.unchecked_into::<js_sys::Uint16Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<u32>> for JsValue {
    fn wire2api(self) -> Vec<u32> {
        self.unchecked_into::<js_sys::Uint32Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<u64>> for JsValue {
    fn wire2api(self) -> Vec<u64> {
        let buf = self.dyn_into::<js_sys::BigUint64Array>().unwrap();
        let buf = js_sys::Uint8Array::new(&buf.buffer());
        support::slice_from_byte_buffer(buf.to_vec()).into()
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
impl Wire2Api<[TestId; 4]> for JsValue {
    fn wire2api(self) -> [TestId; 4] {
        let vec: Vec<TestId> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<u16> for JsValue {
    fn wire2api(self) -> u16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u64> for JsValue {
    fn wire2api(self) -> u64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<[u8; 1600]> for JsValue {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for JsValue {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for JsValue {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Weekdays> for JsValue {
    fn wire2api(self) -> Weekdays {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}

#[wasm_bindgen]
pub fn wire_boxed_blob(port_: MessagePort, blob: Box<[u8]>) {
    wire_boxed_blob_impl(port_, blob)
}

#[wasm_bindgen]
pub fn wire_get_array(port_: MessagePort) {
    wire_get_array_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_complex_array(port_: MessagePort) {
    wire_get_complex_array_impl(port_)
}

#[wasm_bindgen]
pub fn wire_last_number(port_: MessagePort, array: Box<[f64]>) {
    wire_last_number_impl(port_, array)
}

#[wasm_bindgen]
pub fn wire_nested_id(port_: MessagePort, id: JsValue) {
    wire_nested_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_new_msgid(port_: MessagePort, id: Box<[u8]>) {
    wire_new_msgid_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_return_boxed_feed_id(port_: MessagePort, id: Box<[u8]>) {
    wire_return_boxed_feed_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_return_boxed_raw_feed_id(port_: MessagePort, id: JsValue) {
    wire_return_boxed_raw_feed_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_test_id(port_: MessagePort, id: JsValue) {
    wire_test_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_use_boxed_blob(port_: MessagePort, blob: JsValue) {
    wire_use_boxed_blob_impl(port_, blob)
}

#[wasm_bindgen]
pub fn wire_use_msgid(port_: MessagePort, id: JsValue) {
    wire_use_msgid_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
    port_: MessagePort,
    that: JsValue,
) {
    wire_StructWithCommentsTwinNormal_instance_method_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinNormal_static_method_twin_normal(port_: MessagePort) {
    wire_StructWithCommentsTwinNormal_static_method_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star_twin_normal(port_: MessagePort) {
    wire_function_with_comments_slash_star_star_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line_twin_normal(port_: MessagePort) {
    wire_function_with_comments_triple_slash_multi_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line_twin_normal(port_: MessagePort) {
    wire_function_with_comments_triple_slash_single_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_enum_simple_twin_normal(port_: MessagePort, arg: i32) {
    wire_func_enum_simple_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_mixed_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_func_enum_with_item_mixed_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_struct_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_func_enum_with_item_struct_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_tuple_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_func_enum_with_item_tuple_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_enum_parameter(port_: MessagePort, weekday: i32) {
    wire_handle_enum_parameter_impl(port_, weekday)
}

#[wasm_bindgen]
pub fn wire_handle_return_enum(port_: MessagePort, input: String) {
    wire_handle_return_enum_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_multiply_by_ten(port_: MessagePort, measure: JsValue) {
    wire_multiply_by_ten_impl(port_, measure)
}

#[wasm_bindgen]
pub fn wire_print_note(port_: MessagePort, note: JsValue) {
    wire_print_note_impl(port_, note)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_panic_twin_normal(port_: MessagePort) {
    wire_custom_enum_error_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_error_twin_normal(port_: MessagePort) {
    wire_custom_enum_error_return_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_ok_twin_normal(port_: MessagePort, arg: u32) {
    wire_custom_enum_error_return_ok_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_custom_nested_error_return_error_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_custom_nested_error_return_error_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_custom_struct_error_return_error_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_custom_struct_error_return_error_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_return_error_twin_normal(port_: MessagePort) {
    wire_func_return_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_type_fallible_panic_twin_normal(port_: MessagePort) {
    wire_func_type_fallible_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_type_infallible_panic_twin_normal(port_: MessagePort) {
    wire_func_type_infallible_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_macro_struct(port_: MessagePort, arg: JsValue) {
    wire_func_macro_struct_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_big_buffers(port_: MessagePort) {
    wire_handle_big_buffers_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_complex_struct(port_: MessagePort, s: JsValue) {
    wire_handle_complex_struct_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_nested_struct(port_: MessagePort, s: JsValue) {
    wire_handle_nested_struct_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_list_of_primitive_enums(port_: MessagePort, weekdays: JsValue) {
    wire_list_of_primitive_enums_impl(port_, weekdays)
}

#[wasm_bindgen]
pub fn wire_test_abc_enum(port_: MessagePort, abc: JsValue) {
    wire_test_abc_enum_impl(port_, abc)
}

#[wasm_bindgen]
pub fn wire_test_struct_with_enum(port_: MessagePort, se: JsValue) {
    wire_test_struct_with_enum_impl(port_, se)
}

#[wasm_bindgen]
pub fn wire_func_return_unit_twin_normal(port_: MessagePort) {
    wire_func_return_unit_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_string_twin_normal(port_: MessagePort, arg: String) {
    wire_func_string_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_list_of_struct(port_: MessagePort, l: JsValue) {
    wire_handle_list_of_struct_impl(port_, l)
}

#[wasm_bindgen]
pub fn wire_handle_string_list(port_: MessagePort, names: JsValue) {
    wire_handle_string_list_impl(port_, names)
}

#[wasm_bindgen]
pub fn wire_handle_newtype(port_: MessagePort, arg: JsValue) {
    wire_handle_newtype_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    that: JsValue,
) -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_instance_method_twin_sync_impl(that)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinSync_static_method_twin_sync() -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_static_method_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star_twin_sync() -> support::WireSyncReturn {
    wire_function_with_comments_slash_star_star_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line_twin_sync() -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_multi_line_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line_twin_sync() -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_single_line_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_enum_simple_twin_sync(arg: i32) -> support::WireSyncReturn {
    wire_func_enum_simple_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_mixed_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_func_enum_with_item_mixed_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_struct_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_func_enum_with_item_struct_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_tuple_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_func_enum_with_item_tuple_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_panic_twin_sync() -> support::WireSyncReturn {
    wire_custom_enum_error_panic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_error_twin_sync() -> support::WireSyncReturn {
    wire_custom_enum_error_return_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_ok_twin_sync(arg: u32) -> support::WireSyncReturn {
    wire_custom_enum_error_return_ok_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_custom_nested_error_return_error_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_custom_nested_error_return_error_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_custom_struct_error_return_error_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_custom_struct_error_return_error_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_return_error_twin_sync() -> support::WireSyncReturn {
    wire_func_return_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_type_fallible_panic_twin_sync() -> support::WireSyncReturn {
    wire_func_type_fallible_panic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_type_infallible_panic_twin_sync() -> support::WireSyncReturn {
    wire_func_type_infallible_panic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_return_unit_twin_sync() -> support::WireSyncReturn {
    wire_func_return_unit_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_string_twin_sync(arg: String) -> support::WireSyncReturn {
    wire_func_string_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_bool_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f32_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f64_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i16_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i32_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i64_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i8_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u16_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u32_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u64_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u8_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_bool_twin_sync(
    arg: JsValue,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f32_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f64_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i16_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i32_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i64_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i8_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u16_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u32_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u64_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u8_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_bool_twin_normal(port_: MessagePort, arg: bool) {
    wire_example_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f32_twin_normal(port_: MessagePort, arg: f32) {
    wire_example_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f64_twin_normal(port_: MessagePort, arg: f64) {
    wire_example_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i16_twin_normal(port_: MessagePort, arg: i16) {
    wire_example_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i32_twin_normal(port_: MessagePort, arg: i32) {
    wire_example_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i64_twin_normal(port_: MessagePort, arg: i64) {
    wire_example_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i8_twin_normal(port_: MessagePort, arg: i8) {
    wire_example_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u16_twin_normal(port_: MessagePort, arg: u16) {
    wire_example_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u32_twin_normal(port_: MessagePort, arg: u32) {
    wire_example_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u64_twin_normal(port_: MessagePort, arg: u64) {
    wire_example_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u8_twin_normal(port_: MessagePort, arg: u8) {
    wire_example_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_bool_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_primitive_list_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f32_twin_normal(port_: MessagePort, arg: Box<[f32]>) {
    wire_example_primitive_list_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f64_twin_normal(port_: MessagePort, arg: Box<[f64]>) {
    wire_example_primitive_list_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i16_twin_normal(port_: MessagePort, arg: Box<[i16]>) {
    wire_example_primitive_list_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i32_twin_normal(port_: MessagePort, arg: Box<[i32]>) {
    wire_example_primitive_list_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i64_twin_normal(port_: MessagePort, arg: Box<[i64]>) {
    wire_example_primitive_list_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i8_twin_normal(port_: MessagePort, arg: Box<[i8]>) {
    wire_example_primitive_list_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u16_twin_normal(port_: MessagePort, arg: Box<[u16]>) {
    wire_example_primitive_list_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u32_twin_normal(port_: MessagePort, arg: Box<[u32]>) {
    wire_example_primitive_list_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u64_twin_normal(port_: MessagePort, arg: Box<[u64]>) {
    wire_example_primitive_list_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u8_twin_normal(port_: MessagePort, arg: Box<[u8]>) {
    wire_example_primitive_list_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_bool_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_primitive_list_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f32_twin_sync(arg: Box<[f32]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f64_twin_sync(arg: Box<[f64]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i16_twin_sync(arg: Box<[i16]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i32_twin_sync(arg: Box<[i32]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i64_twin_sync(arg: Box<[i64]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i8_twin_sync(arg: Box<[i8]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u16_twin_sync(arg: Box<[u16]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u32_twin_sync(arg: Box<[u32]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u64_twin_sync(arg: Box<[u64]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u8_twin_sync(arg: Box<[u8]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_bool_twin_sync(arg: bool) -> support::WireSyncReturn {
    wire_example_primitive_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f32_twin_sync(arg: f32) -> support::WireSyncReturn {
    wire_example_primitive_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f64_twin_sync(arg: f64) -> support::WireSyncReturn {
    wire_example_primitive_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i16_twin_sync(arg: i16) -> support::WireSyncReturn {
    wire_example_primitive_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i32_twin_sync(arg: i32) -> support::WireSyncReturn {
    wire_example_primitive_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i64_twin_sync(arg: i64) -> support::WireSyncReturn {
    wire_example_primitive_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i8_twin_sync(arg: i8) -> support::WireSyncReturn {
    wire_example_primitive_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u16_twin_sync(arg: u16) -> support::WireSyncReturn {
    wire_example_primitive_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u32_twin_sync(arg: u32) -> support::WireSyncReturn {
    wire_example_primitive_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u64_twin_sync(arg: u64) -> support::WireSyncReturn {
    wire_example_primitive_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u8_twin_sync(arg: u8) -> support::WireSyncReturn {
    wire_example_primitive_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_simple_adder_twin_sync(a: i32, b: i32) -> support::WireSyncReturn {
    wire_simple_adder_twin_sync_impl(a, b)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_one_field_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_func_struct_with_one_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_two_field_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_func_struct_with_two_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_zero_field_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_func_struct_with_zero_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_one_field_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_func_tuple_struct_with_one_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_two_field_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_func_tuple_struct_with_two_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_test_more_than_just_one_raw_string_struct(port_: MessagePort) {
    wire_test_more_than_just_one_raw_string_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_item_struct(port_: MessagePort) {
    wire_test_raw_string_item_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_simple_adder_twin_normal(port_: MessagePort, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_func_stream_realistic_twin_normal(port_: MessagePort, arg: String) {
    wire_func_stream_realistic_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_stream_return_error_twin_normal(port_: MessagePort) {
    wire_func_stream_return_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_stream_return_panic_twin_normal(port_: MessagePort) {
    wire_func_stream_return_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_stream_sink_arg_position_twin_normal(port_: MessagePort, a: u32, b: u32) {
    wire_func_stream_sink_arg_position_twin_normal_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_handle_stream_of_struct(port_: MessagePort) {
    wire_handle_stream_of_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_one_field_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_func_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_two_field_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_func_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_zero_field_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_func_struct_with_zero_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_one_field_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_func_tuple_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_two_field_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_func_tuple_struct_with_two_field_twin_normal_impl(port_, arg)
}
