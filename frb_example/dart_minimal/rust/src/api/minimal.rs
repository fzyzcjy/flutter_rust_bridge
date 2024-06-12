use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb]
pub struct StructWithFieldRenameTwinNormal {
    #[frb(name = "renamed_field")]
    pub class: i32,
}

pub fn func_for_struct_with_field_rename_twin_normal(
    arg: StructWithFieldRenameTwinNormal,
) -> StructWithFieldRenameTwinNormal {
    arg
}

pub struct StructWithDartKeywordFieldTwinNormal {
    pub class: i32,
    pub interface: i64,
}

pub fn func_for_struct_with_dart_keyword_field_twin_normal(
    arg: StructWithDartKeywordFieldTwinNormal,
) -> StructWithDartKeywordFieldTwinNormal {
    arg
}
