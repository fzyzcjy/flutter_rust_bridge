use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn example_basic_list_type_u64_twin_normal(arg: Vec<u64>) -> Vec<u64> {
    arg
}
