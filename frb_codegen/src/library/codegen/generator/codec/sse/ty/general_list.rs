use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for GeneralListCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(general_list_generate_encode(lang, &self.ir.inner))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(general_list_generate_decode(lang, &self.ir.inner))
    }
}

pub(super) fn general_list_generate_encode(lang: &Lang, ir_inner: &IrType) -> String {
    lang.for_loop(
        "item",
        "self",
        &format!("{};", lang.call_encode(ir_inner, "item")),
    )
}

pub(super) fn general_list_generate_decode(lang: &Lang, ir_inner: &IrType) -> String {
    format!(
        "
        {var_decl} ans;
        {}
        return ans;
        ",
        lang.for_loop(
            "item",
            "self",
            &format!("ans.push({});", lang.call_decode(ir_inner))
        ),
        var_decl = lang.var_decl(),
    )
}
