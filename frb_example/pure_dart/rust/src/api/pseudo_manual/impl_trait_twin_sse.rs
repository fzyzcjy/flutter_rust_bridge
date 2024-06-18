// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

// use crate::frb_generated::SimpleTraitForDynTwinSseImplementor;
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

// #2089
pub trait MyTraitWithSelfTwinSse {
    fn method_with_bad_self_twin_sse(&self, another: &Self);

    fn method_with_good_self_twin_sse(&self) -> Self;
}

#[frb(opaque)]
pub struct MyImplTraitWithSelfTwinSse;

impl MyTraitWithSelfTwinSse for MyImplTraitWithSelfTwinSse {
    fn method_with_bad_self_twin_sse(&self, another: &Self) {
        let _ = another;
    }

    fn method_with_good_self_twin_sse(&self) -> Self {
        Self
    }
}
