use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeRustOpaque {
    pub inner: Box<IrType>,
}
}

impl IrTypeRustOpaque {
    pub fn new(inner: IrType) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl IrTypeTrait for IrTypeRustOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_pack: &IrPack) {}

    fn safe_ident(&self) -> String {
        format!("RustOpaque_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("RustOpaque<{}>", self.inner.rust_api_type())
    }
}
