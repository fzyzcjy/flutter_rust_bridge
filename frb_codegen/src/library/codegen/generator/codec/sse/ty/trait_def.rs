use crate::codegen::generator::api_dart::spec_generator::class::method::dart_constructor_postfix;
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::generator::misc::StructOrRecord;
use crate::codegen::generator::misc::StructOrRecord::Struct;
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;
use itertools::Itertools;

impl<'a> CodecSseTyTrait for TraitDefCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        None
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        None
    }
}
