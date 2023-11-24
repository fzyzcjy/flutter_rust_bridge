use super::*;

// Section: imports

use crate::api::rust_opaque::*;
use crate::api::rust_opaque_sync::*;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::Handler;

// Section: impl_wire2api

impl<T> Wire2Api<Option<T>> for JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::milliseconds(self)
    }
}
impl Wire2Api<Vec<chrono::Duration>> for Box<[i64]> {
    fn wire2api(self) -> Vec<chrono::Duration> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<chrono::NaiveDateTime>> for Box<[i64]> {
    fn wire2api(self) -> Vec<chrono::NaiveDateTime> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<flutter_rust_bridge::DartOpaque> for JsValue {
    fn wire2api(self) -> flutter_rust_bridge::DartOpaque {
        let arr = self.dyn_into::<JsArray>().unwrap();
        unsafe { flutter_rust_bridge::DartOpaque::new(arr.get(0), arr.get(1)) }
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
impl Wire2Api<uuid::Uuid> for Box<[u8]> {
    fn wire2api(self) -> uuid::Uuid {
        let single: Vec<u8> = self.wire2api();
        flutter_rust_bridge::wire2api_uuid_ref(single.as_slice())
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for Box<[u8]> {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        flutter_rust_bridge::wire2api_uuids(multiple)
    }
}
impl Wire2Api<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>> for Box<[u8]> {
    fn wire2api(self) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
        flutter_rust_bridge::ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<crate::api::misc_example::A> for JsValue {
    fn wire2api(self) -> crate::api::misc_example::A {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::A {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::Abc> for JsValue {
    fn wire2api(self) -> crate::api::misc_example::Abc {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::misc_example::Abc::A(self_.get(1).wire2api()),
            1 => crate::api::misc_example::Abc::B(self_.get(1).wire2api()),
            2 => crate::api::misc_example::Abc::C(self_.get(1).wire2api()),
            3 => crate::api::misc_example::Abc::JustInt(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::mirror::ApplicationEnv> for JsValue {
    fn wire2api(self) -> crate::api::mirror::ApplicationEnv {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::mirror::ApplicationEnv {
            vars: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::mirror::ApplicationEnvVar> for JsValue {
    fn wire2api(self) -> crate::api::mirror::ApplicationEnvVar {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::mirror::ApplicationEnvVar(self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}
impl Wire2Api<crate::api::mirror::ApplicationSettings> for JsValue {
    fn wire2api(self) -> crate::api::mirror::ApplicationSettings {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            5,
            "Expected 5 elements, got {}",
            self_.length()
        );
        crate::api::mirror::ApplicationSettings {
            name: self_.get(0).wire2api(),
            version: self_.get(1).wire2api(),
            mode: self_.get(2).wire2api(),
            env: self_.get(3).wire2api(),
            env_optional: self_.get(4).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::Attribute> for JsValue {
    fn wire2api(self) -> crate::api::optional::Attribute {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::optional::Attribute {
            key: self_.get(0).wire2api(),
            value: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::B> for JsValue {
    fn wire2api(self) -> crate::api::misc_example::B {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::B {
            b: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::Blob> for JsValue {
    fn wire2api(self) -> crate::api::array::Blob {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::array::Blob(self_.get(0).wire2api())
    }
}
impl Wire2Api<Box<[u8; 1600]>> for Box<[u8]> {
    fn wire2api(self) -> Box<[u8; 1600]> {
        Wire2Api::<[u8; 1600]>::wire2api(self).into()
    }
}
impl Wire2Api<crate::api::misc_example::C> for JsValue {
    fn wire2api(self) -> crate::api::misc_example::C {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::C {
            c: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::method::ConcatenateWith> for JsValue {
    fn wire2api(self) -> crate::api::method::ConcatenateWith {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::method::ConcatenateWith {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorInnerTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorInnerTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::exception::CustomNestedErrorInnerTwinNormal::Three(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::exception::CustomNestedErrorInnerTwinNormal::Four(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
                    0 => { crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync::Three( self_.get(1).wire2api()) },
1 => { crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync::Four( self_.get(1).wire2api()) },
                    _ => unreachable!(),
                }
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorOuterTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorOuterTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::exception::CustomNestedErrorOuterTwinNormal::One(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::exception::CustomNestedErrorOuterTwinNormal::Two(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => {
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync::One(
                    self_.get(1).wire2api(),
                )
            }
            1 => {
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync::Two(
                    self_.get(1).wire2api(),
                )
            }
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomStructErrorTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::exception::CustomStructErrorTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::exception::CustomStructErrorTwinNormal {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync>
    for JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::attribute::Customized> for JsValue {
    fn wire2api(self) -> crate::api::attribute::Customized {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::attribute::Customized {
            final_field: self_.get(0).wire2api(),
            non_final_field: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::dart_opaque::DartOpaqueNested> for JsValue {
    fn wire2api(self) -> crate::api::dart_opaque::DartOpaqueNested {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::dart_opaque::DartOpaqueNested {
            first: self_.get(0).wire2api(),
            second: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::Distance> for JsValue {
    fn wire2api(self) -> crate::api::enumeration::Distance {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::Distance::Unknown,
            1 => crate::api::enumeration::Distance::Map(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::dart_opaque::EnumDartOpaque> for JsValue {
    fn wire2api(self) -> crate::api::dart_opaque::EnumDartOpaque {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::dart_opaque::EnumDartOpaque::Primitive(self_.get(1).wire2api()),
            1 => crate::api::dart_opaque::EnumDartOpaque::Opaque(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::rust_opaque::EnumOpaque> for JsValue {
    fn wire2api(self) -> crate::api::rust_opaque::EnumOpaque {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::rust_opaque::EnumOpaque::Struct(self_.get(1).wire2api()),
            1 => crate::api::rust_opaque::EnumOpaque::Primitive(self_.get(1).wire2api()),
            2 => crate::api::rust_opaque::EnumOpaque::TraitObj(self_.get(1).wire2api()),
            3 => crate::api::rust_opaque::EnumOpaque::Mutex(self_.get(1).wire2api()),
            4 => crate::api::rust_opaque::EnumOpaque::RwLock(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemMixedTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemMixedTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::EnumWithItemMixedTwinNormal::A,
            1 => crate::api::enumeration::EnumWithItemMixedTwinNormal::B(self_.get(1).wire2api()),
            2 => crate::api::enumeration::EnumWithItemMixedTwinNormal::C {
                c_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::A,
            1 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::B(
                self_.get(1).wire2api(),
            ),
            2 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::C {
                c_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemStructTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemStructTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::EnumWithItemStructTwinNormal::A {
                a_field: self_.get(1).wire2api(),
            },
            1 => crate::api::enumeration::EnumWithItemStructTwinNormal::B {
                b_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync::A {
                a_field: self_.get(1).wire2api(),
            },
            1 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync::B {
                b_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemTupleTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemTupleTwinNormal {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::EnumWithItemTupleTwinNormal::A(self_.get(1).wire2api()),
            1 => crate::api::enumeration::EnumWithItemTupleTwinNormal::B(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync::A(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync::B(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::event_listener::Event> for JsValue {
    fn wire2api(self) -> crate::api::event_listener::Event {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::event_listener::Event {
            address: self_.get(0).wire2api(),
            payload: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::ExoticOptionals> for JsValue {
    fn wire2api(self) -> crate::api::optional::ExoticOptionals {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            14,
            "Expected 14 elements, got {}",
            self_.length()
        );
        crate::api::optional::ExoticOptionals {
            int32: self_.get(0).wire2api(),
            int64: self_.get(1).wire2api(),
            float64: self_.get(2).wire2api(),
            boolean: self_.get(3).wire2api(),
            zerocopy: self_.get(4).wire2api(),
            int8list: self_.get(5).wire2api(),
            uint8list: self_.get(6).wire2api(),
            int32list: self_.get(7).wire2api(),
            float32list: self_.get(8).wire2api(),
            float64list: self_.get(9).wire2api(),
            attributes: self_.get(10).wire2api(),
            attributes_nullable: self_.get(11).wire2api(),
            nullable_attributes: self_.get(12).wire2api(),
            newtypeint: self_.get(13).wire2api(),
        }
    }
}
impl Wire2Api<[f64; 16]> for Box<[f64]> {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::chrono_type::FeatureChrono> for JsValue {
    fn wire2api(self) -> crate::api::chrono_type::FeatureChrono {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::chrono_type::FeatureChrono {
            utc: self_.get(0).wire2api(),
            local: self_.get(1).wire2api(),
            duration: self_.get(2).wire2api(),
            naive: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::uuid_type::FeatureUuid> for JsValue {
    fn wire2api(self) -> crate::api::uuid_type::FeatureUuid {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::uuid_type::FeatureUuid {
            one: self_.get(0).wire2api(),
            many: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::FeedId> for JsValue {
    fn wire2api(self) -> crate::api::array::FeedId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::array::FeedId(self_.get(0).wire2api())
    }
}
impl Wire2Api<[i32; 2]> for Box<[i32]> {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Vec<flutter_rust_bridge::DartOpaque>> for JsValue {
    fn wire2api(self) -> Vec<flutter_rust_bridge::DartOpaque> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>>
    for JsValue
{
    fn wire2api(
        self,
    ) -> Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::mirror::ApplicationEnvVar>> for JsValue {
    fn wire2api(self) -> Vec<crate::api::mirror::ApplicationEnvVar> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::optional::Attribute>> for JsValue {
    fn wire2api(self) -> Vec<crate::api::optional::Attribute> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
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
impl Wire2Api<Vec<crate::auxiliary::sample_types::MySize>> for JsValue {
    fn wire2api(self) -> Vec<crate::auxiliary::sample_types::MySize> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::misc_example::MyTreeNode>> for JsValue {
    fn wire2api(self) -> Vec<crate::api::misc_example::MyTreeNode> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<String>>> for JsValue {
    fn wire2api(self) -> Vec<Option<String>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::optional::Attribute>>> for JsValue {
    fn wire2api(self) -> Vec<Option<crate::api::optional::Attribute>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<i32>>> for JsValue {
    fn wire2api(self) -> Vec<Option<i32>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::misc_example::Weekdays>>> for JsValue {
    fn wire2api(self) -> Vec<Option<crate::api::misc_example::Weekdays>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<Vec<i32>>>> for JsValue {
    fn wire2api(self) -> Vec<Option<Vec<i32>>> {
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
impl Wire2Api<Vec<(String, i32)>> for JsValue {
    fn wire2api(self) -> Vec<(String, i32)> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::array::TestId>> for JsValue {
    fn wire2api(self) -> Vec<crate::api::array::TestId> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::misc_example::Weekdays>> for JsValue {
    fn wire2api(self) -> Vec<crate::api::misc_example::Weekdays> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<crate::api::inside_macro::MacroStruct> for JsValue {
    fn wire2api(self) -> crate::api::inside_macro::MacroStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::inside_macro::MacroStruct {
            data: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::Measure> for JsValue {
    fn wire2api(self) -> crate::api::enumeration::Measure {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::Measure::Speed(self_.get(1).wire2api()),
            1 => crate::api::enumeration::Measure::Distance(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::array::MessageId> for JsValue {
    fn wire2api(self) -> crate::api::array::MessageId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::array::MessageId(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::misc_example::MyNestedStruct> for JsValue {
    fn wire2api(self) -> crate::api::misc_example::MyNestedStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::MyNestedStruct {
            tree_node: self_.get(0).wire2api(),
            weekday: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MySize> for JsValue {
    fn wire2api(self) -> crate::auxiliary::sample_types::MySize {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::auxiliary::sample_types::MySize {
            width: self_.get(0).wire2api(),
            height: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MyStruct> for JsValue {
    fn wire2api(self) -> crate::auxiliary::sample_types::MyStruct {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::auxiliary::sample_types::MyStruct {
            content: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::MyTreeNode> for JsValue {
    fn wire2api(self) -> crate::api::misc_example::MyTreeNode {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::MyTreeNode {
            value_i32: self_.get(0).wire2api(),
            value_vec_u8: self_.get(1).wire2api(),
            value_boolean: self_.get(2).wire2api(),
            children: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::newtype_pattern::NewTypeInt> for JsValue {
    fn wire2api(self) -> crate::api::newtype_pattern::NewTypeInt {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::newtype_pattern::NewTypeInt(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::enumeration::Note> for JsValue {
    fn wire2api(self) -> crate::api::enumeration::Note {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::enumeration::Note {
            day: self_.get(0).wire2api(),
            body: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::mirror::Numbers> for JsValue {
    fn wire2api(self) -> crate::api::mirror::Numbers {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::mirror::Numbers(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::rust_opaque::OpaqueNested> for JsValue {
    fn wire2api(self) -> crate::api::rust_opaque::OpaqueNested {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::rust_opaque::OpaqueNested {
            first: self_.get(0).wire2api(),
            second: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<Option<String>> for Option<String> {
    fn wire2api(self) -> Option<String> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Option<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<f32>>> for Option<Box<[f32]>> {
    fn wire2api(self) -> Option<Vec<f32>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<f64>>> for Option<Box<[f64]>> {
    fn wire2api(self) -> Option<Vec<f64>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<i32>>> for Option<Box<[i32]>> {
    fn wire2api(self) -> Option<Vec<i32>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<i8>>> for Option<Box<[i8]>> {
    fn wire2api(self) -> Option<Vec<i8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<u8>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Option<Vec<u8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<crate::api::optional::OptVecs> for JsValue {
    fn wire2api(self) -> crate::api::optional::OptVecs {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::optional::OptVecs {
            i32: self_.get(0).wire2api(),
            enums: self_.get(1).wire2api(),
            strings: self_.get(2).wire2api(),
            buffers: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<(String, i32)> for JsValue {
    fn wire2api(self) -> (String, i32) {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        (self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}
impl Wire2Api<crate::api::mirror::Sequences> for JsValue {
    fn wire2api(self) -> crate::api::mirror::Sequences {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::mirror::Sequences(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::enumeration::Speed> for JsValue {
    fn wire2api(self) -> crate::api::enumeration::Speed {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::Speed::Unknown,
            1 => crate::api::enumeration::Speed::GPS(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::comment::StructWithCommentsTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::comment::StructWithCommentsTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::comment::StructWithCommentsTwinNormal {
            field_with_comments: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync>
    for JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
            field_with_comments: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::StructWithEnum> for JsValue {
    fn wire2api(self) -> crate::api::misc_example::StructWithEnum {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::StructWithEnum {
            abc1: self_.get(0).wire2api(),
            abc2: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithOneFieldTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::structure::StructWithOneFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::structure::StructWithOneFieldTwinNormal {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithTwoFieldTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::structure::StructWithTwoFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::structure::StructWithTwoFieldTwinNormal {
            a: self_.get(0).wire2api(),
            b: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
            a: self_.get(0).wire2api(),
            b: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithZeroFieldTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::structure::StructWithZeroFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            0,
            "Expected 0 elements, got {}",
            self_.length()
        );
        crate::api::structure::StructWithZeroFieldTwinNormal {}
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            0,
            "Expected 0 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {}
    }
}
impl Wire2Api<crate::api::method::SumWith> for JsValue {
    fn wire2api(self) -> crate::api::method::SumWith {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::method::SumWith {
            x: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::TestId> for JsValue {
    fn wire2api(self) -> crate::api::array::TestId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::array::TestId(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithOneFieldTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::structure::TupleStructWithOneFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::structure::TupleStructWithOneFieldTwinNormal(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync(
            self_.get(0).wire2api(),
        )
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithTwoFieldTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::structure::TupleStructWithTwoFieldTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::structure::TupleStructWithTwoFieldTwinNormal(
            self_.get(0).wire2api(),
            self_.get(1).wire2api(),
        )
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync>
    for JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync(
            self_.get(0).wire2api(),
            self_.get(1).wire2api(),
        )
    }
}
impl Wire2Api<[u8; 1600]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::attribute::UserId> for JsValue {
    fn wire2api(self) -> crate::api::attribute::UserId {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::attribute::UserId {
            value: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<chrono::Duration> for JsValue {
    fn wire2api(self) -> chrono::Duration {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<Vec<chrono::Duration>> for JsValue {
    fn wire2api(self) -> Vec<chrono::Duration> {
        self.unchecked_into::<js_sys::BigInt64Array>()
            .to_vec()
            .into_iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Local>> for JsValue {
    fn wire2api(self) -> chrono::DateTime<chrono::Local> {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<chrono::NaiveDateTime> for JsValue {
    fn wire2api(self) -> chrono::NaiveDateTime {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<Vec<chrono::NaiveDateTime>> for JsValue {
    fn wire2api(self) -> Vec<chrono::NaiveDateTime> {
        self.unchecked_into::<js_sys::BigInt64Array>()
            .to_vec()
            .into_iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Utc>> for JsValue {
    fn wire2api(self) -> chrono::DateTime<chrono::Utc> {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<[flutter_rust_bridge::DartOpaque; 1]> for JsValue {
    fn wire2api(self) -> [flutter_rust_bridge::DartOpaque; 1] {
        let vec: Vec<flutter_rust_bridge::DartOpaque> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Mutex<HideData>>> for JsValue {
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Mutex<HideData>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<RwLock<HideData>>> for JsValue {
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<RwLock<HideData>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Box<dyn DartDebug>>> for JsValue {
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Box<dyn DartDebug>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>
    for JsValue
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<[flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>; 2]>
    for JsValue
{
    fn wire2api(
        self,
    ) -> [flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>; 2] {
        let vec: Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>> =
            self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<i32>> for JsValue {
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<i32> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonCloneData>>
    for JsValue
{
    fn wire2api(
        self,
    ) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonCloneData> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonSendHideData>>
    for JsValue
{
    fn wire2api(
        self,
    ) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonSendHideData> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<uuid::Uuid> for JsValue {
    fn wire2api(self) -> uuid::Uuid {
        self.unchecked_into::<js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for JsValue {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        self.unchecked_into::<js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>> for JsValue {
    fn wire2api(self) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
        flutter_rust_bridge::ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<crate::api::mirror::ApplicationMode> for JsValue {
    fn wire2api(self) -> crate::api::mirror::ApplicationMode {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<Box<crate::api::mirror::ApplicationEnv>> for JsValue {
    fn wire2api(self) -> Box<crate::api::mirror::ApplicationEnv> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::array::Blob>> for JsValue {
    fn wire2api(self) -> Box<crate::api::array::Blob> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<bool>> for JsValue {
    fn wire2api(self) -> Box<bool> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::enumeration::Distance>> for JsValue {
    fn wire2api(self) -> Box<crate::api::enumeration::Distance> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::optional::ExoticOptionals>> for JsValue {
    fn wire2api(self) -> Box<crate::api::optional::ExoticOptionals> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<f64>> for JsValue {
    fn wire2api(self) -> Box<f64> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<i32>> for JsValue {
    fn wire2api(self) -> Box<i32> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<i64>> for JsValue {
    fn wire2api(self) -> Box<i64> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<i8>> for JsValue {
    fn wire2api(self) -> Box<i8> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::enumeration::Speed>> for JsValue {
    fn wire2api(self) -> Box<crate::api::enumeration::Speed> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<u8>> for JsValue {
    fn wire2api(self) -> Box<u8> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<[u8; 1600]>> for JsValue {
    fn wire2api(self) -> Box<[u8; 1600]> {
        let vec: Vec<u8> = self.wire2api();
        Box::new(flutter_rust_bridge::support::from_vec_to_array(vec))
    }
}
impl Wire2Api<Box<crate::api::misc_example::Weekdays>> for JsValue {
    fn wire2api(self) -> Box<crate::api::misc_example::Weekdays> {
        let ptr: Box<i32> = self.wire2api();
        Box::new(ptr.wire2api())
    }
}
impl Wire2Api<crate::api::enumeration::EnumSimpleTwinNormal> for JsValue {
    fn wire2api(self) -> crate::api::enumeration::EnumSimpleTwinNormal {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumSimpleTwinSync> for JsValue {
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumSimpleTwinSync {
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
        flutter_rust_bridge::support::from_vec_to_array(vec)
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
        flutter_rust_bridge::support::from_vec_to_array(vec)
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
        flutter_rust_bridge::support::slice_from_byte_buffer(buf.to_vec()).into()
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
        flutter_rust_bridge::support::slice_from_byte_buffer(buf.to_vec()).into()
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MyEnum> for JsValue {
    fn wire2api(self) -> crate::auxiliary::sample_types::MyEnum {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<[crate::api::array::TestId; 4]> for JsValue {
    fn wire2api(self) -> [crate::api::array::TestId; 4] {
        let vec: Vec<crate::api::array::TestId> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
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
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for JsValue {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for JsValue {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<usize> for JsValue {
    fn wire2api(self) -> usize {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<crate::api::misc_example::Weekdays> for JsValue {
    fn wire2api(self) -> crate::api::misc_example::Weekdays {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}

#[wasm_bindgen]
pub fn wire_boxed_blob(port_: flutter_rust_bridge::MessagePort, blob: Box<[u8]>) {
    wire_boxed_blob_impl(port_, blob)
}

#[wasm_bindgen]
pub fn wire_func_test_id(port_: flutter_rust_bridge::MessagePort, id: JsValue) {
    wire_func_test_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_get_array(port_: flutter_rust_bridge::MessagePort) {
    wire_get_array_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_complex_array(port_: flutter_rust_bridge::MessagePort) {
    wire_get_complex_array_impl(port_)
}

#[wasm_bindgen]
pub fn wire_last_number(port_: flutter_rust_bridge::MessagePort, array: Box<[f64]>) {
    wire_last_number_impl(port_, array)
}

#[wasm_bindgen]
pub fn wire_nested_id(port_: flutter_rust_bridge::MessagePort, id: JsValue) {
    wire_nested_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_new_msgid(port_: flutter_rust_bridge::MessagePort, id: Box<[u8]>) {
    wire_new_msgid_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_return_boxed_feed_id(port_: flutter_rust_bridge::MessagePort, id: Box<[u8]>) {
    wire_return_boxed_feed_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_return_boxed_raw_feed_id(port_: flutter_rust_bridge::MessagePort, id: JsValue) {
    wire_return_boxed_raw_feed_id_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_use_boxed_blob(port_: flutter_rust_bridge::MessagePort, blob: JsValue) {
    wire_use_boxed_blob_impl(port_, blob)
}

#[wasm_bindgen]
pub fn wire_use_msgid(port_: flutter_rust_bridge::MessagePort, id: JsValue) {
    wire_use_msgid_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_handle_customized_struct(port_: flutter_rust_bridge::MessagePort, val: JsValue) {
    wire_handle_customized_struct_impl(port_, val)
}

#[wasm_bindgen]
pub fn wire_next_user_id(port_: flutter_rust_bridge::MessagePort, user_id: JsValue) {
    wire_next_user_id_impl(port_, user_id)
}

#[wasm_bindgen]
pub fn wire_datetime_local(port_: flutter_rust_bridge::MessagePort, d: i64) {
    wire_datetime_local_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_datetime_utc(port_: flutter_rust_bridge::MessagePort, d: i64) {
    wire_datetime_utc_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_duration(port_: flutter_rust_bridge::MessagePort, d: i64) {
    wire_duration_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_handle_durations(
    port_: flutter_rust_bridge::MessagePort,
    durations: Box<[i64]>,
    since: i64,
) {
    wire_handle_durations_impl(port_, durations, since)
}

#[wasm_bindgen]
pub fn wire_handle_timestamps(
    port_: flutter_rust_bridge::MessagePort,
    timestamps: Box<[i64]>,
    epoch: i64,
) {
    wire_handle_timestamps_impl(port_, timestamps, epoch)
}

#[wasm_bindgen]
pub fn wire_how_long_does_it_take(port_: flutter_rust_bridge::MessagePort, mine: JsValue) {
    wire_how_long_does_it_take_impl(port_, mine)
}

#[wasm_bindgen]
pub fn wire_naivedatetime(port_: flutter_rust_bridge::MessagePort, d: i64) {
    wire_naivedatetime_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_optional_empty_datetime_utc(port_: flutter_rust_bridge::MessagePort, d: JsValue) {
    wire_optional_empty_datetime_utc_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_test_chrono(port_: flutter_rust_bridge::MessagePort) {
    wire_test_chrono_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_precise_chrono(port_: flutter_rust_bridge::MessagePort) {
    wire_test_precise_chrono_impl(port_)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: JsValue,
) {
    wire_StructWithCommentsTwinNormal_instance_method_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinNormal_static_method_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_StructWithCommentsTwinNormal_static_method_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_function_with_comments_slash_star_star_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_function_with_comments_triple_slash_multi_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_function_with_comments_triple_slash_single_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_dart_dynamic(port_: flutter_rust_bridge::MessagePort) {
    wire_return_dart_dynamic_impl(port_)
}

#[wasm_bindgen]
pub fn wire_async_accept_dart_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_async_accept_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_create_enum_dart_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_create_enum_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_create_nested_dart_opaque(
    port_: flutter_rust_bridge::MessagePort,
    opaque1: JsValue,
    opaque2: JsValue,
) {
    wire_create_nested_dart_opaque_impl(port_, opaque1, opaque2)
}

#[wasm_bindgen]
pub fn wire_drop_static_dart_opaque(port_: flutter_rust_bridge::MessagePort) {
    wire_drop_static_dart_opaque_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_enum_dart_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_get_enum_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_get_nested_dart_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_get_nested_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_loop_back_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_array(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_loop_back_array_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_array_get(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_loop_back_array_get_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_option(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_loop_back_option_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_option_get(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_loop_back_option_get_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_vec(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_loop_back_vec_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_vec_get(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_loop_back_vec_get_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_panic_unwrap_dart_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_panic_unwrap_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_set_static_dart_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_set_static_dart_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_return_non_droppable_dart_opaque(
    opaque: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_non_droppable_dart_opaque_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_unwrap_dart_opaque(opaque: JsValue) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_unwrap_dart_opaque_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_func_enum_simple_twin_normal(port_: flutter_rust_bridge::MessagePort, arg: i32) {
    wire_func_enum_simple_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_mixed_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_func_enum_with_item_mixed_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_func_enum_with_item_struct_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_tuple_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_func_enum_with_item_tuple_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_enum_parameter(port_: flutter_rust_bridge::MessagePort, weekday: i32) {
    wire_handle_enum_parameter_impl(port_, weekday)
}

#[wasm_bindgen]
pub fn wire_handle_return_enum(port_: flutter_rust_bridge::MessagePort, input: String) {
    wire_handle_return_enum_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_multiply_by_ten(port_: flutter_rust_bridge::MessagePort, measure: JsValue) {
    wire_multiply_by_ten_impl(port_, measure)
}

#[wasm_bindgen]
pub fn wire_print_note(port_: flutter_rust_bridge::MessagePort, note: JsValue) {
    wire_print_note_impl(port_, note)
}

#[wasm_bindgen]
pub fn wire_Event_as_string(port_: flutter_rust_bridge::MessagePort, that: JsValue) {
    wire_Event_as_string_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_close_event_listener(port_: flutter_rust_bridge::MessagePort) {
    wire_close_event_listener_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_event(
    port_: flutter_rust_bridge::MessagePort,
    address: String,
    payload: String,
) {
    wire_create_event_impl(port_, address, payload)
}

#[wasm_bindgen]
pub fn wire_register_event_listener(port_: flutter_rust_bridge::MessagePort) {
    wire_register_event_listener_impl(port_)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_panic_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_custom_enum_error_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_error_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_custom_enum_error_return_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_ok_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u32,
) {
    wire_custom_enum_error_return_ok_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_custom_nested_error_return_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_custom_nested_error_return_error_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_custom_struct_error_return_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_custom_struct_error_return_error_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_return_error_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_return_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_type_fallible_panic_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_type_fallible_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_type_infallible_panic_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_type_infallible_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_call_new_module_system(port_: flutter_rust_bridge::MessagePort) {
    wire_call_new_module_system_impl(port_)
}

#[wasm_bindgen]
pub fn wire_call_old_module_system(port_: flutter_rust_bridge::MessagePort) {
    wire_call_old_module_system_impl(port_)
}

#[wasm_bindgen]
pub fn wire_use_imported_enum(port_: flutter_rust_bridge::MessagePort, my_enum: i32) {
    wire_use_imported_enum_impl(port_, my_enum)
}

#[wasm_bindgen]
pub fn wire_use_imported_struct(port_: flutter_rust_bridge::MessagePort, my_struct: JsValue) {
    wire_use_imported_struct_impl(port_, my_struct)
}

#[wasm_bindgen]
pub fn wire_func_macro_struct(port_: flutter_rust_bridge::MessagePort, arg: JsValue) {
    wire_func_macro_struct_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWith_concatenate(
    port_: flutter_rust_bridge::MessagePort,
    that: JsValue,
    b: String,
) {
    wire_ConcatenateWith_concatenate_impl(port_, that, b)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWith_concatenate_static(
    port_: flutter_rust_bridge::MessagePort,
    a: String,
    b: String,
) {
    wire_ConcatenateWith_concatenate_static_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWith_handle_some_static_stream_sink(
    port_: flutter_rust_bridge::MessagePort,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWith_handle_some_static_stream_sink_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWith_handle_some_static_stream_sink_single_arg(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_ConcatenateWith_handle_some_static_stream_sink_single_arg_impl(port_)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWith_handle_some_stream_sink(
    port_: flutter_rust_bridge::MessagePort,
    that: JsValue,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWith_handle_some_stream_sink_impl(port_, that, key, max)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWith_handle_some_stream_sink_at_1(
    port_: flutter_rust_bridge::MessagePort,
    that: JsValue,
) {
    wire_ConcatenateWith_handle_some_stream_sink_at_1_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWith_new(port_: flutter_rust_bridge::MessagePort, a: String) {
    wire_ConcatenateWith_new_impl(port_, a)
}

#[wasm_bindgen]
pub fn wire_SumWith_sum(port_: flutter_rust_bridge::MessagePort, that: JsValue, y: u32, z: u32) {
    wire_SumWith_sum_impl(port_, that, y, z)
}

#[wasm_bindgen]
pub fn wire_get_sum_array(port_: flutter_rust_bridge::MessagePort, a: u32, b: u32, c: u32) {
    wire_get_sum_array_impl(port_, a, b, c)
}

#[wasm_bindgen]
pub fn wire_get_sum_struct(port_: flutter_rust_bridge::MessagePort) {
    wire_get_sum_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_app_settings_stream(port_: flutter_rust_bridge::MessagePort) {
    wire_app_settings_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_app_settings_vec_stream(port_: flutter_rust_bridge::MessagePort) {
    wire_app_settings_vec_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_first_number(port_: flutter_rust_bridge::MessagePort, nums: JsValue) {
    wire_first_number_impl(port_, nums)
}

#[wasm_bindgen]
pub fn wire_first_sequence(port_: flutter_rust_bridge::MessagePort, seqs: JsValue) {
    wire_first_sequence_impl(port_, seqs)
}

#[wasm_bindgen]
pub fn wire_get_app_settings(port_: flutter_rust_bridge::MessagePort) {
    wire_get_app_settings_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_fallible_app_settings(port_: flutter_rust_bridge::MessagePort) {
    wire_get_fallible_app_settings_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_message(port_: flutter_rust_bridge::MessagePort) {
    wire_get_message_impl(port_)
}

#[wasm_bindgen]
pub fn wire_is_app_embedded(port_: flutter_rust_bridge::MessagePort, app_settings: JsValue) {
    wire_is_app_embedded_impl(port_, app_settings)
}

#[wasm_bindgen]
pub fn wire_mirror_struct_stream(port_: flutter_rust_bridge::MessagePort) {
    wire_mirror_struct_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_mirror_tuple_stream(port_: flutter_rust_bridge::MessagePort) {
    wire_mirror_tuple_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_repeat_number(port_: flutter_rust_bridge::MessagePort, num: i32, times: usize) {
    wire_repeat_number_impl(port_, num, times)
}

#[wasm_bindgen]
pub fn wire_repeat_sequence(port_: flutter_rust_bridge::MessagePort, seq: i32, times: usize) {
    wire_repeat_sequence_impl(port_, seq, times)
}

#[wasm_bindgen]
pub fn wire_test_contains_mirrored_sub_struct(port_: flutter_rust_bridge::MessagePort) {
    wire_test_contains_mirrored_sub_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_fallible_of_raw_string_mirrored(port_: flutter_rust_bridge::MessagePort) {
    wire_test_fallible_of_raw_string_mirrored_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_list_of_nested_enums_mirrored(port_: flutter_rust_bridge::MessagePort) {
    wire_test_list_of_nested_enums_mirrored_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_list_of_raw_nested_string_mirrored(port_: flutter_rust_bridge::MessagePort) {
    wire_test_list_of_raw_nested_string_mirrored_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_nested_raw_string_mirrored(port_: flutter_rust_bridge::MessagePort) {
    wire_test_nested_raw_string_mirrored_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_enum_mirrored(port_: flutter_rust_bridge::MessagePort, nested: bool) {
    wire_test_raw_string_enum_mirrored_impl(port_, nested)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_mirrored(port_: flutter_rust_bridge::MessagePort) {
    wire_test_raw_string_mirrored_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_big_buffers(port_: flutter_rust_bridge::MessagePort) {
    wire_handle_big_buffers_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_complex_struct(port_: flutter_rust_bridge::MessagePort, s: JsValue) {
    wire_handle_complex_struct_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_nested_struct(port_: flutter_rust_bridge::MessagePort, s: JsValue) {
    wire_handle_nested_struct_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_list_of_primitive_enums(port_: flutter_rust_bridge::MessagePort, weekdays: JsValue) {
    wire_list_of_primitive_enums_impl(port_, weekdays)
}

#[wasm_bindgen]
pub fn wire_test_abc_enum(port_: flutter_rust_bridge::MessagePort, abc: JsValue) {
    wire_test_abc_enum_impl(port_, abc)
}

#[wasm_bindgen]
pub fn wire_test_struct_with_enum(port_: flutter_rust_bridge::MessagePort, se: JsValue) {
    wire_test_struct_with_enum_impl(port_, se)
}

#[wasm_bindgen]
pub fn wire_func_return_unit_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_return_unit_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_string_twin_normal(port_: flutter_rust_bridge::MessagePort, arg: String) {
    wire_func_string_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_list_of_struct(port_: flutter_rust_bridge::MessagePort, l: JsValue) {
    wire_handle_list_of_struct_impl(port_, l)
}

#[wasm_bindgen]
pub fn wire_handle_string_list(port_: flutter_rust_bridge::MessagePort, names: JsValue) {
    wire_handle_string_list_impl(port_, names)
}

#[wasm_bindgen]
pub fn wire_handle_newtype(port_: flutter_rust_bridge::MessagePort, arg: JsValue) {
    wire_handle_newtype_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_increment_boxed_optional(port_: flutter_rust_bridge::MessagePort, opt: JsValue) {
    wire_handle_increment_boxed_optional_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_handle_option_box_arguments(
    port_: flutter_rust_bridge::MessagePort,
    i8box: JsValue,
    u8box: JsValue,
    i32box: JsValue,
    i64box: JsValue,
    f64box: JsValue,
    boolbox: JsValue,
    structbox: JsValue,
) {
    wire_handle_option_box_arguments_impl(
        port_, i8box, u8box, i32box, i64box, f64box, boolbox, structbox,
    )
}

#[wasm_bindgen]
pub fn wire_handle_optional_increment(port_: flutter_rust_bridge::MessagePort, opt: JsValue) {
    wire_handle_optional_increment_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_handle_optional_return(port_: flutter_rust_bridge::MessagePort, left: f64, right: f64) {
    wire_handle_optional_return_impl(port_, left, right)
}

#[wasm_bindgen]
pub fn wire_handle_optional_struct(
    port_: flutter_rust_bridge::MessagePort,
    document: Option<String>,
) {
    wire_handle_optional_struct_impl(port_, document)
}

#[wasm_bindgen]
pub fn wire_handle_vec_of_opts(port_: flutter_rust_bridge::MessagePort, opt: JsValue) {
    wire_handle_vec_of_opts_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    that: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_instance_method_twin_sync_impl(that)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinSync_static_method_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_static_method_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_slash_star_star_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_triple_slash_multi_line_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_triple_slash_single_line_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_enum_simple_twin_sync(arg: i32) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_simple_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_mixed_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_mixed_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_struct_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_struct_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_tuple_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_tuple_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_panic_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_panic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_return_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_ok_twin_sync(
    arg: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_return_ok_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_custom_nested_error_return_error_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_nested_error_return_error_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_custom_struct_error_return_error_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_struct_error_return_error_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_return_error_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_return_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_type_fallible_panic_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_type_fallible_panic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_type_infallible_panic_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_type_infallible_panic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_return_unit_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_return_unit_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_string_twin_sync(arg: String) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_string_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_bool_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_optional_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_bool_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f32_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f64_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i16_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i32_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i64_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i8_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u16_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u32_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u64_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u8_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_bool_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: bool,
) {
    wire_example_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: f32,
) {
    wire_example_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: f64,
) {
    wire_example_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: i16,
) {
    wire_example_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: i32,
) {
    wire_example_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: i64,
) {
    wire_example_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: i8,
) {
    wire_example_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u16,
) {
    wire_example_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u32,
) {
    wire_example_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u64,
) {
    wire_example_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u8,
) {
    wire_example_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_bool_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_example_primitive_list_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[f32]>,
) {
    wire_example_primitive_list_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[f64]>,
) {
    wire_example_primitive_list_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[i16]>,
) {
    wire_example_primitive_list_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[i32]>,
) {
    wire_example_primitive_list_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[i64]>,
) {
    wire_example_primitive_list_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[i8]>,
) {
    wire_example_primitive_list_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[u16]>,
) {
    wire_example_primitive_list_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[u32]>,
) {
    wire_example_primitive_list_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[u64]>,
) {
    wire_example_primitive_list_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[u8]>,
) {
    wire_example_primitive_list_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_bool_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f32_twin_sync(
    arg: Box<[f32]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f64_twin_sync(
    arg: Box<[f64]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i16_twin_sync(
    arg: Box<[i16]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i32_twin_sync(
    arg: Box<[i32]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i64_twin_sync(
    arg: Box<[i64]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i8_twin_sync(
    arg: Box<[i8]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u16_twin_sync(
    arg: Box<[u16]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u32_twin_sync(
    arg: Box<[u32]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u64_twin_sync(
    arg: Box<[u64]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u8_twin_sync(
    arg: Box<[u8]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_bool_twin_sync(
    arg: bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f32_twin_sync(
    arg: f32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f64_twin_sync(
    arg: f64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i16_twin_sync(
    arg: i16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i32_twin_sync(
    arg: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i64_twin_sync(
    arg: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i8_twin_sync(
    arg: i8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u16_twin_sync(
    arg: u16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u32_twin_sync(
    arg: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u64_twin_sync(
    arg: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u8_twin_sync(
    arg: u8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_simple_adder_twin_sync(a: i32, b: i32) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_simple_adder_twin_sync_impl(a, b)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_one_field_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_one_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_two_field_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_two_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_zero_field_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_zero_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_one_field_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_tuple_struct_with_one_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_two_field_twin_sync(
    arg: JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_tuple_struct_with_two_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_test_more_than_just_one_raw_string_struct(port_: flutter_rust_bridge::MessagePort) {
    wire_test_more_than_just_one_raw_string_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_item_struct(port_: flutter_rust_bridge::MessagePort) {
    wire_test_raw_string_item_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_array_opaque_enum(port_: flutter_rust_bridge::MessagePort) {
    wire_create_array_opaque_enum_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_nested_opaque(port_: flutter_rust_bridge::MessagePort) {
    wire_create_nested_opaque_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_opaque(port_: flutter_rust_bridge::MessagePort) {
    wire_create_opaque_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_option_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_create_option_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_create_sync_opaque(port_: flutter_rust_bridge::MessagePort) {
    wire_create_sync_opaque_impl(port_)
}

#[wasm_bindgen]
pub fn wire_frb_generator_test(port_: flutter_rust_bridge::MessagePort) {
    wire_frb_generator_test_impl(port_)
}

#[wasm_bindgen]
pub fn wire_opaque_array(port_: flutter_rust_bridge::MessagePort) {
    wire_opaque_array_impl(port_)
}

#[wasm_bindgen]
pub fn wire_opaque_array_run(port_: flutter_rust_bridge::MessagePort, data: JsValue) {
    wire_opaque_array_run_impl(port_, data)
}

#[wasm_bindgen]
pub fn wire_opaque_vec(port_: flutter_rust_bridge::MessagePort) {
    wire_opaque_vec_impl(port_)
}

#[wasm_bindgen]
pub fn wire_opaque_vec_run(port_: flutter_rust_bridge::MessagePort, data: JsValue) {
    wire_opaque_vec_run_impl(port_, data)
}

#[wasm_bindgen]
pub fn wire_run_enum_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_run_enum_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_run_nested_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_run_nested_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_run_non_clone(port_: flutter_rust_bridge::MessagePort, clone: JsValue) {
    wire_run_non_clone_impl(port_, clone)
}

#[wasm_bindgen]
pub fn wire_run_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_run_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_run_opaque_with_delay(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_run_opaque_with_delay_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_unwrap_rust_opaque(port_: flutter_rust_bridge::MessagePort, opaque: JsValue) {
    wire_unwrap_rust_opaque_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_frb_sync_generator_test(port_: flutter_rust_bridge::MessagePort) {
    wire_frb_sync_generator_test_impl(port_)
}

#[wasm_bindgen]
pub fn wire_sync_run_opaque(opaque: JsValue) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_run_opaque_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_simple_adder_twin_normal(port_: flutter_rust_bridge::MessagePort, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_func_stream_realistic_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: String,
) {
    wire_func_stream_realistic_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_stream_return_error_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_stream_return_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_stream_return_panic_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_stream_return_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_stream_sink_arg_position_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    a: u32,
    b: u32,
) {
    wire_func_stream_sink_arg_position_twin_normal_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_handle_stream_of_struct(port_: flutter_rust_bridge::MessagePort) {
    wire_handle_stream_of_struct_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_one_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_func_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_two_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_func_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_zero_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_func_struct_with_zero_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_one_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_func_tuple_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_two_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: JsValue,
) {
    wire_func_tuple_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_test_tuple(port_: flutter_rust_bridge::MessagePort, value: JsValue) {
    wire_test_tuple_impl(port_, value)
}

#[wasm_bindgen]
pub fn wire_test_tuple_2(port_: flutter_rust_bridge::MessagePort, value: JsValue) {
    wire_test_tuple_2_impl(port_, value)
}

#[wasm_bindgen]
pub fn wire_handle_type_alias_id(port_: flutter_rust_bridge::MessagePort, input: u64) {
    wire_handle_type_alias_id_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_type_alias_model(port_: flutter_rust_bridge::MessagePort, input: u64) {
    wire_handle_type_alias_model_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_type_nest_alias_id(port_: flutter_rust_bridge::MessagePort, input: u64) {
    wire_handle_type_nest_alias_id_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_nested_uuids(port_: flutter_rust_bridge::MessagePort, ids: JsValue) {
    wire_handle_nested_uuids_impl(port_, ids)
}

#[wasm_bindgen]
pub fn wire_handle_uuid(port_: flutter_rust_bridge::MessagePort, id: Box<[u8]>) {
    wire_handle_uuid_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_handle_uuids(port_: flutter_rust_bridge::MessagePort, ids: Box<[u8]>) {
    wire_handle_uuids_impl(port_, ids)
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_MutexHideData(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Mutex<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_MutexHideData(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Mutex<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_RwLockHideData(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<RwLock<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_RwLockHideData(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<RwLock<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_box_dynDartDebug(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebug>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_box_dynDartDebug(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebug>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_frb_opaque_return(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueReturn>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_frb_opaque_return(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueReturn>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_frb_opaque_sync_return(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueSyncReturn>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_frb_opaque_sync_return(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueSyncReturn>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_hide_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::HideData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_hide_data(ptr: *const std::ffi::c_void) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::HideData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_i_32(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<i32>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_i_32(ptr: *const std::ffi::c_void) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<i32>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_non_clone_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonCloneData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_non_clone_data(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonCloneData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_non_send_hide_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonSendHideData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_non_send_hide_data(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonSendHideData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}
