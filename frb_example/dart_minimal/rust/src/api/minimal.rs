use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub trait MyTraitToBeUsedAsBoxDyn {
    fn my_method_twin_normal(&self);
}

fn function_with_box_dyn_trait(arg: Box<dyn MyTraitToBeUsedAsBoxDyn>) {
    let _ = arg;
}
