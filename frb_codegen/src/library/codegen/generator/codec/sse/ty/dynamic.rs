use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DynamicCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        format!("{};", lang.throw_unimplemented())
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        format!("{};", lang.throw_unimplemented())
    }
}
