// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

use crate::frb_generated::SimpleTraitForDynTwinSseImplementor;
use flutter_rust_bridge::frb;

pub trait SimpleTraitTwinSse {
    fn simple_trait_fn_twin_sse(value: i32) -> Self;

    fn simple_trait_fn_with_default_impl_twin_sse() -> i32 {
        42
    }

    fn simple_trait_fn_receiver_borrow_twin_sse(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitTwinSse {
    pub one: i32,
}

impl SimpleTraitTwinSse for StructOneWithTraitTwinSse {
    fn simple_trait_fn_twin_sse(value: i32) -> Self {
        StructOneWithTraitTwinSse { one: value }
    }

    fn simple_trait_fn_receiver_borrow_twin_sse(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitTwinSse {
    pub two: i32,
}

impl SimpleTraitTwinSse for StructTwoWithTraitTwinSse {
    fn simple_trait_fn_twin_sse(value: i32) -> Self {
        StructTwoWithTraitTwinSse { two: value * 2 }
    }

    fn simple_trait_fn_receiver_borrow_twin_sse(&self) -> i32 {
        self.two * 2
    }
}

#[frb(generate_implementor_enum)]
pub trait SimpleTraitForDynTwinSse {
    fn simple_method_twin_sse(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitForDynTwinSse {
    pub one: i32,
}

impl StructOneWithTraitForDynTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn create_twin_sse(one: i32) -> Self {
        Self { one }
    }
}

impl SimpleTraitForDynTwinSse for StructOneWithTraitForDynTwinSse {
    fn simple_method_twin_sse(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitForDynTwinSse {
    pub two: i32,
}

impl SimpleTraitForDynTwinSse for StructTwoWithTraitForDynTwinSse {
    fn simple_method_twin_sse(&self) -> i32 {
        self.two * 2
    }
}

// #[flutter_rust_bridge::frb(serialize)] pub fn func_arg_trait_impl_twin_sse(arg: SimpleTraitForDynTwinSseImplementor) -> i32 {
//     let arg = arg.blocking_read();
//     arg.simple_method_twin_sse()
// }
