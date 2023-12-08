use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::rust_opaque::RUST_OPAQUE_WIRE_TYPE;
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for RustAutoOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        let needs_move = self.ir.needs_move();
        simple_delegate_encode(
            lang,
            &RUST_OPAQUE_WIRE_TYPE,
            &format!("src.sseEncode(move: {needs_move})"),
        )
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        simple_delegate_decode(lang, &RUST_OPAQUE_WIRE_TYPE, "inner")
    }
}
