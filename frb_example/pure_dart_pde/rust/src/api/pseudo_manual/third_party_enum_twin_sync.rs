// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `third_party_enum.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// Enums from a third-party crate listed in `rust_input` should be translatable, not opaque,
// even though their variant fields cannot be written with `pub`
use frb_example_pure_dart_example_third_party_lib::ThirdPartyEnumWithFields;

#[flutter_rust_bridge::frb(sync)]
pub fn third_party_enum_with_fields_roundtrip_twin_sync(
    arg: ThirdPartyEnumWithFields,
) -> ThirdPartyEnumWithFields {
    arg
}
