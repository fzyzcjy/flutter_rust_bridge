// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// Enums from a third-party crate listed in `rust_input` should be translatable, not opaque,
// even though their variant fields cannot be written with `pub`
use frb_example_pure_dart_example_third_party_lib::{
    ThirdPartyEnumWithFields, ThirdPartyNonExhaustiveStruct,
};

pub fn third_party_enum_with_fields_roundtrip_twin_normal(
    arg: ThirdPartyEnumWithFields,
) -> ThirdPartyEnumWithFields {
    arg
}

// `#[non_exhaustive]` third-party structs cannot be constructed by generated code, so they are opaque
pub fn third_party_non_exhaustive_struct_roundtrip_twin_normal(
    arg: ThirdPartyNonExhaustiveStruct,
) -> ThirdPartyNonExhaustiveStruct {
    arg
}
