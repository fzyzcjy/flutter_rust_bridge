use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// #2089
pub trait MyTraitWithSelfTwinNormal {
    fn method_twin_normal(&self, another: &Self);
}

#[frb(opaque)]
pub struct MyImplTraitWithSelfTwinNormal;

impl MyTraitWithSelfTwinNormal for MyImplTraitWithSelfTwinNormal {
    fn method_twin_normal(&self, another: &Self) {
        let _ = another;
    }
}
