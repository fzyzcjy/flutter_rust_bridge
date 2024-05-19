use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub struct StructWithCustomNameMethodTwinNormal(i32);

impl StructWithCustomNameMethodTwinNormal {
    #[frb(name = "renamedMethod", sync)]
    pub fn method_with_custom_name_twin_normal(&self) {}
}

#[frb(name = "renamedFunction")]
pub fn function_with_custom_name_twin_normal() {}
