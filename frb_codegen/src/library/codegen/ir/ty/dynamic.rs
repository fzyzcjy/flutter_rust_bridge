use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeDynamic;
}

impl IrTypeTrait for IrTypeDynamic {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_file: &IrFile) {}

    fn safe_ident(&self) -> String {
        "dartabi".to_owned()
    }
}