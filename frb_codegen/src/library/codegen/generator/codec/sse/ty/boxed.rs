use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for BoxedCodecSseTy<'a> {
    fn generate_encode(&self, lang: &impl Lang) -> String {
        lang.call_encode(&*self.ir.inner)
    }

    fn generate_decode(&self, lang: &impl Lang) -> String {
        lang.call_decode(&*self.ir.inner)
    }
}
