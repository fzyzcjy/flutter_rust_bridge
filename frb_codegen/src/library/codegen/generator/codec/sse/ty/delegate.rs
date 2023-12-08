use crate::codegen::generator::codec::sse::lang::*;
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DelegateCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        let gen = SimpleDelegateCodecCodegen::new(lang, self.ir.get_delegate());
        match self.ir {
            IrTypeDelegate::Array(inner) => TODO,
            IrTypeDelegate::String => gen.encode("utf8.encoder.convert(src)"),
            IrTypeDelegate::StringList => {}
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(inner) => {}
            IrTypeDelegate::PrimitiveEnum(inner) => {}
            IrTypeDelegate::Time(inner) => {}
            IrTypeDelegate::TimeList(inner) => {}
            IrTypeDelegate::Uuid => {}
            IrTypeDelegate::Uuids => {}
            IrTypeDelegate::Backtrace => {}
            IrTypeDelegate::Anyhow => {}
        }
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        todo!()
    }
}

pub(super) struct SimpleDelegateCodecCodegen<'a> {
    lang: &'a Lang,
    inner_ty: IrType,
}

impl<'a> SimpleDelegateCodecCodegen<'a> {
    pub fn new(lang: &'a Lang, inner_ty: IrType) -> Self {
        Self { lang, inner_ty }
    }

    pub(super) fn encode(&self, inner_expr: &str) -> String {
        format!("{};", self.lang.call_encode(&self.inner_ty, inner_expr))
    }

    pub(super) fn decode(&self, wrapper_expr: &str) -> String {
        format!(
            "{var_decl} inner = {};
            return {wrapper_expr};",
            self.lang.call_decode(&self.inner_ty),
            var_decl = self.lang.var_decl()
        )
    }
}
