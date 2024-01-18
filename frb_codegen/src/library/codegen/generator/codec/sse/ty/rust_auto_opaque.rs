use crate::codegen::generator::codec::sse::ty::rust_opaque::{
    generate_generalized_rust_opaque_decode, generate_generalized_rust_opaque_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for RustAutoOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        match lang {
            Lang::DartLang(_) => {
                let needs_move = self.ir.needs_move();
                Some(generate_generalized_rust_opaque_encode(
                    lang,
                    &format!("{needs_move}"),
                ))
            }
            Lang::RustLang(_) => None,
        }
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        match lang {
            Lang::DartLang(_) => Some(generate_generalized_rust_opaque_decode(
                lang,
                self.ir.clone().into(),
                self.ir.inner.codec,
                self.context,
            )),
            Lang::RustLang(_) => None,
        }
    }
}
