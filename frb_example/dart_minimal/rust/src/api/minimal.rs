use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn f(a: Vec<u64>) -> Vec<u64> {
    a
}

pub fn func_type_fallible_panic_twin_normal() -> Result<i32> {
    panic!("deliberate panic")
}
