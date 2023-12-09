use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DartOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_encode(
            lang,
            &DART_OPAQUE_WIRE_TYPE,
            "wire.dart_opaque_dart2rust_encode(self)",
        ))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_decode(
            lang,
            &DART_OPAQUE_WIRE_TYPE,
            "inner",
        ))
    }
}

pub(super) const DART_OPAQUE_WIRE_TYPE: IrType = Primitive(IrTypePrimitive::Usize);
