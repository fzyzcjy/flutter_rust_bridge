use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO move
// #[derive(Debug, PartialEq, Eq, Default, Clone)]
#[derive(Debug)]
pub struct StructWithRustAutoOpaqueFieldWithManyDerive {
    pub content: crate::frb_generated::RustAutoOpaque<String>,
}

impl StructWithRustAutoOpaqueFieldWithManyDerive {
    pub fn f(&self) {}
}
