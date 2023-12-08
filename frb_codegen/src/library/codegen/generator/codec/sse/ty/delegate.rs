use crate::codegen::generator::codec::sse::lang::*;
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DelegateCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        let inner_expr = match &self.ir {
            IrTypeDelegate::Array(inner) => TODO,
            IrTypeDelegate::String => "utf8.encoder.convert(src)",
            IrTypeDelegate::PrimitiveEnum(inner) => {}
            IrTypeDelegate::Time(inner) => {}
            IrTypeDelegate::Uuid => {}
            IrTypeDelegate::Backtrace => {}
            IrTypeDelegate::Anyhow => {}
        };
        simple_delegate_encode(lang, &self.ir.get_delegate(), inner_expr)
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        let wrapper_expr = match &self.ir {
            IrTypeDelegate::Array(inner) => TODO,
            IrTypeDelegate::String => "TODO",
            IrTypeDelegate::PrimitiveEnum(inner) => {}
            IrTypeDelegate::Time(inner) => {}
            IrTypeDelegate::Uuid => {}
            IrTypeDelegate::Backtrace => {}
            IrTypeDelegate::Anyhow => {}
        };
        simple_delegate_decode(lang, &self.ir.get_delegate(), wrapper_expr)
    }
}

pub(super) fn simple_delegate_encode(lang: &Lang, inner_ty: &IrType, inner_expr: &str) -> String {
    format!("{};", lang.call_encode(inner_ty, inner_expr))
}

pub(super) fn simple_delegate_decode(lang: &Lang, inner_ty: &IrType, wrapper_expr: &str) -> String {
    format!(
        "{var_decl} inner = {};
        return {wrapper_expr};",
        lang.call_decode(&inner_ty),
        var_decl = lang.var_decl()
    )
}
