use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct StructOne {}

impl StructOne {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {}
    }
}

#[frb(opaque)]
pub struct StructTwo {}

impl StructTwo {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {}
    }
}

pub fn f(a: &mut StructOne, b: &mut StructTwo) {}
