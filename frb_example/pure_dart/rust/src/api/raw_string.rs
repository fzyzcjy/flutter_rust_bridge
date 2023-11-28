pub struct RawStringItemStructTwinNormal {
    pub r#type: String,
}

pub fn test_raw_string_item_struct_twin_normal() -> RawStringItemStruct {
    RawStringItemStruct {
        r#type: "test".to_owned(),
    }
}

pub struct MoreThanJustOneRawStringStructTwinNormal {
    pub regular: String,
    pub r#type: String,
    pub r#async: bool,
    pub another: String,
}

pub fn test_more_than_just_one_raw_string_struct_twin_normal() -> MoreThanJustOneRawStringStruct {
    MoreThanJustOneRawStringStruct {
        regular: "regular".to_owned(),
        r#type: "type".to_owned(),
        r#async: true,
        another: "another".to_owned(),
    }
}
