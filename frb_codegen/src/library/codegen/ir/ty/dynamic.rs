use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeDynamic;
}

impl IrTypeTrait for IrTypeDynamic {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        _f: &mut F,
        _ir_context: &impl IrContext,
    ) {
    }

    fn safe_ident(&self) -> String {
        "dartabi".to_owned()
    }

    fn rust_api_type(&self) -> String {
        "flutter_rust_bridge::for_generated::DartAbi".to_owned()
    }
}
