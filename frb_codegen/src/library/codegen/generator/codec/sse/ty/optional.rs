use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for OptionalCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        let src_is_not_null = format!("src != {}", lang.null());
        format!(
            "
            {};
            if ({src_is_not_null}) {{
                {};
            }}
            ",
            lang.call_encode(&Primitive(IrTypePrimitive::Bool), &src_is_not_null),
            lang.call_encode(&*self.ir.inner, "src"),
        )
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        format!(
            "
            if ({}) {{
                return {};
            }} else {{
                return {};
            }}
            ",
            lang.call_decode(&Primitive(IrTypePrimitive::Bool)),
            lang.call_decode(&*self.ir.inner),
            lang.null(),
        )
    }
}
