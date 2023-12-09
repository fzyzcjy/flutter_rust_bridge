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
    let len_func = match lang {
        Lang::DartLang(_) => "length",
        Lang::RustLang(_) => "len()",
    };

    format!(
        "{};
        {}",
        lang.call_encode(&LEN_TYPE, &format!("self.{}", len_func)),
        lang.for_loop(
            "item",
            "self",
            &format!("{};", lang.call_encode(ir_inner, "item")),
        )
    )
}

pub(super) fn general_list_generate_decode(lang: &Lang, ir_inner: &IrType) -> String {
    let var_decl = lang.var_decl();

    format!(
        "
        {var_decl} len_ = {};
        {var_decl} ans_;
        {}
        return ans_;
        ",
        lang.call_decode(&LEN_TYPE),
        lang.for_loop(
            "item",
            "self",
            &format!("ans_.push({});", lang.call_decode(ir_inner))
        ),
    )
}

const LEN_TYPE: IrType = Primitive(IrTypePrimitive::I32);
