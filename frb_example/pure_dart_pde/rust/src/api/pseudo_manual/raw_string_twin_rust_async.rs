// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `raw_string.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

pub struct RawStringItemStructTwinRustAsync {
    pub r#type: String,
}

pub async fn test_raw_string_item_struct_twin_rust_async() -> RawStringItemStructTwinRustAsync {
    RawStringItemStructTwinRustAsync {
        r#type: "test".to_owned(),
    }
}

pub struct MoreThanJustOneRawStringStructTwinRustAsync {
    pub regular: String,
    pub r#type: String,
    pub r#async: bool,
    pub another: String,
}

pub async fn test_more_than_just_one_raw_string_struct_twin_rust_async(
) -> MoreThanJustOneRawStringStructTwinRustAsync {
    MoreThanJustOneRawStringStructTwinRustAsync {
        regular: "regular".to_owned(),
        r#type: "type".to_owned(),
        r#async: true,
        another: "another".to_owned(),
    }
}
