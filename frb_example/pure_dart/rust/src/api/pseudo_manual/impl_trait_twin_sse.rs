// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub struct StructWithTraitTwinSse {
    pub value: u32,
}

pub trait SimpleTraitTwinSse {
    fn simple_trait_fn_twin_sse() -> Self;
}

impl SimpleTraitTwinSse for StructWithTraitTwinSse {
    fn simple_trait_fn_twin_sse() -> Self {
        StructWithTraitTwinSse { value: 42 }
    }
}
