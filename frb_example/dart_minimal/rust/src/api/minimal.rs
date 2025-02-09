use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

use std::collections::HashSet;

// #[flutter_rust_bridge::frb(sync)]
// pub fn fn_hashset_u64(vals: HashSet<u64>) -> String {
//     format!("{:?}", vals)
// }

#[flutter_rust_bridge::frb(sync)]
pub fn fn_vec_u64(vals: Vec<u64>) -> String {
    format!("{:?}", vals)
}
