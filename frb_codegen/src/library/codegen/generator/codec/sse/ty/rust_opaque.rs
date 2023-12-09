use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for RustOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_encode(
            lang,
            &self.ir.get_delegate(),
            "self.sseEncode()",
        ))
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
                    lang.call_decode(&Primitive(IrTypePrimitive::Bool)),
                    lang.call_decode(&*self.ir.inner),
                    lang.null(),
                )
            }
            Lang::RustLang(_) => simple_delegate_decode(lang, &self.ir.get_delegate(), "inner"),
        })
    }
}
