pub use std::panic::{RefUnwindSafe, UnwindSafe};

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn hi_rust_opaque(a: Box<dyn Fn() + UnwindSafe + RefUnwindSafe>) {}
