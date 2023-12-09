use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for OptionalCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let self_is_not_null = match lang {
            Lang::DartLang(_) => "self != null",
            Lang::RustLang(_) => "self.is_some()",
        };

        Some(format!(
            "
            {};
            if ({self_is_not_null}) {{
                {};
            }}
            ",
            lang.call_encode(&Primitive(IrTypePrimitive::Bool), &self_is_not_null),
            lang.call_encode(&*self.ir.inner, "self"),
        ))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let null = match lang {
            Lang::DartLang(_) => "null",
            Lang::RustLang(_) => "None",
        };
        Some(format!(
            "
            if ({}) {{
                return {};
            }} else {{
                return {null};
            }}
            ",
            lang.call_decode(&Primitive(IrTypePrimitive::Bool)),
            lang.call_decode(&*self.ir.inner),
        ))
    }
}
