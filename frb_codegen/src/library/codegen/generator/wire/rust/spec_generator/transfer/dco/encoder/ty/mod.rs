mod boxed;
mod dart_fn;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enumeration;
mod general_list;
mod optional;
mod ownership;
mod primitive;
mod primitive_list;
mod record;
mod rust_auto_opaque;
mod rust_opaque;
mod structure;
mod unencodable;

use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorImplTrait;
use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::base::*;
use crate::codegen::ir::pack::IrPack;
use crate::library::codegen::ir::ty::IrTypeTrait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustCodecDcoGeneratorEncoderTrait:
    WireRustCodecDcoGeneratorImplTrait
{
    fn intodart_type(&self, _ir_pack: &IrPack) -> String {
        self.ir_type().rust_api_type()
    }

    fn generate_impl_into_dart(&self) -> Option<String> {
        None
    }

    fn generate_access_object_core(&self, obj: String) -> String {
        obj
    }
}
