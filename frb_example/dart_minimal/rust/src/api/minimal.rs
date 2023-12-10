use flutter_rust_bridge::{frb, DartOpaque};
pub use std::panic::{RefUnwindSafe, UnwindSafe};

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn hi_one(a: DartOpaque) -> DartOpaque {
    a
}

#[frb(serialize)]
pub fn hi_two(a: DartOpaque) -> DartOpaque {
    a
}
