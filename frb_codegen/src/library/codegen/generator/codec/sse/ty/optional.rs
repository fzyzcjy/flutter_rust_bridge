use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for OptionalCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        let self_is_not_null = format!("self != {}", lang.null());
        format!(
            "
            {};
            if ({self_is_not_null}) {{
                {};
            }}
            ",
            lang.call_encode(&Primitive(IrTypePrimitive::Bool), &self_is_not_null),
            lang.call_encode(&*self.ir.inner, "self"),
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
