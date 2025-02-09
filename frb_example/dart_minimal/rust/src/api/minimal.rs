use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(json_serializable)]
pub enum MyEnumWithJsonSerializable {
    Apple(String),
    Orange { a: i32 },
}

impl MyEnumWithJsonSerializable {
    pub fn f(&self) {}
}

#[frb(json_serializable)]
pub struct MyStructWithJsonSerializable {
    field_one: String,
}

impl MyStructWithJsonSerializable {
    pub fn f(&self) {}
}
