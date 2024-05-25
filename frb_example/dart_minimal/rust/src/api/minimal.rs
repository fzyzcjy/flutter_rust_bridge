use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct StructWithSimpleSetterTwinNormal(i32);

impl StructWithSimpleSetterTwinNormal {
    #[frb(sync)]
    pub fn new() -> Self {
        Self(100)
    }

    #[frb(getter, sync)]
    pub fn simple_getter(&self) -> i32 {
        self.0
    }

    #[frb(setter, sync)]
    pub fn simple_setter(&mut self, value: i32) {
        self.0 = value;
    }
}
