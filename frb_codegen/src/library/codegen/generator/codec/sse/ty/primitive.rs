use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for PrimitiveCodecSseTy<'a> {
    fn generate_encode(&self, lang: &impl Lang) -> String {
        format!("serializer.serialize_{};", TODO);
    }

    fn generate_decode(&self, lang: &impl Lang) -> String {
        format!("return deserializer.deserialize_{};", TODO);
    }
}
