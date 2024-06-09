use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub trait SimpleTraitForDynTwinNormal {
    fn simple_method_twin_normal(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitForDynTwinNormal {
    pub one: i32,
}

impl StructOneWithTraitForDynTwinNormal {
    pub fn create_twin_normal(one: i32) -> Self {
        Self { one }
    }
}

impl SimpleTraitForDynTwinNormal for StructOneWithTraitForDynTwinNormal {
    fn simple_method_twin_normal(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitForDynTwinNormal {
    pub two: i32,
}

impl StructTwoWithTraitForDynTwinNormal {
    pub fn create_twin_normal(two: i32) -> Self {
        Self { two }
    }
}

impl SimpleTraitForDynTwinNormal for StructTwoWithTraitForDynTwinNormal {
    fn simple_method_twin_normal(&self) -> i32 {
        self.two * 2
    }
}

pub fn func_arg_dyn_trait_twin_normal(arg: &dyn SimpleTraitForDynTwinNormal) -> i32 {
    arg.simple_method_twin_normal()
}
