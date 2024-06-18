// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

// use crate::frb_generated::SimpleTraitForDynTwinSyncImplementor;
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

// #2089
pub trait MyTraitWithSelfTwinSync {
    fn method_with_bad_self_twin_sync(&self, another: &Self);

    fn method_with_good_self_twin_sync(&self) -> Self;
}

#[frb(opaque)]
pub struct MyImplTraitWithSelfTwinSync;

impl MyTraitWithSelfTwinSync for MyImplTraitWithSelfTwinSync {
    fn method_with_bad_self_twin_sync(&self, another: &Self) {
        let _ = another;
    }

    fn method_with_good_self_twin_sync(&self) -> Self {
        Self
    }
}
