use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for BoxedCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        format!("{};", lang.call_encode(&*self.ir.inner, "src"))
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        format!("return {};", lang.call_decode(&*self.ir.inner))
    }
}
