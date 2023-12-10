pub use std::panic::{RefUnwindSafe, UnwindSafe};

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn example_primitive_list_type_u8_twin_sync_sse(arg: Vec<u8>) -> Vec<u8> {
    arg
}
