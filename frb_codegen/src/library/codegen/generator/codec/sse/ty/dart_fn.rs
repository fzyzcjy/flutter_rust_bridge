use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DartFnCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        simple_delegate_encode(lang, &self.ir.get_delegate(), "src")
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        simple_delegate_decode(lang, &self.ir.get_delegate(), "inner")
    }
}
