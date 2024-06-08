use crate::frb_generated::SimpleTraitTwinNormalImpl;
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
pub trait SimpleTraitTwinNormal {
    fn simple_trait_fn_twin_normal(value: i32) -> Self;

    fn simple_trait_fn_with_default_impl_twin_normal() -> i32 {
        42
    }

    fn simple_trait_fn_receiver_borrow_twin_normal(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitTwinNormal {
    pub one: i32,
}

impl SimpleTraitTwinNormal for StructOneWithTraitTwinNormal {
    fn simple_trait_fn_twin_normal(value: i32) -> Self {
        StructOneWithTraitTwinNormal { one: value }
    }

    fn simple_trait_fn_receiver_borrow_twin_normal(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitTwinNormal {
    pub two: i32,
}

impl SimpleTraitTwinNormal for StructTwoWithTraitTwinNormal {
    fn simple_trait_fn_twin_normal(value: i32) -> Self {
        StructTwoWithTraitTwinNormal { two: value * 2 }
    }

    fn simple_trait_fn_receiver_borrow_twin_normal(&self) -> i32 {
        self.two * 2
    }
}

pub fn func_arg_trait_impl_twin_normal(arg: SimpleTraitTwinNormalImpl) -> i32 {
    // TODO
}
