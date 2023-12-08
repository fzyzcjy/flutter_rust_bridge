use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for GeneralListCodecSseTy<'a> {
    fn generate_encode(&self, lang: &impl Lang) -> String {
        lang.for_loop(
            "item",
            "src",
            &format!("{};", lang.call_encode(&self.ir.inner, "item")),
        )
    }

    fn generate_decode(&self, lang: &impl Lang) -> String {
        format!(
            "
            {var_decl} ans;
            {}
            return ans;
            ",
            lang.for_loop(
                "item",
                "src",
                &format!("ans.push({});", lang.call_decode(&self.ir.inner))
            ),
            var_decl = lang.var_decl(),
        )
    }
}
