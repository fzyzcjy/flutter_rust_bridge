use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO temp demo
pub enum SimplePrimitiveEnumTwinNormal {
    First,
    Second,
}

impl SimplePrimitiveEnumTwinNormal {
    pub fn simple_method_twin_normal(&self) -> i32 {
        match self {
            SimplePrimitiveEnumTwinNormal::First => 100,
            SimplePrimitiveEnumTwinNormal::Second => 200,
        }
    }
}
