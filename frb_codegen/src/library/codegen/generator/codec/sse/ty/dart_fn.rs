use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DartFnCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        self.should_generate(lang)
            .then(|| simple_delegate_encode(lang, &self.ir.get_delegate(), "self"))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        None
    }
}

impl<'a> DartFnCodecSseTy<'a> {
    fn should_generate(&self, lang: &Lang) -> bool {
        !matches!(lang, Lang::RustLang(_))
    }
}
