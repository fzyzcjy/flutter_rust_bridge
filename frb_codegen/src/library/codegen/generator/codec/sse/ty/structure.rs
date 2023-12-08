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
        todo!()
    }
}
