use crate::codegen::generator::codec::sse::ty::*;
use itertools::Itertools;

impl<'a> CodecSseTyTrait for StructRefCodecSseTy<'a> {
    fn generate_encode(&self, lang: &impl Lang) -> String {
        let st = self.ir.get(self.context.ir_pack);
        st.fields
            .iter()
            .map(|field| {
                format!(
                    "{};\n",
                    lang.call_encode(&field.ty, &format!("src.{}", field.name))
                )
            })
            .join("")
    }

    fn generate_decode(&self, lang: &impl Lang) -> String {
        let st = self.ir.get(self.context.ir_pack);
        let decode_fields = st
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{} {} = {};\n",
                    lang.var_decl(),
                    field.name,
                    lang.call_decode(&field.ty)
                )
            })
            .join("");
        format!(
            "
            {decode_fields}
            return {};
            ",
            lang.call_constructor(
                &st.name.name,
                &st.fields.iter().map(|x| x.name.raw.clone()).collect_vec()
            )
        )
    }
}
