use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for DynamicCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        format!("{};", lang.throw_unimplemented())
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        format!("{};", lang.throw_unimplemented())
    }
}
