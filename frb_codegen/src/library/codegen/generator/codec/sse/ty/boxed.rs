use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for BoxedCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        self.should_generate(lang)
            .then(|| format!("{};", lang.call_encode(&*self.ir.inner, "self")))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        self.should_generate(lang)
            .then(|| format!("return {};", lang.call_decode(&*self.ir.inner)))
    }
}

impl<'a> BoxedCodecSseTy<'a> {
    fn should_generate(&self, lang: &Lang) -> bool {
        // The fake Box is only needed for CST codec, thus here we mostly ignore it.
        self.ir.exist_in_real_api || matches!(lang, Lang::DartLang(_))
    }
}
