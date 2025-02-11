use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// These consts should be ignored
pub(crate) const CONST_PUB_CRATE_SHOULD_IGNORE: i32 = 42;
const CONST_PRIVATE_SHOULD_IGNORE: i32 = 42;
#[frb(ignore)]
pub const CONST_WITH_EXPLICIT_IGNORE_SHOULD_IGNORE: i32 = 42;
