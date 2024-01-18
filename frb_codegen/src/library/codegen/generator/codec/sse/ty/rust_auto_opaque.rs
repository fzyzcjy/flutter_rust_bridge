use crate::codegen::generator::codec::sse::ty::rust_opaque::{
    generate_generalized_rust_opaque_decode, generate_generalized_rust_opaque_encode,
};
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::body::generate_sse_encode_or_decode_impl;

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

    fn generate_extra(&self, lang: &Lang) -> String {
        match lang {
            Lang::DartLang(_) => "".to_owned(),
            Lang::RustLang(_) => {
                let arc = self.ir.inner.codec.arc_ty();
                generate_sse_encode_or_decode_impl(
                    &self.ir.raw.string,
                    &format!("flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, {arc}<_>>(self)"),
                    EncodeOrDecode::Encode,
                )
            }
        }
    }
}

impl<'a> RustAutoOpaqueCodecSseTy<'a> {
    fn should_generate(&self, lang: &Lang) -> bool {
        !matches!(lang, Lang::RustLang(_))
    }
}
