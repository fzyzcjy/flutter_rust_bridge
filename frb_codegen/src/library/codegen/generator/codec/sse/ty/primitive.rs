use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for PrimitiveCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        format!("return TODO_depend_on_serializer;")
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        format!("return TODO_depend_on_serializer;")
    }
}
