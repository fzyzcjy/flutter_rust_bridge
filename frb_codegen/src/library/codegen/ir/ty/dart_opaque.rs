use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeDartOpaque;
}

impl IrTypeTrait for IrTypeDartOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_pack: &IrPack) {
        // do nothing.
    }

    fn safe_ident(&self) -> String {
        "DartOpaque".to_owned()
    }
}
