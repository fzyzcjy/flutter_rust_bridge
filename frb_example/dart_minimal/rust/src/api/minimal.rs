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

#[flutter_rust_bridge::frb(opaque, ignore_all)]
pub struct TypeForIgnoreAll {
    pub field_1: u32,
    #[frb(unignore)]
    pub field_2: u32,
}

impl TypeForIgnoreAll {
    pub fn new() -> Self {
        Self {
            field_1: 0,
            field_2: 0,
        }
    }

    pub fn field_1(&self) -> u32 {
        1
    }
}

impl Default for TypeForIgnoreAll {
    fn default() -> Self {
        Self::new()
    }
}
