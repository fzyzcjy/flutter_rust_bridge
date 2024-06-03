// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

pub struct StructWithTraitTwinNormal {
    pub value: u32,
}

pub trait SimpleTraitTwinNormal {
    fn simple_trait_fn_twin_normal() -> Self;

    fn simple_trait_fn_with_default_impl_twin_normal() -> i32 {
        42
    }
}

impl SimpleTraitTwinNormal for StructWithTraitTwinNormal {
    fn simple_trait_fn_twin_normal() -> Self {
        StructWithTraitTwinNormal { value: 42 }
    }
}
