use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for RustOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_encode(
            lang,
            &RUST_OPAQUE_WIRE_TYPE,
            "self.sseEncode()",
        ))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_decode(
            lang,
            &RUST_OPAQUE_WIRE_TYPE,
            "inner",
        ))
    }
}

pub(super) const RUST_OPAQUE_WIRE_TYPE: IrType = Primitive(IrTypePrimitive::Usize);
