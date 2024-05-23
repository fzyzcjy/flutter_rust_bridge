use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn example_basic_type_i64_twin_normal(arg: i64, expect: String) -> i64 {
    assert_eq!(arg, expect.parse().unwrap());
    arg
}

pub fn example_basic_type_u64_twin_normal(arg: u64, expect: String) -> u64 {
    assert_eq!(arg, expect.parse().unwrap());
    arg
}
