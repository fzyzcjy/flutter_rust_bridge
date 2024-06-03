use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}


pub struct StructWithTraitTwinNormal {
    pub value: u32,
}

pub trait SimpleTraitTwinNormal {
    fn simple_trait_fn_twin_normal() -> Self;

    fn simple_trait_fn_with_default_impl_twin_normal() -> i32 {
        42
    }
}

impl SimpleTraitTwinNormal for StructWithTraitTwinNormal {
    fn simple_trait_fn_twin_normal() -> Self {
        StructWithTraitTwinNormal { value: 42 }
    }
}
