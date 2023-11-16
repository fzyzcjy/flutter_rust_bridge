use crate::codegen::generator::dart_api::base::*;
use enum_dispatch::enum_dispatch;

pub(super) mod delegate;
pub(super) mod enumeration;
pub(super) mod rust_opaque;
pub(super) mod structure;

#[enum_dispatch]
pub(super) trait DartApiClassGeneratorTrait {
    // TODO
    // fn generate(&self);
}
