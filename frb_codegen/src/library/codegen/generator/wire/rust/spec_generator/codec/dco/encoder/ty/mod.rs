mod boxed;
mod dart_fn;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enumeration;
mod general_list;
mod optional;
mod primitive;
mod primitive_list;
mod record;
mod rust_auto_opaque;
mod rust_opaque;
mod structure;

use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::ir::pack::IrPack;
use crate::library::codegen::ir::ty::IrTypeTrait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustCodecDcoGeneratorEncoderTrait:
    WireRustCodecDcoGeneratorImplTrait
{
    fn generate_impl_into_dart(&self) -> Option<String> {
        None
    }
}
