use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for EnumRefCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        format!("return TODO;")
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        format!("return TODO;")
    }
}
