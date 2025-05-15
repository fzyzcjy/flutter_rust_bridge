use crate::codegen::generator::codec::sse::ty::delegate::simple_delegate_encode;
use crate::codegen::generator::codec::sse::ty::*;

impl CodecSseTyTrait for DartFnCodecSseTy<'_> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        self.should_generate(lang).then(|| {
            simple_delegate_encode(
                lang,
                &self.mir.get_delegate(),
                &format!("encode_{}(self)", self.mir.safe_ident()),
            )
        })
    }

    fn generate_decode(&self, _lang: &Lang) -> Option<String> {
        None
    }
}

impl DartFnCodecSseTy<'_> {
    fn should_generate(&self, lang: &Lang) -> bool {
        !matches!(lang, Lang::RustLang(_))
    }
}
