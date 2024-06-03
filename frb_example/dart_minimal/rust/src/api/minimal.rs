use flutter_rust_bridge::frb;
pub use flutter_rust_bridge::HelloStruct;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO demo
#[frb(mirror(HelloStruct))]
pub struct _HelloStruct {
    pub a: i32,
    pub b: String,
}
pub fn f(a: HelloStruct) -> HelloStruct {
    a
}
