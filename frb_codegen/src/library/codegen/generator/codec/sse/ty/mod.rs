mod boxed;
mod dart_fn;
mod dart_opaque;
pub(crate) mod delegate;
mod dynamic;
pub(crate) mod enumeration;
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

use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGeneratorContext;
use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::generator::codec::sse::*;
use crate::codegen::generator::codec::structs::EncodeOrDecode;
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
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

impl<'a> CodecSseTyContext<'a> {
    pub fn new(ir_pack: &'a IrPack, api_dart_config: &'a GeneratorApiDartInternalConfig) -> Self {
        Self {
            ir_pack,
            api_dart_config,
        }
    }

    pub(crate) fn as_api_dart_context(&self) -> ApiDartGeneratorContext {
        ApiDartGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.api_dart_config,
        }
    }
}

#[enum_dispatch]
pub(crate) trait CodecSseTyTrait {
    fn generate(&self, lang: &Lang, mode: EncodeOrDecode) -> Option<String> {
        match mode {
            EncodeOrDecode::Encode => self.generate_encode(lang),
            EncodeOrDecode::Decode => self.generate_decode(lang),
        }
    }

    fn generate_encode(&self, lang: &Lang) -> Option<String>;

    fn generate_decode(&self, lang: &Lang) -> Option<String>;
}
