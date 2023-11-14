use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
#[no_serde]
pub struct IrTypeOptionalList {
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeOptionalList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.inner.visit_types(f, ir_file)
    }

    fn safe_ident(&self) -> String {
        format!("list_opt_{}", self.inner.safe_ident())
    }
}
