use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for OptionalCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let self_is_not_null = match lang {
            Lang::DartLang(_) => "self != null",
            Lang::RustLang(_) => "self.is_some()",
        };
        let encode_flag = lang.call_encode(&Primitive(MirTypePrimitive::Bool), self_is_not_null);

        Some(match lang {
            Lang::DartLang(_) => format!(
                "
                {encode_flag};
                if (self != null) {{
                    {};
                }}
                ",
                lang.call_encode(&self.mir.inner, "self"),
            ),
            Lang::RustLang(_) => format!(
                "
                {encode_flag};
                if let Some(value) = self {{
                    {};
                }}
                ",
                lang.call_encode(&self.mir.inner, "value"),
            ),
        })
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let wrapper = match lang {
            Lang::DartLang(_) => "",
            Lang::RustLang(_) => "Some",
        };
        let null = match lang {
            Lang::DartLang(_) => "null",
            Lang::RustLang(_) => "None",
        };

        Some(format!(
            "
            if ({}) {{
                return {wrapper}({});
            }} else {{
                return {null};
            }}
            ",
            lang.call_decode(&Primitive(MirTypePrimitive::Bool)),
            lang.call_decode(&self.mir.inner),
        ))
    }
}
