// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub struct StructWithTraitTwinSync {
    pub value: u32,
}

pub trait SimpleTraitTwinSync {
    fn simple_trait_fn_twin_sync() -> Self;
}

impl SimpleTraitTwinSync for StructWithTraitTwinSync {
    fn simple_trait_fn_twin_sync() -> Self {
        StructWithTraitTwinSync { value: 42 }
    }
}
