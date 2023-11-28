// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `raw_string.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub struct RawStringItemStructTwinSync {
    pub r#type: String,
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_raw_string_item_struct_twin_sync() -> RawStringItemStructTwinSync {
    RawStringItemStructTwinSync {
        r#type: "test".to_owned(),
    }
}

pub struct MoreThanJustOneRawStringStructTwinSync {
    pub regular: String,
    pub r#type: String,
    pub r#async: bool,
    pub another: String,
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_more_than_just_one_raw_string_struct_twin_sync(
) -> MoreThanJustOneRawStringStructTwinSync {
    MoreThanJustOneRawStringStructTwinSync {
        regular: "regular".to_owned(),
        r#type: "type".to_owned(),
        r#async: true,
        another: "another".to_owned(),
    }
}
