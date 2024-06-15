// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::frb;

pub trait SimpleTraitForDynTwinNormal {
    #[frb(serialize)]
    fn simple_method_twin_normal(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitForDynTwinNormal {
    pub one: i32,
}

impl StructOneWithTraitForDynTwinNormal {
    #[frb(serialize)]
    pub fn create_twin_normal(one: i32) -> Self {
        Self { one }
    }
}

impl SimpleTraitForDynTwinNormal for StructOneWithTraitForDynTwinNormal {
    #[frb(serialize)]
    fn simple_method_twin_normal(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitForDynTwinNormal {
    pub two: i32,
}

impl StructTwoWithTraitForDynTwinNormal {
    #[frb(serialize)]
    pub fn create_twin_normal(two: i32) -> Self {
        Self { two }
    }
}

impl SimpleTraitForDynTwinNormal for StructTwoWithTraitForDynTwinNormal {
    #[frb(serialize)]
    fn simple_method_twin_normal(&self) -> i32 {
        self.two * 2
    }
}

#[frb(serialize)]
pub fn func_arg_dyn_trait_twin_normal(arg: &dyn SimpleTraitForDynTwinNormal) -> i32 {
    arg.simple_method_twin_normal()
}
