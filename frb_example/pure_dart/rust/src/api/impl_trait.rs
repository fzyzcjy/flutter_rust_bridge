pub struct StructWithTraitTwinNormal {
    pub value: u32,
}

pub trait SimpleTraitTwinNormal {
    fn simple_trait_fn_twin_normal() -> Self;
}

impl SimpleTraitTwinNormal for StructWithTraitTwinNormal {
    fn simple_trait_fn_twin_normal() -> Self {
        StructWithTraitTwinNormal { value: 42 }
    }
}
