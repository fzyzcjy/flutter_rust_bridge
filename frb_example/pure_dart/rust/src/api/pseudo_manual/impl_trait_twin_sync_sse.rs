// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

use crate::frb_generated::SimpleTraitForDynTwinSyncSseImplementor;
use flutter_rust_bridge::frb;

pub trait SimpleTraitTwinSyncSse {
    fn simple_trait_fn_twin_sync_sse(value: i32) -> Self;

    fn simple_trait_fn_with_default_impl_twin_sync_sse() -> i32 {
        42
    }

    fn simple_trait_fn_receiver_borrow_twin_sync_sse(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitTwinSyncSse {
    pub one: i32,
}

impl SimpleTraitTwinSyncSse for StructOneWithTraitTwinSyncSse {
    fn simple_trait_fn_twin_sync_sse(value: i32) -> Self {
        StructOneWithTraitTwinSyncSse { one: value }
    }

    fn simple_trait_fn_receiver_borrow_twin_sync_sse(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitTwinSyncSse {
    pub two: i32,
}

impl SimpleTraitTwinSyncSse for StructTwoWithTraitTwinSyncSse {
    fn simple_trait_fn_twin_sync_sse(value: i32) -> Self {
        StructTwoWithTraitTwinSyncSse { two: value * 2 }
    }

    fn simple_trait_fn_receiver_borrow_twin_sync_sse(&self) -> i32 {
        self.two * 2
    }
}

#[frb(generate_implementor_enum)]
pub trait SimpleTraitForDynTwinSyncSse {
    fn simple_method_twin_sync_sse(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitForDynTwinSyncSse {
    pub one: i32,
}

impl StructOneWithTraitForDynTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn create_twin_sync_sse(one: i32) -> Self {
        Self { one }
    }
}

impl SimpleTraitForDynTwinSyncSse for StructOneWithTraitForDynTwinSyncSse {
    fn simple_method_twin_sync_sse(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitForDynTwinSyncSse {
    pub two: i32,
}

impl SimpleTraitForDynTwinSyncSse for StructTwoWithTraitForDynTwinSyncSse {
    fn simple_method_twin_sync_sse(&self) -> i32 {
        self.two * 2
    }
}

// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn func_arg_trait_impl_twin_sync_sse(arg: SimpleTraitForDynTwinSyncSseImplementor) -> i32 {
//     let arg = arg.blocking_read();
//     arg.simple_method_twin_sync_sse()
// }
