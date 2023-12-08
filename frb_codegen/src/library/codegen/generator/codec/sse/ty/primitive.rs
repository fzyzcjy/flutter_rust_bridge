use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for PrimitiveCodecSseTy<'a> {
    fn generate_encode(&self, _lang: &impl Lang) -> String {
        format!("serializer.serialize_{};", self.ir.safe_ident());
    }

    fn generate_decode(&self, _lang: &impl Lang) -> String {
        format!("return deserializer.deserialize_{};", self.ir.safe_ident());
    }
}
