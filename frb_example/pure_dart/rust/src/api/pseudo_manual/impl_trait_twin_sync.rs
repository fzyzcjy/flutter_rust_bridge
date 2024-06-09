// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

use crate::frb_generated::SimpleTraitForDynTwinSyncImplementor;
use flutter_rust_bridge::frb;

pub trait SimpleTraitTwinSync {
    fn simple_trait_fn_twin_sync(value: i32) -> Self;

    fn simple_trait_fn_with_default_impl_twin_sync() -> i32 {
        42
    }

    fn simple_trait_fn_receiver_borrow_twin_sync(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitTwinSync {
    pub one: i32,
}

impl SimpleTraitTwinSync for StructOneWithTraitTwinSync {
    fn simple_trait_fn_twin_sync(value: i32) -> Self {
        StructOneWithTraitTwinSync { one: value }
    }

    fn simple_trait_fn_receiver_borrow_twin_sync(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitTwinSync {
    pub two: i32,
}

impl SimpleTraitTwinSync for StructTwoWithTraitTwinSync {
    fn simple_trait_fn_twin_sync(value: i32) -> Self {
        StructTwoWithTraitTwinSync { two: value * 2 }
    }

    fn simple_trait_fn_receiver_borrow_twin_sync(&self) -> i32 {
        self.two * 2
    }
}

#[frb(generate_implementor_enum)]
pub trait SimpleTraitForDynTwinSync {
    fn simple_method_twin_sync(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitForDynTwinSync {
    pub one: i32,
}

impl StructOneWithTraitForDynTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn create_twin_sync(one: i32) -> Self {
        Self { one }
    }
}

impl SimpleTraitForDynTwinSync for StructOneWithTraitForDynTwinSync {
    fn simple_method_twin_sync(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitForDynTwinSync {
    pub two: i32,
}

impl SimpleTraitForDynTwinSync for StructTwoWithTraitForDynTwinSync {
    fn simple_method_twin_sync(&self) -> i32 {
        self.two * 2
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_arg_trait_impl_twin_sync(arg: SimpleTraitForDynTwinSyncImplementor) -> i32 {
    let arg = arg.blocking_read();
    arg.simple_method_twin_sync()
}
