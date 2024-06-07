// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;

pub trait SimpleTraitTwinRustAsync {
    fn simple_trait_fn_twin_rust_async(value: i32) -> Self;

    fn simple_trait_fn_with_default_impl_twin_rust_async() -> i32 {
        42
    }

    fn simple_trait_fn_receiver_borrow_twin_rust_async(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitTwinRustAsync {
    pub one: i32,
}

impl SimpleTraitTwinRustAsync for StructOneWithTraitTwinRustAsync {
    fn simple_trait_fn_twin_rust_async(value: i32) -> Self {
        StructOneWithTraitTwinRustAsync { one: value }
    }

    fn simple_trait_fn_receiver_borrow_twin_rust_async(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitTwinRustAsync {
    pub two: i32,
}

impl SimpleTraitTwinRustAsync for StructTwoWithTraitTwinRustAsync {
    fn simple_trait_fn_twin_rust_async(value: i32) -> Self {
        StructTwoWithTraitTwinRustAsync { two: value * 2 }
    }

    fn simple_trait_fn_receiver_borrow_twin_rust_async(&self) -> i32 {
        self.two * 2
    }
}
