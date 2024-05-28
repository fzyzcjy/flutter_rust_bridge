use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(positional)]
pub fn f(a: i32, b: i32) {}

pub struct S {
    a: i32,
}

impl S {
    #[frb(positional)]
    pub fn g(&self, c: i32) {}
}
