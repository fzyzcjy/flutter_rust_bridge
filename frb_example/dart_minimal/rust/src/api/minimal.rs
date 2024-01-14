use flutter_rust_bridge::frb;
pub use std::panic::{RefUnwindSafe, UnwindSafe};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO temporary experiment
pub struct MyStruct {
    pub field: String,
}

pub enum MyEnum {
    Apple,
    Orange(Vec<u8>),
}

pub fn hello_ty(x: MyStruct, y: MyEnum) {}
