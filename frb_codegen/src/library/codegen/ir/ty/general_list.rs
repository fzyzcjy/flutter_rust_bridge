use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeGeneralList {
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeGeneralList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        self.inner.visit_types(f, ir_pack);
    }

    fn safe_ident(&self) -> String {
        format!("list_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.inner.rust_api_type())
    }
}
