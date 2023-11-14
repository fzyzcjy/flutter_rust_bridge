use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
#[no_serde]
pub struct IrTypeGeneralList {
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeGeneralList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.inner.visit_types(f, ir_file);
    }

    fn safe_ident(&self) -> String {
        format!("list_{}", self.inner.safe_ident())
    }
}
