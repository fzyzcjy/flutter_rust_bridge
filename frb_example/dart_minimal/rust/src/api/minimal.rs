use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub enum RawStringItemEnumTwinNormal {
    Regular { regular: String },
    Raw { r#type: String },
}

pub fn test_raw_string_item_enum_twin_normal() -> RawStringItemEnumTwinNormal {
    RawStringItemEnumTwinNormal::Raw {
        r#type: "test".to_owned(),
    }
}
