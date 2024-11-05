use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub struct StructWithRawNameField {
    pub r#type: String,
}

impl StructWithRawNameField {
    #[frb(serialize)]
    pub fn dummy_function() {}
}

#[frb(serialize)]
pub fn r#for(r#type: String) {
    let _ = r#type;
}
