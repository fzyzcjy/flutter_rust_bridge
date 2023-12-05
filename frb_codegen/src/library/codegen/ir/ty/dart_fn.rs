use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeDartFn {
    // TODO
}
}

impl IrTypeTrait for IrTypeDartFn {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        _f: &mut F,
        _ir_context: &impl IrContext,
    ) {
        todo!()
    }

    fn safe_ident(&self) -> String {
        todo!()
    }

    fn rust_api_type(&self) -> String {
        todo!()
    }
}
