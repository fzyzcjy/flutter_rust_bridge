use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO temp
pub struct MyExternalType {
    this_is_fake_external: i32,
}
#[frb(external)]
impl MyExternalType {
    pub fn f(&self, a: i32) {}
}
