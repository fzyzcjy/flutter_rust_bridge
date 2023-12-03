use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeDartOpaque;
}

impl IrTypeTrait for IrTypeDartOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        _f: &mut F,
        _ir_context: &impl IrContext,
    ) {
        // do nothing.
    }

    fn safe_ident(&self) -> String {
        "DartOpaque".to_owned()
    }

    fn rust_api_type(&self) -> String {
        "flutter_rust_bridge::DartOpaque".to_owned()
    }
}
