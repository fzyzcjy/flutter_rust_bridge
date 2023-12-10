use flutter_rust_bridge::{frb, DartFnFuture, DartOpaque};
pub use std::panic::{RefUnwindSafe, UnwindSafe};

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    panic!("hello this is deliberate panic")
}

pub fn hi(a: TheEnum) -> TheEnum {
    a
}

enum TheEnum {
    TheVariant(i32),
}
