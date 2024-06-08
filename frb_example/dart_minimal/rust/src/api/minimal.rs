use crate::frb_generated::{RustAutoOpaque, SimpleTraitForDynTwinNormalImpl, SimpleTraitTwinNormalImpl};
use flutter_rust_bridge::frb;
use flutter_rust_bridge::rust_async::RwLockReadGuard;
use std::ops;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO temp demo

#[frb(generate_impl_enum)]
pub trait SimpleTraitForDynTwinNormal {
    fn simple_method_twin_normal(&self) -> i32;
}

#[frb(opaque)]
pub struct StructOneWithTraitForDynTwinNormal {
    pub one: i32,
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

impl SimpleTraitForDynTwinNormal for StructTwoWithTraitForDynTwinNormal {
    fn simple_method_twin_normal(&self) -> i32 {
        self.two * 2
    }
}

pub fn func_arg_trait_impl_twin_normal(arg: SimpleTraitForDynTwinNormalImpl) -> i32 {
    let arg = arg.blocking_read();
    arg.simple_method_twin_normal()
}

impl SimpleTraitForDynTwinNormalImpl {
    #[frb(ignore)]
    pub fn blocking_read(&self) -> SimpleTraitTwinNormalRwLockReadGuard {
        match self {
            Self::StructOneWithTraitTwinNormal(inner) => {
                SimpleTraitTwinNormalRwLockReadGuard::StructOneWithTraitTwinNormal(
                    inner.blocking_read(),
                )
            }
            Self::StructTwoWithTraitTwinNormal(inner) => {
                SimpleTraitTwinNormalRwLockReadGuard::StructTwoWithTraitTwinNormal(
                    inner.blocking_read(),
                )
            }
        }
    }
}

#[frb(ignore)]
pub enum SimpleTraitTwinNormalRwLockReadGuard<'a> {
    StructOneWithTraitTwinNormal(
        flutter_rust_bridge::for_generated::rust_async::RwLockReadGuard<'a, StructOneWithTraitForDynTwinNormal, >,
    ),
    StructTwoWithTraitTwinNormal(
        flutter_rust_bridge::for_generated::rust_async::RwLockReadGuard<
            'a,
            StructTwoWithTraitForDynTwinNormal,
        >,
    ),
}

impl std::ops::Deref for SimpleTraitTwinNormalRwLockReadGuard<'_> {
    type Target = dyn SimpleTraitForDynTwinNormal;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::StructOneWithTraitTwinNormal(inner) => inner.deref(),
            Self::StructTwoWithTraitTwinNormal(inner) => inner.deref(),
        }
    }
}
