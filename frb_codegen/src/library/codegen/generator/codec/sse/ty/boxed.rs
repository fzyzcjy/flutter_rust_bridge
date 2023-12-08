use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for BoxedCodecSseTy<'a> {
    fn generate_encode(&self, lang: &impl Lang) -> String {
        lang.call_encode(&*self.ir.inner, "src")
    }

    fn generate_decode(&self, lang: &impl Lang) -> String {
        format!("return {};", lang.call_decode(&*self.ir.inner))
    }
}
