use flutter_rust_bridge::frb;
pub use std::panic::{RefUnwindSafe, UnwindSafe};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct AnotherOpaqueType {}

#[frb(opaque)]
pub struct MyOpaqueType {
    pub sad: AnotherOpaqueType,
}

pub fn foo() -> (MyOpaqueType, AnotherOpaqueType) {
    todo!()
}
pub fn bar() -> AnotherOpaqueType {
    todo!()
}
