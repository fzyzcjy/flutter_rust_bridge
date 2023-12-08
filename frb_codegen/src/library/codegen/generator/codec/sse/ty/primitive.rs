use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for PrimitiveCodecSseTy<'a> {
    fn generate_encode(&self, lang: &impl Lang) -> String {
        format!("{};", lang.call_encode(&Primitive(self.ir.clone()), "src"))
    }

    fn generate_decode(&self, lang: &impl Lang) -> String {
        format!("return {};", lang.call_decode(&Primitive(self.ir.clone())))
    }
}
