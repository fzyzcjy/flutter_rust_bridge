use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeOptionalList {
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeOptionalList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.inner.visit_types(f, ir_context)
    }

    fn safe_ident(&self) -> String {
        format!("list_opt_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<Option<{}>>", self.inner.rust_api_type())
    }
}
