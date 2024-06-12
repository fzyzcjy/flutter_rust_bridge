use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub use automerge::ScalarValue;

#[frb(external)]
impl ScalarValue {
    #[frb(sync)]
    pub fn is_str(&self) -> bool {}
}