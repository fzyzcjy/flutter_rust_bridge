use flutter_rust_bridge::{frb, DartOpaque};
pub use std::panic::{RefUnwindSafe, UnwindSafe};

#[frb(serialize)]
pub async fn minimal_adder(a: i32, b: i32) -> i32 {
    panic!("hello this is deliberate panic")
}
