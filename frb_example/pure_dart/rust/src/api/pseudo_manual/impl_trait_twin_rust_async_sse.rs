// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;

pub trait SimpleTraitTwinRustAsyncSse {
    fn simple_trait_fn_twin_rust_async_sse(value: i32) -> Self;

    fn simple_trait_fn_with_default_impl_twin_rust_async_sse() -> i32 {
        42
    }

    fn simple_trait_fn_receiver_borrow_twin_rust_async_sse(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitTwinRustAsyncSse {
    pub one: i32,
}

impl SimpleTraitTwinRustAsyncSse for StructOneWithTraitTwinRustAsyncSse {
    fn simple_trait_fn_twin_rust_async_sse(value: i32) -> Self {
        StructOneWithTraitTwinRustAsyncSse { one: value }
    }

    fn simple_trait_fn_receiver_borrow_twin_rust_async_sse(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitTwinRustAsyncSse {
    pub two: i32,
}

impl SimpleTraitTwinRustAsyncSse for StructTwoWithTraitTwinRustAsyncSse {
    fn simple_trait_fn_twin_rust_async_sse(value: i32) -> Self {
        StructTwoWithTraitTwinRustAsyncSse { two: value * 2 }
    }

    fn simple_trait_fn_receiver_borrow_twin_rust_async_sse(&self) -> i32 {
        self.two * 2
    }
}
