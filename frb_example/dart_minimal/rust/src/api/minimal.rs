use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct DisposalProbe;

impl DisposalProbe {
    pub fn new() -> Self {
        Self
    }

    pub fn read(&self) -> i32 {
        0
    }
}
