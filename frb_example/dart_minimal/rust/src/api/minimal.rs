use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[flutter_rust_bridge::frb(opaque)]
pub struct TypeForIgnore {
    #[frb(ignore)]
    pub field_1: u32,
}

impl TypeForIgnore {
    pub fn new() -> Self {
        Self { field_1: 0 }
    }

    pub fn field_1(&self) -> u32 {
        1
    }
}

impl Default for TypeForIgnore {
    fn default() -> Self {
        Self::new()
    }
}
