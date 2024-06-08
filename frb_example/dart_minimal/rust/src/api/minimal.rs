use crate::frb_generated::SimpleTraitForDynTwinNormalImpl;
use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO temp demo

#[frb(generate_impl_enum)]
pub trait SimpleTraitForDynTwinNormal {
    fn simple_method_twin_normal(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitForDynTwinNormal {
    pub one: i32,
}

impl SimpleTraitForDynTwinNormal for StructOneWithTraitForDynTwinNormal {
    fn simple_method_twin_normal(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitForDynTwinNormal {
    pub two: i32,
}

impl SimpleTraitForDynTwinNormal for StructTwoWithTraitForDynTwinNormal {
    fn simple_method_twin_normal(&self) -> i32 {
        self.two * 2
    }
}

pub fn func_arg_trait_impl_twin_normal(arg: SimpleTraitForDynTwinNormalImpl) -> i32 {
    let arg = arg.blocking_read();
    arg.simple_method_twin_normal()
}
