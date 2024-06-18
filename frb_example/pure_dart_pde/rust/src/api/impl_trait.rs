// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

// use crate::frb_generated::SimpleTraitForDynTwinNormalImplementor;
use flutter_rust_bridge::frb;

pub trait SimpleTraitTwinNormal {
    fn simple_trait_fn_twin_normal(value: i32) -> Self;

    fn simple_trait_fn_with_default_impl_twin_normal() -> i32 {
        42
    }

    fn simple_trait_fn_receiver_borrow_twin_normal(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitTwinNormal {
    pub one: i32,
}

impl SimpleTraitTwinNormal for StructOneWithTraitTwinNormal {
    fn simple_trait_fn_twin_normal(value: i32) -> Self {
        StructOneWithTraitTwinNormal { one: value }
    }

    fn simple_trait_fn_receiver_borrow_twin_normal(&self) -> i32 {
        self.one
    }
}

#[frb(opaque)]
pub struct StructTwoWithTraitTwinNormal {
    pub two: i32,
}

impl SimpleTraitTwinNormal for StructTwoWithTraitTwinNormal {
    fn simple_trait_fn_twin_normal(value: i32) -> Self {
        StructTwoWithTraitTwinNormal { two: value * 2 }
    }

    fn simple_trait_fn_receiver_borrow_twin_normal(&self) -> i32 {
        self.two * 2
    }
}

// #2089
pub trait MyTraitWithSelfTwinNormal {
    fn method_with_bad_self_twin_normal(&self, another: &Self);

    fn method_with_good_self_twin_normal(&self) -> Self;
}

#[frb(opaque)]
pub struct MyImplTraitWithSelfTwinNormal;

impl MyTraitWithSelfTwinNormal for MyImplTraitWithSelfTwinNormal {
    fn method_with_bad_self_twin_normal(&self, another: &Self) {
        let _ = another;
    }

    fn method_with_good_self_twin_normal(&self) -> Self {
        Self
    }
}
