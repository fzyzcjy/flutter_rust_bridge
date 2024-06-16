use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// ----------------------------------------------- tests -------------------------------------------------

// Lt := Lifetime Testers
// Try *NOT* to impl Clone to ensure there are no extra clones
#[frb(opaque)]
#[derive(Debug)]
pub struct LtOneTwinNormal {
    value: String,
}

// Try *NOT* to impl Clone to ensure there are no extra clones
#[frb(opaque)]
#[derive(Debug)]
pub struct LtTwoTwinNormal<'a> {
    one: &'a LtOneTwinNormal,
}

impl LtOneTwinNormal {
    pub fn compute_two_twin_normal(&self) -> LtTwoTwinNormal {
        LtTwoTwinNormal { one: self }
    }
}
