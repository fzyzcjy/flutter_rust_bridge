use crate::codegen::generator::codec::sse::ty::rust_opaque::{
    generate_generalized_rust_opaque_decode, generate_generalized_rust_opaque_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for RustAutoOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let needs_move = self.ir.needs_move();
        self.should_generate(lang)
            .then(|| generate_generalized_rust_opaque_encode(lang, &format!("{needs_move}")))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        self.should_generate(lang).then(|| {
            generate_generalized_rust_opaque_decode(
                lang,
                self.ir.clone().into(),
                self.ir.inner.codec,
                self.context,
            )
        })
    }

    fn generate_extra(&self, lang: &Lang) -> Option<String> {
        match lang {
            Lang::DartLang(_) => None,
            Lang::RustLang(_) => {
                let arc = self.ir.inner.codec.arc_ty();
                Some(format!("flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, {arc}<_>>(self)"))
            }
        }
    }
}

impl<'a> RustAutoOpaqueCodecSseTy<'a> {
    fn should_generate(&self, lang: &Lang) -> bool {
        !matches!(lang, Lang::RustLang(_))
    }
}
