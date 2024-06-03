// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub struct StructWithTraitTwinRustAsyncSse {
    pub value: u32,
}

pub trait SimpleTraitTwinRustAsyncSse {
    fn simple_trait_fn_twin_rust_async_sse() -> Self;

    fn simple_trait_fn_with_default_impl_twin_rust_async_sse() -> i32 {
        42
    }
}

impl SimpleTraitTwinRustAsyncSse for StructWithTraitTwinRustAsyncSse {
    fn simple_trait_fn_twin_rust_async_sse() -> Self {
        StructWithTraitTwinRustAsyncSse { value: 42 }
    }
}
