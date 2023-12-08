use crate::codegen::generator::codec::sse::lang::*;
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for DelegateCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        let gen = SimpleDelegateCodecCodegen::new(lang);
        match self.ir {
            IrTypeDelegate::Array(inner) => gen.encode(),
            IrTypeDelegate::String => {}
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
}

impl<'a> SimpleDelegateCodecCodegen<'a> {
    pub fn new(lang: &'a Lang) -> Self {
        Self { lang }
    }

    pub(super) fn encode(&self, inner_ty: &IrType, inner_name: &str) -> String {
        format!("{};", self.lang.call_encode(inner_ty, inner_name))
    }

    pub(super) fn decode(&self, inner_ty: &IrType, wrapper: &str) -> String {
        format!(
            "{var_decl} inner = {};
            return {wrapper};",
            self.lang.call_decode(inner_ty),
            var_decl = self.lang.var_decl()
        )
    }
}
