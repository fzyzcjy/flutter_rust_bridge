use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DartOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(match lang {
            Lang::DartLang(_) => {
                simple_delegate_encode(
                    lang,
                    &MirType::Primitive(MirTypePrimitive::Isize),
                    "PlatformPointerUtil.ptrToPlatformInt64(encodeDartOpaque(self, portManager.dartHandlerPort, generalizedFrbRustBinding))",
                )
            }
            Lang::RustLang(_) => {
                simple_delegate_encode(
                    lang,
                    &MirType::Primitive(MirTypePrimitive::Usize),
                    "self.encode()",
                )
            }
        })
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(match lang {
            Lang::DartLang(_) => simple_delegate_decode(
                lang,
                &MirType::Primitive(MirTypePrimitive::Isize),
                "decodeDartOpaque(inner, generalizedFrbRustBinding)",
            ),
            Lang::RustLang(_) => simple_delegate_decode(
                lang,
                &MirType::Primitive(MirTypePrimitive::Usize),
                "unsafe { flutter_rust_bridge::for_generated::sse_decode_dart_opaque(inner) }",
            ),
        })
    }
}
