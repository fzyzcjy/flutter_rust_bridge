use crate::codegen::generator::codec::sse::ty::*;

impl CodecSseTyTrait for TraitDefCodecSseTy<'_> {
    fn generate_encode(&self, _lang: &Lang) -> Option<String> {
        None
    }

    fn generate_decode(&self, _lang: &Lang) -> Option<String> {
        None
    }
}
