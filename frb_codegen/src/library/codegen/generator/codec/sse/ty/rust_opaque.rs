use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for RustOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(generate_generalized_rust_opaque_encode(lang, "null"))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let var_decl = lang.var_decl();
        Some(match lang {
            Lang::DartLang(_) => {
                format!(
                    "
                    {var_decl} ptr = {};
                    {var_decl} externalSizeOnNative = {};
                    return {}.sseDecode(ptr, externalSizeOnNative);
                    ",
                    lang.call_decode(&self.ir.get_delegate()),
                    lang.call_decode(&EXTERNAL_SIZE_TYPE),
                    lang.null(),
                )
            }
            Lang::RustLang(_) => simple_delegate_decode(lang, &self.ir.get_delegate(), "inner"),
        })
    }
}

const EXTERNAL_SIZE_TYPE: IrType = IrType::Primitive(IrTypePrimitive::I32);

pub(super) fn generate_generalized_rust_opaque_encode(lang: &Lang, needs_move: &str) -> String {
    match lang {
        Lang::DartLang(_) => simple_delegate_encode(
            lang,
            &IrTypeRustOpaque::DELEGATE_TYPE,
            &format!("self.sseEncode(move: {needs_move})"),
        ),
        Lang::RustLang(_) => {
            format!(
                "
                {};
                {};
                ",
                lang.call_encode(&IrTypeRustOpaque::DELEGATE_TYPE, "TODO"),
                lang.call_encode(&EXTERNAL_SIZE_TYPE, "TODO"),
            )
        }
    }
}
