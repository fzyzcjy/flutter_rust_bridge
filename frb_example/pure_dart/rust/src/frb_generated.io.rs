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
impl Wire2Api<flutter_rust_bridge::DartOpaque> for wire_DartOpaque {
    fn wire2api(self) -> flutter_rust_bridge::DartOpaque {
        unsafe { flutter_rust_bridge::DartOpaque::new(self.handle as _, self.port) }
    }
}
impl Wire2Api<[flutter_rust_bridge::DartOpaque; 1]> for *mut wire_list_DartOpaque {
    fn wire2api(self) -> [flutter_rust_bridge::DartOpaque; 1] {
        let vec: Vec<flutter_rust_bridge::DartOpaque> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Mutex<HideData>>> for wire_RustOpaque_MutexHideData {
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Mutex<HideData>> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<RwLock<HideData>>>
    for wire_RustOpaque_RwLockHideData
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<RwLock<HideData>> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Box<dyn DartDebug>>>
    for wire_RustOpaque_box_dynDartDebug
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Box<dyn DartDebug>> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>
    for wire_RustOpaque_hide_data
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<[flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>; 2]>
    for *mut wire_list_RustOpaque_hide_data
{
    fn wire2api(
        self,
    ) -> [flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>; 2] {
        let vec: Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>> =
            self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<i32>> for wire_RustOpaque_i_32 {
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<i32> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonCloneData>>
    for wire_RustOpaque_non_clone_data
{
    fn wire2api(
        self,
    ) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonCloneData> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonSendHideData>>
    for wire_RustOpaque_non_send_hide_data
{
    fn wire2api(
        self,
    ) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonSendHideData> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
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
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<uuid::Uuid> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> uuid::Uuid {
        let single: Vec<u8> = self.wire2api();
        flutter_rust_bridge::wire2api_uuid_ref(single.as_slice())
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        flutter_rust_bridge::wire2api_uuids(multiple)
    }
}
impl Wire2Api<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
        flutter_rust_bridge::ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<crate::api::misc_example::A> for wire_a {
    fn wire2api(self) -> crate::api::misc_example::A {
        crate::api::misc_example::A {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::Abc> for wire_abc {
    fn wire2api(self) -> crate::api::misc_example::Abc {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::misc_example::Abc::A(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::misc_example::Abc::B(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.C);
                crate::api::misc_example::Abc::C(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.JustInt);
                crate::api::misc_example::Abc::JustInt(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::mirror::ApplicationEnv> for wire_application_env {
    fn wire2api(self) -> crate::api::mirror::ApplicationEnv {
        crate::api::mirror::ApplicationEnv {
            vars: self.vars.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::mirror::ApplicationEnvVar> for wire_application_env_var {
    fn wire2api(self) -> crate::api::mirror::ApplicationEnvVar {
        crate::api::mirror::ApplicationEnvVar(self.field0.wire2api(), self.field1.wire2api())
    }
}
impl Wire2Api<crate::api::mirror::ApplicationSettings> for wire_application_settings {
    fn wire2api(self) -> crate::api::mirror::ApplicationSettings {
        crate::api::mirror::ApplicationSettings {
            name: self.name.wire2api(),
            version: self.version.wire2api(),
            mode: self.mode.wire2api(),
            env: self.env.wire2api(),
            env_optional: self.env_optional.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::Attribute> for wire_attribute {
    fn wire2api(self) -> crate::api::optional::Attribute {
        crate::api::optional::Attribute {
            key: self.key.wire2api(),
            value: self.value.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::B> for wire_b {
    fn wire2api(self) -> crate::api::misc_example::B {
        crate::api::misc_example::B {
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::Blob> for wire_blob {
    fn wire2api(self) -> crate::api::array::Blob {
        crate::api::array::Blob(self.field0.wire2api())
    }
}
impl Wire2Api<Box<crate::api::mirror::ApplicationEnv>> for *mut wire_application_env {
    fn wire2api(self) -> Box<crate::api::mirror::ApplicationEnv> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::mirror::ApplicationEnv>::wire2api(*wrap).into()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Utc>> for *mut i64 {
    fn wire2api(self) -> chrono::DateTime<chrono::Utc> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<chrono::DateTime<chrono::Utc>>::wire2api(*wrap).into()
    }
}
impl Wire2Api<flutter_rust_bridge::DartOpaque> for *mut wire_DartOpaque {
    fn wire2api(self) -> flutter_rust_bridge::DartOpaque {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<flutter_rust_bridge::DartOpaque>::wire2api(*wrap).into()
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>
    for *mut wire_RustOpaque_hide_data
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::A> for *mut wire_a {
    fn wire2api(self) -> crate::api::misc_example::A {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::A>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::Abc> for *mut wire_abc {
    fn wire2api(self) -> crate::api::misc_example::Abc {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::Abc>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::mirror::ApplicationEnv> for *mut wire_application_env {
    fn wire2api(self) -> crate::api::mirror::ApplicationEnv {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::mirror::ApplicationEnv>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::mirror::ApplicationSettings> for *mut wire_application_settings {
    fn wire2api(self) -> crate::api::mirror::ApplicationSettings {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::mirror::ApplicationSettings>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::optional::Attribute> for *mut wire_attribute {
    fn wire2api(self) -> crate::api::optional::Attribute {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::optional::Attribute>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::B> for *mut wire_b {
    fn wire2api(self) -> crate::api::misc_example::B {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::B>::wire2api(*wrap).into()
    }
}
impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<crate::api::misc_example::C> for *mut wire_c {
    fn wire2api(self) -> crate::api::misc_example::C {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::C>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::method::ConcatenateWith> for *mut wire_concatenate_with {
    fn wire2api(self) -> crate::api::method::ConcatenateWith {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::method::ConcatenateWith>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorInnerTwinNormal>
    for *mut wire_custom_nested_error_inner_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorInnerTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::exception::CustomNestedErrorInnerTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync>
    for *mut wire_custom_nested_error_inner_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorOuterTwinNormal>
    for *mut wire_custom_nested_error_outer_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorOuterTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::exception::CustomNestedErrorOuterTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync>
    for *mut wire_custom_nested_error_outer_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::exception::CustomStructErrorTwinNormal>
    for *mut wire_custom_struct_error_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomStructErrorTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::exception::CustomStructErrorTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync>
    for *mut wire_custom_struct_error_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::attribute::Customized> for *mut wire_customized {
    fn wire2api(self) -> crate::api::attribute::Customized {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::attribute::Customized>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::dart_opaque::DartOpaqueNested> for *mut wire_dart_opaque_nested {
    fn wire2api(self) -> crate::api::dart_opaque::DartOpaqueNested {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::dart_opaque::DartOpaqueNested>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::dart_opaque::EnumDartOpaque> for *mut wire_enum_dart_opaque {
    fn wire2api(self) -> crate::api::dart_opaque::EnumDartOpaque {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::dart_opaque::EnumDartOpaque>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::rust_opaque::EnumOpaque> for *mut wire_enum_opaque {
    fn wire2api(self) -> crate::api::rust_opaque::EnumOpaque {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::rust_opaque::EnumOpaque>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemMixedTwinNormal>
    for *mut wire_enum_with_item_mixed_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemMixedTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::EnumWithItemMixedTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync>
    for *mut wire_enum_with_item_mixed_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemStructTwinNormal>
    for *mut wire_enum_with_item_struct_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemStructTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::EnumWithItemStructTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync>
    for *mut wire_enum_with_item_struct_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemTupleTwinNormal>
    for *mut wire_enum_with_item_tuple_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemTupleTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::EnumWithItemTupleTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync>
    for *mut wire_enum_with_item_tuple_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::event_listener::Event> for *mut wire_event {
    fn wire2api(self) -> crate::api::event_listener::Event {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::event_listener::Event>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::optional::ExoticOptionals> for *mut wire_exotic_optionals {
    fn wire2api(self) -> crate::api::optional::ExoticOptionals {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::optional::ExoticOptionals>::wire2api(*wrap).into()
    }
}
impl Wire2Api<f32> for *mut f32 {
    fn wire2api(self) -> f32 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<crate::api::chrono_type::FeatureChrono> for *mut wire_feature_chrono {
    fn wire2api(self) -> crate::api::chrono_type::FeatureChrono {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::chrono_type::FeatureChrono>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::uuid_type::FeatureUuid> for *mut wire_feature_uuid {
    fn wire2api(self) -> crate::api::uuid_type::FeatureUuid {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::uuid_type::FeatureUuid>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::array::FeedId> for *mut wire_feed_id {
    fn wire2api(self) -> crate::api::array::FeedId {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::array::FeedId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<i16> for *mut i16 {
    fn wire2api(self) -> i16 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i8> for *mut i8 {
    fn wire2api(self) -> i8 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<crate::api::inside_macro::MacroStruct> for *mut wire_macro_struct {
    fn wire2api(self) -> crate::api::inside_macro::MacroStruct {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::inside_macro::MacroStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::enumeration::Measure> for *mut wire_measure {
    fn wire2api(self) -> crate::api::enumeration::Measure {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::Measure>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::array::MessageId> for *mut wire_message_id {
    fn wire2api(self) -> crate::api::array::MessageId {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::array::MessageId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::MyNestedStruct> for *mut wire_my_nested_struct {
    fn wire2api(self) -> crate::api::misc_example::MyNestedStruct {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::MyNestedStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MyStruct> for *mut wire_my_struct {
    fn wire2api(self) -> crate::auxiliary::sample_types::MyStruct {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::auxiliary::sample_types::MyStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::MyTreeNode> for *mut wire_my_tree_node {
    fn wire2api(self) -> crate::api::misc_example::MyTreeNode {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::MyTreeNode>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::newtype_pattern::NewTypeInt> for *mut wire_new_type_int {
    fn wire2api(self) -> crate::api::newtype_pattern::NewTypeInt {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::newtype_pattern::NewTypeInt>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::enumeration::Note> for *mut wire_note {
    fn wire2api(self) -> crate::api::enumeration::Note {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::Note>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::mirror::Numbers> for *mut wire_numbers {
    fn wire2api(self) -> crate::api::mirror::Numbers {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::mirror::Numbers>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::rust_opaque::OpaqueNested> for *mut wire_opaque_nested {
    fn wire2api(self) -> crate::api::rust_opaque::OpaqueNested {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::rust_opaque::OpaqueNested>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::optional::OptVecs> for *mut wire_opt_vecs {
    fn wire2api(self) -> crate::api::optional::OptVecs {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::optional::OptVecs>::wire2api(*wrap).into()
    }
}
impl Wire2Api<(String, i32)> for *mut wire_record_string_i_32 {
    fn wire2api(self) -> (String, i32) {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<(String, i32)>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::mirror::Sequences> for *mut wire_sequences {
    fn wire2api(self) -> crate::api::mirror::Sequences {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::mirror::Sequences>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::comment::StructWithCommentsTwinNormal>
    for *mut wire_struct_with_comments_twin_normal
{
    fn wire2api(self) -> crate::api::comment::StructWithCommentsTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::comment::StructWithCommentsTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync>
    for *mut wire_struct_with_comments_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::StructWithEnum> for *mut wire_struct_with_enum {
    fn wire2api(self) -> crate::api::misc_example::StructWithEnum {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::StructWithEnum>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::structure::StructWithOneFieldTwinNormal>
    for *mut wire_struct_with_one_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithOneFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::StructWithOneFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync>
    for *mut wire_struct_with_one_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::structure::StructWithTwoFieldTwinNormal>
    for *mut wire_struct_with_two_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithTwoFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::StructWithTwoFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync>
    for *mut wire_struct_with_two_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::structure::StructWithZeroFieldTwinNormal>
    for *mut wire_struct_with_zero_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithZeroFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::StructWithZeroFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync>
    for *mut wire_struct_with_zero_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::method::SumWith> for *mut wire_sum_with {
    fn wire2api(self) -> crate::api::method::SumWith {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::method::SumWith>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::array::TestId> for *mut wire_test_id {
    fn wire2api(self) -> crate::api::array::TestId {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::array::TestId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithOneFieldTwinNormal>
    for *mut wire_tuple_struct_with_one_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithOneFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::TupleStructWithOneFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync>
    for *mut wire_tuple_struct_with_one_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithTwoFieldTwinNormal>
    for *mut wire_tuple_struct_with_two_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithTwoFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::TupleStructWithTwoFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync>
    for *mut wire_tuple_struct_with_two_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u16> for *mut u16 {
    fn wire2api(self) -> u16 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u64> for *mut u64 {
    fn wire2api(self) -> u64 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u8> for *mut u8 {
    fn wire2api(self) -> u8 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<crate::api::attribute::UserId> for *mut wire_user_id {
    fn wire2api(self) -> crate::api::attribute::UserId {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::attribute::UserId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::Weekdays> for *mut i32 {
    fn wire2api(self) -> crate::api::misc_example::Weekdays {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::Weekdays>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::array::Blob>> for *mut wire_blob {
    fn wire2api(self) -> Box<crate::api::array::Blob> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::array::Blob>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<bool>> for *mut bool {
    fn wire2api(self) -> Box<bool> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<crate::api::enumeration::Distance>> for *mut wire_distance {
    fn wire2api(self) -> Box<crate::api::enumeration::Distance> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::Distance>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::optional::ExoticOptionals>> for *mut wire_exotic_optionals {
    fn wire2api(self) -> Box<crate::api::optional::ExoticOptionals> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::optional::ExoticOptionals>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<f64>> for *mut f64 {
    fn wire2api(self) -> Box<f64> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<i32>> for *mut i32 {
    fn wire2api(self) -> Box<i32> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<i64>> for *mut i64 {
    fn wire2api(self) -> Box<i64> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<i8>> for *mut i8 {
    fn wire2api(self) -> Box<i8> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<crate::api::enumeration::Speed>> for *mut wire_speed {
    fn wire2api(self) -> Box<crate::api::enumeration::Speed> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::Speed>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<u8>> for *mut u8 {
    fn wire2api(self) -> Box<u8> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<[u8; 1600]>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Box<[u8; 1600]> {
        Wire2Api::<[u8; 1600]>::wire2api(self).into()
    }
}
impl Wire2Api<Box<crate::api::misc_example::Weekdays>> for *mut i32 {
    fn wire2api(self) -> Box<crate::api::misc_example::Weekdays> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::Weekdays>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::C> for wire_c {
    fn wire2api(self) -> crate::api::misc_example::C {
        crate::api::misc_example::C {
            c: self.c.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::method::ConcatenateWith> for wire_concatenate_with {
    fn wire2api(self) -> crate::api::method::ConcatenateWith {
        crate::api::method::ConcatenateWith {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorInnerTwinNormal>
    for wire_custom_nested_error_inner_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorInnerTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Three);
                crate::api::exception::CustomNestedErrorInnerTwinNormal::Three(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Four);
                crate::api::exception::CustomNestedErrorInnerTwinNormal::Four(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync>
    for wire_custom_nested_error_inner_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Three);
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync::Three( ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Four);
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync::Four(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorOuterTwinNormal>
    for wire_custom_nested_error_outer_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorOuterTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.One);
                crate::api::exception::CustomNestedErrorOuterTwinNormal::One(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Two);
                crate::api::exception::CustomNestedErrorOuterTwinNormal::Two(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync>
    for wire_custom_nested_error_outer_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.One);
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync::One(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Two);
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync::Two(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomStructErrorTwinNormal>
    for wire_custom_struct_error_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomStructErrorTwinNormal {
        crate::api::exception::CustomStructErrorTwinNormal {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync>
    for wire_custom_struct_error_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
        crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::attribute::Customized> for wire_customized {
    fn wire2api(self) -> crate::api::attribute::Customized {
        crate::api::attribute::Customized {
            final_field: self.final_field.wire2api(),
            non_final_field: self.non_final_field.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::dart_opaque::DartOpaqueNested> for wire_dart_opaque_nested {
    fn wire2api(self) -> crate::api::dart_opaque::DartOpaqueNested {
        crate::api::dart_opaque::DartOpaqueNested {
            first: self.first.wire2api(),
            second: self.second.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::Distance> for wire_distance {
    fn wire2api(self) -> crate::api::enumeration::Distance {
        match self.tag {
            0 => crate::api::enumeration::Distance::Unknown,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Map);
                crate::api::enumeration::Distance::Map(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::dart_opaque::EnumDartOpaque> for wire_enum_dart_opaque {
    fn wire2api(self) -> crate::api::dart_opaque::EnumDartOpaque {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Primitive);
                crate::api::dart_opaque::EnumDartOpaque::Primitive(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Opaque);
                crate::api::dart_opaque::EnumDartOpaque::Opaque(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::rust_opaque::EnumOpaque> for wire_enum_opaque {
    fn wire2api(self) -> crate::api::rust_opaque::EnumOpaque {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Struct);
                crate::api::rust_opaque::EnumOpaque::Struct(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Primitive);
                crate::api::rust_opaque::EnumOpaque::Primitive(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.TraitObj);
                crate::api::rust_opaque::EnumOpaque::TraitObj(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Mutex);
                crate::api::rust_opaque::EnumOpaque::Mutex(ans.field0.wire2api())
            },
            4 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.RwLock);
                crate::api::rust_opaque::EnumOpaque::RwLock(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemMixedTwinNormal>
    for wire_enum_with_item_mixed_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemMixedTwinNormal {
        match self.tag {
            0 => crate::api::enumeration::EnumWithItemMixedTwinNormal::A,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::enumeration::EnumWithItemMixedTwinNormal::B(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.C);
                crate::api::enumeration::EnumWithItemMixedTwinNormal::C {
                    c_field: ans.c_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync>
    for wire_enum_with_item_mixed_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync {
        match self.tag {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::A,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::B(
                    ans.field0.wire2api(),
                )
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.C);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::C {
                    c_field: ans.c_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemStructTwinNormal>
    for wire_enum_with_item_struct_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemStructTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::enumeration::EnumWithItemStructTwinNormal::A {
                    a_field: ans.a_field.wire2api(),
                }
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::enumeration::EnumWithItemStructTwinNormal::B {
                    b_field: ans.b_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync>
    for wire_enum_with_item_struct_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync::A {
                    a_field: ans.a_field.wire2api(),
                }
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync::B {
                    b_field: ans.b_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemTupleTwinNormal>
    for wire_enum_with_item_tuple_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemTupleTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::enumeration::EnumWithItemTupleTwinNormal::A(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::enumeration::EnumWithItemTupleTwinNormal::B(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync>
    for wire_enum_with_item_tuple_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync::A(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync::B(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::event_listener::Event> for wire_event {
    fn wire2api(self) -> crate::api::event_listener::Event {
        crate::api::event_listener::Event {
            address: self.address.wire2api(),
            payload: self.payload.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::ExoticOptionals> for wire_exotic_optionals {
    fn wire2api(self) -> crate::api::optional::ExoticOptionals {
        crate::api::optional::ExoticOptionals {
            int32: self.int32.wire2api(),
            int64: self.int64.wire2api(),
            float64: self.float64.wire2api(),
            boolean: self.boolean.wire2api(),
            zerocopy: self.zerocopy.wire2api(),
            int8list: self.int8list.wire2api(),
            uint8list: self.uint8list.wire2api(),
            int32list: self.int32list.wire2api(),
            float32list: self.float32list.wire2api(),
            float64list: self.float64list.wire2api(),
            attributes: self.attributes.wire2api(),
            attributes_nullable: self.attributes_nullable.wire2api(),
            nullable_attributes: self.nullable_attributes.wire2api(),
            newtypeint: self.newtypeint.wire2api(),
        }
    }
}
impl Wire2Api<[f64; 16]> for *mut wire_list_prim_f_64 {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::chrono_type::FeatureChrono> for wire_feature_chrono {
    fn wire2api(self) -> crate::api::chrono_type::FeatureChrono {
        crate::api::chrono_type::FeatureChrono {
            utc: self.utc.wire2api(),
            local: self.local.wire2api(),
            duration: self.duration.wire2api(),
            naive: self.naive.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::uuid_type::FeatureUuid> for wire_feature_uuid {
    fn wire2api(self) -> crate::api::uuid_type::FeatureUuid {
        crate::api::uuid_type::FeatureUuid {
            one: self.one.wire2api(),
            many: self.many.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::FeedId> for wire_feed_id {
    fn wire2api(self) -> crate::api::array::FeedId {
        crate::api::array::FeedId(self.field0.wire2api())
    }
}
impl Wire2Api<[i32; 2]> for *mut wire_list_prim_i_32 {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Vec<flutter_rust_bridge::DartOpaque>> for *mut wire_list_DartOpaque {
    fn wire2api(self) -> Vec<flutter_rust_bridge::DartOpaque> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>>
    for *mut wire_list_RustOpaque_hide_data
{
    fn wire2api(
        self,
    ) -> Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::mirror::ApplicationEnvVar>> for *mut wire_list_application_env_var {
    fn wire2api(self) -> Vec<crate::api::mirror::ApplicationEnvVar> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::optional::Attribute>> for *mut wire_list_attribute {
    fn wire2api(self) -> Vec<crate::api::optional::Attribute> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<bool>> for *mut wire_list_bool {
    fn wire2api(self) -> Vec<bool> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::auxiliary::sample_types::MySize>> for *mut wire_list_my_size {
    fn wire2api(self) -> Vec<crate::auxiliary::sample_types::MySize> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::misc_example::MyTreeNode>> for *mut wire_list_my_tree_node {
    fn wire2api(self) -> Vec<crate::api::misc_example::MyTreeNode> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<String>>> for *mut wire_list_opt_String {
    fn wire2api(self) -> Vec<Option<String>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::optional::Attribute>>>
    for *mut wire_list_opt_box_autoadd_attribute
{
    fn wire2api(self) -> Vec<Option<crate::api::optional::Attribute>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<i32>>> for *mut wire_list_opt_box_autoadd_i_32 {
    fn wire2api(self) -> Vec<Option<i32>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::misc_example::Weekdays>>>
    for *mut wire_list_opt_box_autoadd_weekdays
{
    fn wire2api(self) -> Vec<Option<crate::api::misc_example::Weekdays>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<Vec<i32>>>> for *mut wire_list_opt_list_prim_i_32 {
    fn wire2api(self) -> Vec<Option<Vec<i32>>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<f32>> for *mut wire_list_prim_f_32 {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<f64>> for *mut wire_list_prim_f_64 {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i16>> for *mut wire_list_prim_i_16 {
    fn wire2api(self) -> Vec<i16> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i32>> for *mut wire_list_prim_i_32 {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i64>> for *mut wire_list_prim_i_64 {
    fn wire2api(self) -> Vec<i64> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i8>> for *mut wire_list_prim_i_8 {
    fn wire2api(self) -> Vec<i8> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u16>> for *mut wire_list_prim_u_16 {
    fn wire2api(self) -> Vec<u16> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u32>> for *mut wire_list_prim_u_32 {
    fn wire2api(self) -> Vec<u32> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u64>> for *mut wire_list_prim_u_64 {
    fn wire2api(self) -> Vec<u64> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<(String, i32)>> for *mut wire_list_record_string_i_32 {
    fn wire2api(self) -> Vec<(String, i32)> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::array::TestId>> for *mut wire_list_test_id {
    fn wire2api(self) -> Vec<crate::api::array::TestId> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::misc_example::Weekdays>> for *mut wire_list_weekdays {
    fn wire2api(self) -> Vec<crate::api::misc_example::Weekdays> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<crate::api::inside_macro::MacroStruct> for wire_macro_struct {
    fn wire2api(self) -> crate::api::inside_macro::MacroStruct {
        crate::api::inside_macro::MacroStruct {
            data: self.data.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::Measure> for wire_measure {
    fn wire2api(self) -> crate::api::enumeration::Measure {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Speed);
                crate::api::enumeration::Measure::Speed(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Distance);
                crate::api::enumeration::Measure::Distance(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::array::MessageId> for wire_message_id {
    fn wire2api(self) -> crate::api::array::MessageId {
        crate::api::array::MessageId(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::misc_example::MyNestedStruct> for wire_my_nested_struct {
    fn wire2api(self) -> crate::api::misc_example::MyNestedStruct {
        crate::api::misc_example::MyNestedStruct {
            tree_node: self.tree_node.wire2api(),
            weekday: self.weekday.wire2api(),
        }
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MySize> for wire_my_size {
    fn wire2api(self) -> crate::auxiliary::sample_types::MySize {
        crate::auxiliary::sample_types::MySize {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MyStruct> for wire_my_struct {
    fn wire2api(self) -> crate::auxiliary::sample_types::MyStruct {
        crate::auxiliary::sample_types::MyStruct {
            content: self.content.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::MyTreeNode> for wire_my_tree_node {
    fn wire2api(self) -> crate::api::misc_example::MyTreeNode {
        crate::api::misc_example::MyTreeNode {
            value_i32: self.value_i32.wire2api(),
            value_vec_u8: self.value_vec_u8.wire2api(),
            value_boolean: self.value_boolean.wire2api(),
            children: self.children.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::newtype_pattern::NewTypeInt> for wire_new_type_int {
    fn wire2api(self) -> crate::api::newtype_pattern::NewTypeInt {
        crate::api::newtype_pattern::NewTypeInt(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::enumeration::Note> for wire_note {
    fn wire2api(self) -> crate::api::enumeration::Note {
        crate::api::enumeration::Note {
            day: self.day.wire2api(),
            body: self.body.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::mirror::Numbers> for wire_numbers {
    fn wire2api(self) -> crate::api::mirror::Numbers {
        crate::api::mirror::Numbers(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::rust_opaque::OpaqueNested> for wire_opaque_nested {
    fn wire2api(self) -> crate::api::rust_opaque::OpaqueNested {
        crate::api::rust_opaque::OpaqueNested {
            first: self.first.wire2api(),
            second: self.second.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::OptVecs> for wire_opt_vecs {
    fn wire2api(self) -> crate::api::optional::OptVecs {
        crate::api::optional::OptVecs {
            i32: self.i32.wire2api(),
            enums: self.enums.wire2api(),
            strings: self.strings.wire2api(),
            buffers: self.buffers.wire2api(),
        }
    }
}
impl Wire2Api<(String, i32)> for wire_record_string_i_32 {
    fn wire2api(self) -> (String, i32) {
        (self.field0.wire2api(), self.field1.wire2api())
    }
}
impl Wire2Api<crate::api::mirror::Sequences> for wire_sequences {
    fn wire2api(self) -> crate::api::mirror::Sequences {
        crate::api::mirror::Sequences(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::enumeration::Speed> for wire_speed {
    fn wire2api(self) -> crate::api::enumeration::Speed {
        match self.tag {
            0 => crate::api::enumeration::Speed::Unknown,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.GPS);
                crate::api::enumeration::Speed::GPS(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::comment::StructWithCommentsTwinNormal>
    for wire_struct_with_comments_twin_normal
{
    fn wire2api(self) -> crate::api::comment::StructWithCommentsTwinNormal {
        crate::api::comment::StructWithCommentsTwinNormal {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync>
    for wire_struct_with_comments_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
        crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::StructWithEnum> for wire_struct_with_enum {
    fn wire2api(self) -> crate::api::misc_example::StructWithEnum {
        crate::api::misc_example::StructWithEnum {
            abc1: self.abc1.wire2api(),
            abc2: self.abc2.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithOneFieldTwinNormal>
    for wire_struct_with_one_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithOneFieldTwinNormal {
        crate::api::structure::StructWithOneFieldTwinNormal {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync>
    for wire_struct_with_one_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithTwoFieldTwinNormal>
    for wire_struct_with_two_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithTwoFieldTwinNormal {
        crate::api::structure::StructWithTwoFieldTwinNormal {
            a: self.a.wire2api(),
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync>
    for wire_struct_with_two_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
            a: self.a.wire2api(),
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithZeroFieldTwinNormal>
    for wire_struct_with_zero_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithZeroFieldTwinNormal {
        crate::api::structure::StructWithZeroFieldTwinNormal {}
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync>
    for wire_struct_with_zero_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {}
    }
}
impl Wire2Api<crate::api::method::SumWith> for wire_sum_with {
    fn wire2api(self) -> crate::api::method::SumWith {
        crate::api::method::SumWith {
            x: self.x.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::TestId> for wire_test_id {
    fn wire2api(self) -> crate::api::array::TestId {
        crate::api::array::TestId(self.field0.wire2api())
    }
}
impl Wire2Api<[crate::api::array::TestId; 4]> for *mut wire_list_test_id {
    fn wire2api(self) -> [crate::api::array::TestId; 4] {
        let vec: Vec<crate::api::array::TestId> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithOneFieldTwinNormal>
    for wire_tuple_struct_with_one_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithOneFieldTwinNormal {
        crate::api::structure::TupleStructWithOneFieldTwinNormal(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync>
    for wire_tuple_struct_with_one_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync(
            self.field0.wire2api(),
        )
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithTwoFieldTwinNormal>
    for wire_tuple_struct_with_two_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithTwoFieldTwinNormal {
        crate::api::structure::TupleStructWithTwoFieldTwinNormal(
            self.field0.wire2api(),
            self.field1.wire2api(),
        )
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync>
    for wire_tuple_struct_with_two_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync(
            self.field0.wire2api(),
            self.field1.wire2api(),
        )
    }
}
impl Wire2Api<[u8; 1600]> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::attribute::UserId> for wire_user_id {
    fn wire2api(self) -> crate::api::attribute::UserId {
        crate::api::attribute::UserId {
            value: self.value.wire2api(),
        }
    }
}

// Section: wire2api_class

#[repr(C)]
#[derive(Clone)]
pub struct wire_DartOpaque {
    port: i64,
    handle: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_MutexHideData {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_RwLockHideData {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_box_dynDartDebug {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_hide_data {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_i_32 {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_non_clone_data {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_non_send_hide_data {
    ptr: *const core::ffi::c_void,
}

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
pub struct wire_application_env {
    vars: *mut wire_list_application_env_var,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_application_env_var {
    field0: *mut wire_list_prim_u_8,
    field1: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_application_settings {
    name: *mut wire_list_prim_u_8,
    version: *mut wire_list_prim_u_8,
    mode: i32,
    env: *mut wire_application_env,
    env_optional: *mut wire_application_env,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_attribute {
    key: *mut wire_list_prim_u_8,
    value: *mut wire_list_prim_u_8,
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
pub struct wire_concatenate_with {
    a: *mut wire_list_prim_u_8,
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
pub struct wire_customized {
    final_field: *mut wire_list_prim_u_8,
    non_final_field: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_dart_opaque_nested {
    first: wire_DartOpaque,
    second: wire_DartOpaque,
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
pub struct wire_enum_dart_opaque {
    tag: i32,
    kind: *mut EnumDartOpaqueKind,
}

#[repr(C)]
pub union EnumDartOpaqueKind {
    Primitive: *mut wire_EnumDartOpaque_Primitive,
    Opaque: *mut wire_EnumDartOpaque_Opaque,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDartOpaque_Primitive {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDartOpaque_Opaque {
    field0: wire_DartOpaque,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_opaque {
    tag: i32,
    kind: *mut EnumOpaqueKind,
}

#[repr(C)]
pub union EnumOpaqueKind {
    Struct: *mut wire_EnumOpaque_Struct,
    Primitive: *mut wire_EnumOpaque_Primitive,
    TraitObj: *mut wire_EnumOpaque_TraitObj,
    Mutex: *mut wire_EnumOpaque_Mutex,
    RwLock: *mut wire_EnumOpaque_RwLock,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_Struct {
    field0: wire_RustOpaque_hide_data,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_Primitive {
    field0: wire_RustOpaque_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_TraitObj {
    field0: wire_RustOpaque_box_dynDartDebug,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_Mutex {
    field0: wire_RustOpaque_MutexHideData,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaque_RwLock {
    field0: wire_RustOpaque_RwLockHideData,
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
pub struct wire_event {
    address: *mut wire_list_prim_u_8,
    payload: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_exotic_optionals {
    int32: *mut i32,
    int64: *mut i64,
    float64: *mut f64,
    boolean: *mut bool,
    zerocopy: *mut wire_list_prim_u_8,
    int8list: *mut wire_list_prim_i_8,
    uint8list: *mut wire_list_prim_u_8,
    int32list: *mut wire_list_prim_i_32,
    float32list: *mut wire_list_prim_f_32,
    float64list: *mut wire_list_prim_f_64,
    attributes: *mut wire_list_attribute,
    attributes_nullable: *mut wire_list_opt_box_autoadd_attribute,
    nullable_attributes: *mut wire_list_opt_box_autoadd_attribute,
    newtypeint: *mut wire_new_type_int,
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
pub struct wire_list_DartOpaque {
    ptr: *mut wire_DartOpaque,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_RustOpaque_hide_data {
    ptr: *mut wire_RustOpaque_hide_data,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_application_env_var {
    ptr: *mut wire_application_env_var,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_attribute {
    ptr: *mut wire_attribute,
    len: i32,
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
pub struct wire_list_opt_String {
    ptr: *mut *mut wire_list_prim_u_8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_attribute {
    ptr: *mut *mut wire_attribute,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_i_32 {
    ptr: *mut *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_weekdays {
    ptr: *mut *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_list_prim_i_32 {
    ptr: *mut *mut wire_list_prim_i_32,
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
pub struct wire_my_struct {
    content: bool,
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
pub struct wire_numbers {
    field0: *mut wire_list_prim_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_opaque_nested {
    first: wire_RustOpaque_hide_data,
    second: wire_RustOpaque_hide_data,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_opt_vecs {
    i32: *mut wire_list_opt_box_autoadd_i_32,
    enums: *mut wire_list_opt_box_autoadd_weekdays,
    strings: *mut wire_list_opt_String,
    buffers: *mut wire_list_opt_list_prim_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_record_string_i_32 {
    field0: *mut wire_list_prim_u_8,
    field1: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_sequences {
    field0: *mut wire_list_prim_i_32,
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
pub struct wire_sum_with {
    x: u32,
}

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

#[repr(C)]
#[derive(Clone)]
pub struct wire_user_id {
    value: u32,
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
impl NewWithNullPtr for wire_DartOpaque {
    fn new_with_null_ptr() -> Self {
        Self { port: 0, handle: 0 }
    }
}
impl NewWithNullPtr for wire_RustOpaque_MutexHideData {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_RwLockHideData {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_box_dynDartDebug {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_hide_data {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_i_32 {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_non_clone_data {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_non_send_hide_data {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
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
impl NewWithNullPtr for wire_application_env {
    fn new_with_null_ptr() -> Self {
        Self {
            vars: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_application_env {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_application_env_var {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }
    }
}
impl Default for wire_application_env_var {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_application_settings {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            version: core::ptr::null_mut(),
            mode: Default::default(),
            env: core::ptr::null_mut(),
            env_optional: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_application_settings {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_attribute {
    fn new_with_null_ptr() -> Self {
        Self {
            key: core::ptr::null_mut(),
            value: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_attribute {
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
impl NewWithNullPtr for wire_concatenate_with {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_concatenate_with {
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
impl NewWithNullPtr for wire_customized {
    fn new_with_null_ptr() -> Self {
        Self {
            final_field: core::ptr::null_mut(),
            non_final_field: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_customized {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_dart_opaque_nested {
    fn new_with_null_ptr() -> Self {
        Self {
            first: wire_DartOpaque::new_with_null_ptr(),
            second: wire_DartOpaque::new_with_null_ptr(),
        }
    }
}
impl Default for wire_dart_opaque_nested {
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
impl NewWithNullPtr for wire_enum_dart_opaque {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_dart_opaque {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_opaque {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_opaque {
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
impl NewWithNullPtr for wire_event {
    fn new_with_null_ptr() -> Self {
        Self {
            address: core::ptr::null_mut(),
            payload: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_event {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_exotic_optionals {
    fn new_with_null_ptr() -> Self {
        Self {
            int32: core::ptr::null_mut(),
            int64: core::ptr::null_mut(),
            float64: core::ptr::null_mut(),
            boolean: core::ptr::null_mut(),
            zerocopy: core::ptr::null_mut(),
            int8list: core::ptr::null_mut(),
            uint8list: core::ptr::null_mut(),
            int32list: core::ptr::null_mut(),
            float32list: core::ptr::null_mut(),
            float64list: core::ptr::null_mut(),
            attributes: core::ptr::null_mut(),
            attributes_nullable: core::ptr::null_mut(),
            nullable_attributes: core::ptr::null_mut(),
            newtypeint: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_exotic_optionals {
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
impl NewWithNullPtr for wire_my_struct {
    fn new_with_null_ptr() -> Self {
        Self {
            content: Default::default(),
        }
    }
}
impl Default for wire_my_struct {
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
impl NewWithNullPtr for wire_numbers {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_numbers {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_opaque_nested {
    fn new_with_null_ptr() -> Self {
        Self {
            first: wire_RustOpaque_hide_data::new_with_null_ptr(),
            second: wire_RustOpaque_hide_data::new_with_null_ptr(),
        }
    }
}
impl Default for wire_opaque_nested {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_opt_vecs {
    fn new_with_null_ptr() -> Self {
        Self {
            i32: core::ptr::null_mut(),
            enums: core::ptr::null_mut(),
            strings: core::ptr::null_mut(),
            buffers: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_opt_vecs {
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
impl NewWithNullPtr for wire_sequences {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_sequences {
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
impl NewWithNullPtr for wire_sum_with {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
        }
    }
}
impl Default for wire_sum_with {
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
impl NewWithNullPtr for wire_user_id {
    fn new_with_null_ptr() -> Self {
        Self {
            value: Default::default(),
        }
    }
}
impl Default for wire_user_id {
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
pub extern "C" fn wire_handle_customized_struct(port_: i64, val: *mut wire_customized) {
    wire_handle_customized_struct_impl(port_, val)
}

#[no_mangle]
pub extern "C" fn wire_next_user_id(port_: i64, user_id: *mut wire_user_id) {
    wire_next_user_id_impl(port_, user_id)
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
pub extern "C" fn wire_async_accept_dart_opaque(port_: i64, opaque: wire_DartOpaque) {
    wire_async_accept_dart_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_create_enum_dart_opaque(port_: i64, opaque: wire_DartOpaque) {
    wire_create_enum_dart_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_create_nested_dart_opaque(
    port_: i64,
    opaque1: wire_DartOpaque,
    opaque2: wire_DartOpaque,
) {
    wire_create_nested_dart_opaque_impl(port_, opaque1, opaque2)
}

#[no_mangle]
pub extern "C" fn wire_drop_static_dart_opaque(port_: i64) {
    wire_drop_static_dart_opaque_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_enum_dart_opaque(port_: i64, opaque: *mut wire_enum_dart_opaque) {
    wire_get_enum_dart_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_get_nested_dart_opaque(port_: i64, opaque: *mut wire_dart_opaque_nested) {
    wire_get_nested_dart_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back(port_: i64, opaque: wire_DartOpaque) {
    wire_loop_back_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_array(port_: i64, opaque: wire_DartOpaque) {
    wire_loop_back_array_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_array_get(port_: i64, opaque: *mut wire_list_DartOpaque) {
    wire_loop_back_array_get_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_option(port_: i64, opaque: wire_DartOpaque) {
    wire_loop_back_option_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_option_get(port_: i64, opaque: *mut wire_DartOpaque) {
    wire_loop_back_option_get_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_vec(port_: i64, opaque: wire_DartOpaque) {
    wire_loop_back_vec_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_vec_get(port_: i64, opaque: *mut wire_list_DartOpaque) {
    wire_loop_back_vec_get_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_panic_unwrap_dart_opaque(port_: i64, opaque: wire_DartOpaque) {
    wire_panic_unwrap_dart_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_set_static_dart_opaque(port_: i64, opaque: wire_DartOpaque) {
    wire_set_static_dart_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_return_non_droppable_dart_opaque(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_non_droppable_dart_opaque_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_unwrap_dart_opaque(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_unwrap_dart_opaque_impl(opaque)
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
pub extern "C" fn wire_Event_as_string(port_: i64, that: *mut wire_event) {
    wire_Event_as_string_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_close_event_listener(port_: i64) {
    wire_close_event_listener_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_event(
    port_: i64,
    address: *mut wire_list_prim_u_8,
    payload: *mut wire_list_prim_u_8,
) {
    wire_create_event_impl(port_, address, payload)
}

#[no_mangle]
pub extern "C" fn wire_register_event_listener(port_: i64) {
    wire_register_event_listener_impl(port_)
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
pub extern "C" fn wire_call_new_module_system(port_: i64) {
    wire_call_new_module_system_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_call_old_module_system(port_: i64) {
    wire_call_old_module_system_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_use_imported_enum(port_: i64, my_enum: i32) {
    wire_use_imported_enum_impl(port_, my_enum)
}

#[no_mangle]
pub extern "C" fn wire_use_imported_struct(port_: i64, my_struct: *mut wire_my_struct) {
    wire_use_imported_struct_impl(port_, my_struct)
}

#[no_mangle]
pub extern "C" fn wire_func_macro_struct(port_: i64, arg: *mut wire_macro_struct) {
    wire_func_macro_struct_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWith_concatenate(
    port_: i64,
    that: *mut wire_concatenate_with,
    b: *mut wire_list_prim_u_8,
) {
    wire_ConcatenateWith_concatenate_impl(port_, that, b)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWith_concatenate_static(
    port_: i64,
    a: *mut wire_list_prim_u_8,
    b: *mut wire_list_prim_u_8,
) {
    wire_ConcatenateWith_concatenate_static_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWith_handle_some_static_stream_sink(
    port_: i64,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWith_handle_some_static_stream_sink_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWith_handle_some_static_stream_sink_single_arg(port_: i64) {
    wire_ConcatenateWith_handle_some_static_stream_sink_single_arg_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWith_handle_some_stream_sink(
    port_: i64,
    that: *mut wire_concatenate_with,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWith_handle_some_stream_sink_impl(port_, that, key, max)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWith_handle_some_stream_sink_at_1(
    port_: i64,
    that: *mut wire_concatenate_with,
) {
    wire_ConcatenateWith_handle_some_stream_sink_at_1_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWith_new(port_: i64, a: *mut wire_list_prim_u_8) {
    wire_ConcatenateWith_new_impl(port_, a)
}

#[no_mangle]
pub extern "C" fn wire_SumWith_sum(port_: i64, that: *mut wire_sum_with, y: u32, z: u32) {
    wire_SumWith_sum_impl(port_, that, y, z)
}

#[no_mangle]
pub extern "C" fn wire_get_sum_array(port_: i64, a: u32, b: u32, c: u32) {
    wire_get_sum_array_impl(port_, a, b, c)
}

#[no_mangle]
pub extern "C" fn wire_get_sum_struct(port_: i64) {
    wire_get_sum_struct_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_app_settings_stream(port_: i64) {
    wire_app_settings_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_app_settings_vec_stream(port_: i64) {
    wire_app_settings_vec_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_first_number(port_: i64, nums: *mut wire_numbers) {
    wire_first_number_impl(port_, nums)
}

#[no_mangle]
pub extern "C" fn wire_first_sequence(port_: i64, seqs: *mut wire_sequences) {
    wire_first_sequence_impl(port_, seqs)
}

#[no_mangle]
pub extern "C" fn wire_get_app_settings(port_: i64) {
    wire_get_app_settings_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_fallible_app_settings(port_: i64) {
    wire_get_fallible_app_settings_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_message(port_: i64) {
    wire_get_message_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_app_embedded(port_: i64, app_settings: *mut wire_application_settings) {
    wire_is_app_embedded_impl(port_, app_settings)
}

#[no_mangle]
pub extern "C" fn wire_mirror_struct_stream(port_: i64) {
    wire_mirror_struct_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_mirror_tuple_stream(port_: i64) {
    wire_mirror_tuple_stream_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_repeat_number(port_: i64, num: i32, times: usize) {
    wire_repeat_number_impl(port_, num, times)
}

#[no_mangle]
pub extern "C" fn wire_repeat_sequence(port_: i64, seq: i32, times: usize) {
    wire_repeat_sequence_impl(port_, seq, times)
}

#[no_mangle]
pub extern "C" fn wire_test_contains_mirrored_sub_struct(port_: i64) {
    wire_test_contains_mirrored_sub_struct_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_fallible_of_raw_string_mirrored(port_: i64) {
    wire_test_fallible_of_raw_string_mirrored_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_list_of_nested_enums_mirrored(port_: i64) {
    wire_test_list_of_nested_enums_mirrored_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_list_of_raw_nested_string_mirrored(port_: i64) {
    wire_test_list_of_raw_nested_string_mirrored_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_nested_raw_string_mirrored(port_: i64) {
    wire_test_nested_raw_string_mirrored_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_raw_string_enum_mirrored(port_: i64, nested: bool) {
    wire_test_raw_string_enum_mirrored_impl(port_, nested)
}

#[no_mangle]
pub extern "C" fn wire_test_raw_string_mirrored(port_: i64) {
    wire_test_raw_string_mirrored_impl(port_)
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
pub extern "C" fn wire_handle_increment_boxed_optional(port_: i64, opt: *mut f64) {
    wire_handle_increment_boxed_optional_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_option_box_arguments(
    port_: i64,
    i8box: *mut i8,
    u8box: *mut u8,
    i32box: *mut i32,
    i64box: *mut i64,
    f64box: *mut f64,
    boolbox: *mut bool,
    structbox: *mut wire_exotic_optionals,
) {
    wire_handle_option_box_arguments_impl(
        port_, i8box, u8box, i32box, i64box, f64box, boolbox, structbox,
    )
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_increment(port_: i64, opt: *mut wire_exotic_optionals) {
    wire_handle_optional_increment_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_return(port_: i64, left: f64, right: f64) {
    wire_handle_optional_return_impl(port_, left, right)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_struct(port_: i64, document: *mut wire_list_prim_u_8) {
    wire_handle_optional_struct_impl(port_, document)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_of_opts(port_: i64, opt: *mut wire_opt_vecs) {
    wire_handle_vec_of_opts_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    that: *mut wire_struct_with_comments_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_instance_method_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_static_method_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_static_method_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_slash_star_star_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_slash_star_star_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_triple_slash_multi_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_triple_slash_single_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_enum_simple_twin_sync(
    arg: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_simple_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_mixed_twin_sync(
    arg: *mut wire_enum_with_item_mixed_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_mixed_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_struct_twin_sync(
    arg: *mut wire_enum_with_item_struct_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_struct_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_tuple_twin_sync(
    arg: *mut wire_enum_with_item_tuple_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_tuple_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_panic_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_panic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_return_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_ok_twin_sync(
    arg: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_return_ok_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_nested_error_return_error_twin_sync(
    arg: *mut wire_custom_nested_error_outer_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_nested_error_return_error_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_struct_error_return_error_twin_sync(
    arg: *mut wire_custom_struct_error_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_struct_error_return_error_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_return_error_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_func_return_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_type_fallible_panic_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_type_fallible_panic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_type_infallible_panic_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_type_infallible_panic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_return_unit_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_func_return_unit_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_string_twin_sync(
    arg: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
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
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32_twin_sync(
    arg: *mut f32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64_twin_sync(
    arg: *mut f64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16_twin_sync(
    arg: *mut i16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32_twin_sync(
    arg: *mut i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64_twin_sync(
    arg: *mut i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8_twin_sync(
    arg: *mut i8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16_twin_sync(
    arg: *mut u16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32_twin_sync(
    arg: *mut u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64_twin_sync(
    arg: *mut u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8_twin_sync(
    arg: *mut u8,
) -> flutter_rust_bridge::support::WireSyncReturn {
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
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f32_twin_sync(
    arg: *mut wire_list_prim_f_32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f64_twin_sync(
    arg: *mut wire_list_prim_f_64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i16_twin_sync(
    arg: *mut wire_list_prim_i_16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i32_twin_sync(
    arg: *mut wire_list_prim_i_32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i64_twin_sync(
    arg: *mut wire_list_prim_i_64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i8_twin_sync(
    arg: *mut wire_list_prim_i_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u16_twin_sync(
    arg: *mut wire_list_prim_u_16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u32_twin_sync(
    arg: *mut wire_list_prim_u_32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u64_twin_sync(
    arg: *mut wire_list_prim_u_64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u8_twin_sync(
    arg: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_bool_twin_sync(
    arg: bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32_twin_sync(
    arg: f32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64_twin_sync(
    arg: f64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16_twin_sync(
    arg: i16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32_twin_sync(
    arg: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64_twin_sync(
    arg: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8_twin_sync(
    arg: i8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16_twin_sync(
    arg: u16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32_twin_sync(
    arg: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64_twin_sync(
    arg: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8_twin_sync(
    arg: u8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_twin_sync(
    a: i32,
    b: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_simple_adder_twin_sync_impl(a, b)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_one_field_twin_sync(
    arg: *mut wire_struct_with_one_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_one_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_two_field_twin_sync(
    arg: *mut wire_struct_with_two_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_two_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_zero_field_twin_sync(
    arg: *mut wire_struct_with_zero_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_zero_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_one_field_twin_sync(
    arg: *mut wire_tuple_struct_with_one_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_tuple_struct_with_one_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_two_field_twin_sync(
    arg: *mut wire_tuple_struct_with_two_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
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
pub extern "C" fn wire_create_array_opaque_enum(port_: i64) {
    wire_create_array_opaque_enum_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_nested_opaque(port_: i64) {
    wire_create_nested_opaque_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_opaque(port_: i64) {
    wire_create_opaque_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_option_opaque(port_: i64, opaque: *mut wire_RustOpaque_hide_data) {
    wire_create_option_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_create_sync_opaque(port_: i64) {
    wire_create_sync_opaque_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_frb_generator_test(port_: i64) {
    wire_frb_generator_test_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_opaque_array(port_: i64) {
    wire_opaque_array_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_opaque_array_run(port_: i64, data: *mut wire_list_RustOpaque_hide_data) {
    wire_opaque_array_run_impl(port_, data)
}

#[no_mangle]
pub extern "C" fn wire_opaque_vec(port_: i64) {
    wire_opaque_vec_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_opaque_vec_run(port_: i64, data: *mut wire_list_RustOpaque_hide_data) {
    wire_opaque_vec_run_impl(port_, data)
}

#[no_mangle]
pub extern "C" fn wire_run_enum_opaque(port_: i64, opaque: *mut wire_enum_opaque) {
    wire_run_enum_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_nested_opaque(port_: i64, opaque: *mut wire_opaque_nested) {
    wire_run_nested_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_non_clone(port_: i64, clone: wire_RustOpaque_non_clone_data) {
    wire_run_non_clone_impl(port_, clone)
}

#[no_mangle]
pub extern "C" fn wire_run_opaque(port_: i64, opaque: wire_RustOpaque_hide_data) {
    wire_run_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_opaque_with_delay(port_: i64, opaque: wire_RustOpaque_hide_data) {
    wire_run_opaque_with_delay_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_unwrap_rust_opaque(port_: i64, opaque: wire_RustOpaque_hide_data) {
    wire_unwrap_rust_opaque_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_frb_sync_generator_test(port_: i64) {
    wire_frb_sync_generator_test_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_sync_run_opaque(
    opaque: wire_RustOpaque_non_send_hide_data,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_run_opaque_impl(opaque)
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
pub extern "C" fn wire_handle_type_alias_id(port_: i64, input: u64) {
    wire_handle_type_alias_id_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_type_alias_model(port_: i64, input: u64) {
    wire_handle_type_alias_model_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_type_nest_alias_id(port_: i64, input: u64) {
    wire_handle_type_nest_alias_id_impl(port_, input)
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
pub extern "C" fn new_DartOpaque() -> wire_DartOpaque {
    wire_DartOpaque::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_MutexHideData() -> wire_RustOpaque_MutexHideData {
    wire_RustOpaque_MutexHideData::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_RwLockHideData() -> wire_RustOpaque_RwLockHideData {
    wire_RustOpaque_RwLockHideData::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_box_dynDartDebug() -> wire_RustOpaque_box_dynDartDebug {
    wire_RustOpaque_box_dynDartDebug::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_hide_data() -> wire_RustOpaque_hide_data {
    wire_RustOpaque_hide_data::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_i_32() -> wire_RustOpaque_i_32 {
    wire_RustOpaque_i_32::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_non_clone_data() -> wire_RustOpaque_non_clone_data {
    wire_RustOpaque_non_clone_data::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_non_send_hide_data() -> wire_RustOpaque_non_send_hide_data {
    wire_RustOpaque_non_send_hide_data::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_StringList(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <*mut wire_list_prim_u_8>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_application_env() -> *mut wire_application_env {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_application_env::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_Chrono_Utc(value: i64) -> *mut i64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_DartOpaque() -> *mut wire_DartOpaque {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_DartOpaque::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_RustOpaque_hide_data() -> *mut wire_RustOpaque_hide_data {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_RustOpaque_hide_data::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_a() -> *mut wire_a {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_a::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_abc() -> *mut wire_abc {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_abc::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_env() -> *mut wire_application_env {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_application_env::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_settings() -> *mut wire_application_settings {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_application_settings::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_attribute() -> *mut wire_attribute {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_attribute::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_b() -> *mut wire_b {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_b::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool(value: bool) -> *mut bool {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_c() -> *mut wire_c {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_c::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_concatenate_with() -> *mut wire_concatenate_with {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_concatenate_with::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_inner_twin_normal(
) -> *mut wire_custom_nested_error_inner_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_nested_error_inner_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_inner_twin_sync(
) -> *mut wire_custom_nested_error_inner_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_nested_error_inner_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_outer_twin_normal(
) -> *mut wire_custom_nested_error_outer_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_nested_error_outer_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_outer_twin_sync(
) -> *mut wire_custom_nested_error_outer_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_nested_error_outer_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_struct_error_twin_normal(
) -> *mut wire_custom_struct_error_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_struct_error_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_struct_error_twin_sync(
) -> *mut wire_custom_struct_error_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_struct_error_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_customized() -> *mut wire_customized {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_customized::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_dart_opaque_nested() -> *mut wire_dart_opaque_nested {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_dart_opaque_nested::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_dart_opaque() -> *mut wire_enum_dart_opaque {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_enum_dart_opaque::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_opaque() -> *mut wire_enum_opaque {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_enum_opaque::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_mixed_twin_normal(
) -> *mut wire_enum_with_item_mixed_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_mixed_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_mixed_twin_sync(
) -> *mut wire_enum_with_item_mixed_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_mixed_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_struct_twin_normal(
) -> *mut wire_enum_with_item_struct_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_struct_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_struct_twin_sync(
) -> *mut wire_enum_with_item_struct_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_struct_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_tuple_twin_normal(
) -> *mut wire_enum_with_item_tuple_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_tuple_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_tuple_twin_sync(
) -> *mut wire_enum_with_item_tuple_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_tuple_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_event() -> *mut wire_event {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_event::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_exotic_optionals() -> *mut wire_exotic_optionals {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_exotic_optionals::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_32(value: f32) -> *mut f32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_64(value: f64) -> *mut f64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feature_chrono() -> *mut wire_feature_chrono {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_feature_chrono::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feature_uuid() -> *mut wire_feature_uuid {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_feature_uuid::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feed_id() -> *mut wire_feed_id {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_feed_id::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_16(value: i16) -> *mut i16 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_32(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_64(value: i64) -> *mut i64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_8(value: i8) -> *mut i8 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_macro_struct() -> *mut wire_macro_struct {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_macro_struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_measure() -> *mut wire_measure {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_measure::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_message_id() -> *mut wire_message_id {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_message_id::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_nested_struct() -> *mut wire_my_nested_struct {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_my_nested_struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_struct() -> *mut wire_my_struct {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_my_struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_tree_node() -> *mut wire_my_tree_node {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_my_tree_node::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_new_type_int() -> *mut wire_new_type_int {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_new_type_int::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_note() -> *mut wire_note {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_note::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_numbers() -> *mut wire_numbers {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_numbers::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_opaque_nested() -> *mut wire_opaque_nested {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_opaque_nested::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_opt_vecs() -> *mut wire_opt_vecs {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_opt_vecs::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_record_string_i_32() -> *mut wire_record_string_i_32 {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_record_string_i_32::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sequences() -> *mut wire_sequences {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_sequences::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_normal(
) -> *mut wire_struct_with_comments_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_comments_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_sync(
) -> *mut wire_struct_with_comments_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_comments_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_enum() -> *mut wire_struct_with_enum {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_struct_with_enum::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_one_field_twin_normal(
) -> *mut wire_struct_with_one_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_one_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_one_field_twin_sync(
) -> *mut wire_struct_with_one_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_one_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_two_field_twin_normal(
) -> *mut wire_struct_with_two_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_two_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_two_field_twin_sync(
) -> *mut wire_struct_with_two_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_two_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_zero_field_twin_normal(
) -> *mut wire_struct_with_zero_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_zero_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_zero_field_twin_sync(
) -> *mut wire_struct_with_zero_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_zero_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sum_with() -> *mut wire_sum_with {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_sum_with::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_test_id() -> *mut wire_test_id {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_test_id::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_one_field_twin_normal(
) -> *mut wire_tuple_struct_with_one_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_tuple_struct_with_one_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_one_field_twin_sync(
) -> *mut wire_tuple_struct_with_one_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_tuple_struct_with_one_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_two_field_twin_normal(
) -> *mut wire_tuple_struct_with_two_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_tuple_struct_with_two_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_two_field_twin_sync(
) -> *mut wire_tuple_struct_with_two_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_tuple_struct_with_two_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_16(value: u16) -> *mut u16 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_32(value: u32) -> *mut u32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_64(value: u64) -> *mut u64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_8(value: u8) -> *mut u8 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_user_id() -> *mut wire_user_id {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_user_id::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_weekdays(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_blob() -> *mut wire_blob {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_blob::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_bool(value: bool) -> *mut bool {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_distance() -> *mut wire_distance {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_distance::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_exotic_optionals() -> *mut wire_exotic_optionals {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_exotic_optionals::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_f_64(value: f64) -> *mut f64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i_32(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i_64(value: i64) -> *mut i64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i_8(value: i8) -> *mut i8 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_speed() -> *mut wire_speed {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_speed::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_u_8(value: u8) -> *mut u8 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_weekdays(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_list_DartOpaque(len: i32) -> *mut wire_list_DartOpaque {
    let wrap = wire_list_DartOpaque {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_DartOpaque>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_RustOpaque_hide_data(len: i32) -> *mut wire_list_RustOpaque_hide_data {
    let wrap = wire_list_RustOpaque_hide_data {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_RustOpaque_hide_data>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_application_env_var(len: i32) -> *mut wire_list_application_env_var {
    let wrap = wire_list_application_env_var {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_application_env_var>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_attribute(len: i32) -> *mut wire_list_attribute {
    let wrap = wire_list_attribute {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_attribute>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_bool(len: i32) -> *mut wire_list_bool {
    let wrap = wire_list_bool {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_size(len: i32) -> *mut wire_list_my_size {
    let wrap = wire_list_my_size {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_my_size>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_tree_node(len: i32) -> *mut wire_list_my_tree_node {
    let wrap = wire_list_my_tree_node {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_my_tree_node>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_String(len: i32) -> *mut wire_list_opt_String {
    let wrap = wire_list_opt_String {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_attribute(
    len: i32,
) -> *mut wire_list_opt_box_autoadd_attribute {
    let wrap = wire_list_opt_box_autoadd_attribute {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_i_32(len: i32) -> *mut wire_list_opt_box_autoadd_i_32 {
    let wrap = wire_list_opt_box_autoadd_i_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_weekdays(
    len: i32,
) -> *mut wire_list_opt_box_autoadd_weekdays {
    let wrap = wire_list_opt_box_autoadd_weekdays {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_list_prim_i_32(len: i32) -> *mut wire_list_opt_list_prim_i_32 {
    let wrap = wire_list_opt_list_prim_i_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_32(len: i32) -> *mut wire_list_prim_f_32 {
    let ans = wire_list_prim_f_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_64(len: i32) -> *mut wire_list_prim_f_64 {
    let ans = wire_list_prim_f_64 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_16(len: i32) -> *mut wire_list_prim_i_16 {
    let ans = wire_list_prim_i_16 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_32(len: i32) -> *mut wire_list_prim_i_32 {
    let ans = wire_list_prim_i_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_64(len: i32) -> *mut wire_list_prim_i_64 {
    let ans = wire_list_prim_i_64 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_8(len: i32) -> *mut wire_list_prim_i_8 {
    let ans = wire_list_prim_i_8 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_16(len: i32) -> *mut wire_list_prim_u_16 {
    let ans = wire_list_prim_u_16 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_32(len: i32) -> *mut wire_list_prim_u_32 {
    let ans = wire_list_prim_u_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_64(len: i32) -> *mut wire_list_prim_u_64 {
    let ans = wire_list_prim_u_64 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_8(len: i32) -> *mut wire_list_prim_u_8 {
    let ans = wire_list_prim_u_8 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_record_string_i_32(len: i32) -> *mut wire_list_record_string_i_32 {
    let wrap = wire_list_record_string_i_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_record_string_i_32>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_test_id(len: i32) -> *mut wire_list_test_id {
    let wrap = wire_list_test_id {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_test_id>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_weekdays(len: i32) -> *mut wire_list_weekdays {
    let wrap = wire_list_weekdays {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_MutexHideData(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Mutex<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_MutexHideData(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Mutex<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_RwLockHideData(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<RwLock<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_RwLockHideData(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<RwLock<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_box_dynDartDebug(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebug>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_box_dynDartDebug(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebug>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_frb_opaque_return(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueReturn>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_frb_opaque_return(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueReturn>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_frb_opaque_sync_return(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueSyncReturn>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_frb_opaque_sync_return(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueSyncReturn>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_hide_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::HideData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_hide_data(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::HideData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_i_32(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<i32>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_i_32(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<i32>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_non_send_hide_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonSendHideData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_non_send_hide_data(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonSendHideData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[no_mangle]
pub extern "C" fn inflate_Abc_A() -> *mut AbcKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_Abc_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Abc_B() -> *mut AbcKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_Abc_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Abc_C() -> *mut AbcKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcKind {
        C: flutter_rust_bridge::support::new_leak_box_ptr(wire_Abc_C {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Abc_JustInt() -> *mut AbcKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcKind {
        JustInt: flutter_rust_bridge::support::new_leak_box_ptr(wire_Abc_JustInt {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinNormal_Three(
) -> *mut CustomNestedErrorInnerTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorInnerTwinNormalKind {
        Three: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorInnerTwinNormal_Three {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinNormal_Four(
) -> *mut CustomNestedErrorInnerTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorInnerTwinNormalKind {
        Four: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorInnerTwinNormal_Four {
                field0: Default::default(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinSync_Three(
) -> *mut CustomNestedErrorInnerTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorInnerTwinSyncKind {
        Three: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorInnerTwinSync_Three {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinSync_Four(
) -> *mut CustomNestedErrorInnerTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorInnerTwinSyncKind {
        Four: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorInnerTwinSync_Four {
                field0: Default::default(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinNormal_One(
) -> *mut CustomNestedErrorOuterTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorOuterTwinNormalKind {
        One: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorOuterTwinNormal_One {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinNormal_Two(
) -> *mut CustomNestedErrorOuterTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorOuterTwinNormalKind {
        Two: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorOuterTwinNormal_Two {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinSync_One(
) -> *mut CustomNestedErrorOuterTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorOuterTwinSyncKind {
        One: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorOuterTwinSync_One {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinSync_Two(
) -> *mut CustomNestedErrorOuterTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorOuterTwinSyncKind {
        Two: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorOuterTwinSync_Two {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Distance_Map() -> *mut DistanceKind {
    flutter_rust_bridge::support::new_leak_box_ptr(DistanceKind {
        Map: flutter_rust_bridge::support::new_leak_box_ptr(wire_Distance_Map {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDartOpaque_Primitive() -> *mut EnumDartOpaqueKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumDartOpaqueKind {
        Primitive: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumDartOpaque_Primitive {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDartOpaque_Opaque() -> *mut EnumDartOpaqueKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumDartOpaqueKind {
        Opaque: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumDartOpaque_Opaque {
            field0: wire_DartOpaque::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_Struct() -> *mut EnumOpaqueKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueKind {
        Struct: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaque_Struct {
            field0: wire_RustOpaque_hide_data::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_Primitive() -> *mut EnumOpaqueKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueKind {
        Primitive: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaque_Primitive {
            field0: wire_RustOpaque_i_32::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_TraitObj() -> *mut EnumOpaqueKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueKind {
        TraitObj: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaque_TraitObj {
            field0: wire_RustOpaque_box_dynDartDebug::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_Mutex() -> *mut EnumOpaqueKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueKind {
        Mutex: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaque_Mutex {
            field0: wire_RustOpaque_MutexHideData::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaque_RwLock() -> *mut EnumOpaqueKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueKind {
        RwLock: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaque_RwLock {
            field0: wire_RustOpaque_RwLockHideData::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinNormal_B() -> *mut EnumWithItemMixedTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemMixedTwinNormalKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemMixedTwinNormal_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinNormal_C() -> *mut EnumWithItemMixedTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemMixedTwinNormalKind {
        C: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemMixedTwinNormal_C {
            c_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinSync_B() -> *mut EnumWithItemMixedTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemMixedTwinSyncKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemMixedTwinSync_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinSync_C() -> *mut EnumWithItemMixedTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemMixedTwinSyncKind {
        C: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemMixedTwinSync_C {
            c_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinNormal_A() -> *mut EnumWithItemStructTwinNormalKind
{
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemStructTwinNormalKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemStructTwinNormal_A {
            a_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinNormal_B() -> *mut EnumWithItemStructTwinNormalKind
{
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemStructTwinNormalKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemStructTwinNormal_B {
            b_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinSync_A() -> *mut EnumWithItemStructTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemStructTwinSyncKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemStructTwinSync_A {
            a_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinSync_B() -> *mut EnumWithItemStructTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemStructTwinSyncKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemStructTwinSync_B {
            b_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinNormal_A() -> *mut EnumWithItemTupleTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemTupleTwinNormalKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemTupleTwinNormal_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinNormal_B() -> *mut EnumWithItemTupleTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemTupleTwinNormalKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemTupleTwinNormal_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinSync_A() -> *mut EnumWithItemTupleTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemTupleTwinSyncKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemTupleTwinSync_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinSync_B() -> *mut EnumWithItemTupleTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemTupleTwinSyncKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemTupleTwinSync_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Measure_Speed() -> *mut MeasureKind {
    flutter_rust_bridge::support::new_leak_box_ptr(MeasureKind {
        Speed: flutter_rust_bridge::support::new_leak_box_ptr(wire_Measure_Speed {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Measure_Distance() -> *mut MeasureKind {
    flutter_rust_bridge::support::new_leak_box_ptr(MeasureKind {
        Distance: flutter_rust_bridge::support::new_leak_box_ptr(wire_Measure_Distance {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Speed_GPS() -> *mut SpeedKind {
    flutter_rust_bridge::support::new_leak_box_ptr(SpeedKind {
        GPS: flutter_rust_bridge::support::new_leak_box_ptr(wire_Speed_GPS {
            field0: Default::default(),
        }),
    })
}
