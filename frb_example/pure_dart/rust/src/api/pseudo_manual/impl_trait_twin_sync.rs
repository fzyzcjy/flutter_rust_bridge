// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub struct StructWithDefaultValue {
    pub val: u32,
}

pub trait TestTrait {
    fn trait_fun() -> Self;
}

impl TestTrait for StructWithDefaultValue {
    fn trait_fun() -> Self {
        StructWithDefaultValue { val: 42 }
    }
}
