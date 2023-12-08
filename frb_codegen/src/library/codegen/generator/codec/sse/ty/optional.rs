use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for OptionalCodecSseTy<'a> {
    fn generate_encode(&self, lang: &impl Lang) -> String {
        let src_is_not_null = format!("src != {}", lang.null());
        format!(
            "
            {};
            if ({src_is_not_null}) {{
                {}
            }}
            ",
            lang.call_encode(&Primitive(IrTypePrimitive::Bool), &src_is_not_null),
            lang.call_encode(&*self.ir.inner, "src"),
        )
    }

    fn generate_decode(&self, lang: &impl Lang) -> String {
        format!(
            "
            if ({}) {{
                return {};
            }} else {{
                return {};
            }}
            ",
            lang.call_decode(&Primitive(IrTypePrimitive::Bool)),
            lang.call_encode(&*self.ir.inner, "src"),
            lang.null(),
        )
    }
}
