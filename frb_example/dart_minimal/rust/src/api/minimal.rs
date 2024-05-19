use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct StructWithCustomNameMethodTwinNormal(i32);

impl StructWithCustomNameMethodTwinNormal {
    #[frb(name = "operator<", sync)]
    pub fn less_than(&self, other: &StructWithCustomNameMethodTwinNormal) -> bool {
        self.0 < other.0
    }
}

#[frb(name = "renamedFunction")]
pub fn function_with_custom_name_twin_normal() {}
