use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
#[no_serde]
pub struct IrTypeOptional {
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeOptional {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        self.inner.visit_types(f, ir_pack);
    }

    fn safe_ident(&self) -> String {
        format!("opt_{}", self.inner.safe_ident())
    }
}
