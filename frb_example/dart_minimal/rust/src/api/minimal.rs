pub use std::panic::{RefUnwindSafe, UnwindSafe};

// TODO for temporary experiment
pub struct MyStruct {
    my_field: String,
}

pub fn minimal_adder(a: i32, b: i32, x: MyStruct) -> i32 {
    a + b
}
