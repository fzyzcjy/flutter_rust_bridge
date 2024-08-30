use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DartOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_encode(
            lang,
            &self.mir.get_delegate(),
            match lang {
                Lang::DartLang(_) => {
                    "PlatformPointerUtil.ptrToPlatformInt64(encodeDartOpaque(self, portManager.dartHandlerPort, generalizedFrbRustBinding))"
                }
                Lang::RustLang(_) => "self.encode()",
            },
        ))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_decode(
            lang,
            &self.mir.get_delegate(),
            match lang {
                Lang::DartLang(_) => "decodeDartOpaque(inner, generalizedFrbRustBinding)",
                Lang::RustLang(_) => {
                    "unsafe { flutter_rust_bridge::for_generated::sse_decode_dart_opaque(inner) }"
                }
            },
        ))
    }
}
