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

use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::generator::codec::sse::*;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorImplTrait;
use crate::codegen_generator_structs;
use crate::library::codegen::ir::ty::IrTypeTrait;
use enum_dispatch::enum_dispatch;

codegen_generator_structs!(
    #[enum_dispatch(CodecSseTyTrait)]
    CodecSseTy
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct CodecSseTyContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
}

#[enum_dispatch]
pub(crate) trait CodecSseTyTrait {
    fn generate_encode(&self, lang: &Lang) -> String;

    fn generate_decode(&self, lang: &Lang) -> String;
}
