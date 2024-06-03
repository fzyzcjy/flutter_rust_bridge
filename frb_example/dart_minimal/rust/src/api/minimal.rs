use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO temp
pub fn borrow_str_twin_normal(arg: &str) -> &str {
    arg
}
pub fn borrow_vec_u8_twin_normal(arg: &[u8]) -> &[u8] {
    arg
}
