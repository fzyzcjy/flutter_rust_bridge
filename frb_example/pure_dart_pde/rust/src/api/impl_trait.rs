// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

// use crate::frb_generated::SimpleTraitForDynTwinNormalImplementor;
use flutter_rust_bridge::frb;

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

#[frb(generate_implementor_enum)]
pub trait SimpleTraitForDynTwinNormal {
    fn simple_method_twin_normal(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitForDynTwinNormal {
    pub one: i32,
}

impl StructOneWithTraitForDynTwinNormal {
    pub fn create_twin_normal(one: i32) -> Self {
        Self { one }
    }
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

// pub fn func_arg_trait_impl_twin_normal(arg: SimpleTraitForDynTwinNormalImplementor) -> i32 {
//     let arg = arg.blocking_read();
//     arg.simple_method_twin_normal()
// }
