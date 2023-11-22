use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeRustOpaque {
    pub namespace: Namespace,
    pub inner: Box<IrType>,
}
}

impl IrTypeRustOpaque {
    pub fn new(namespace: Namespace, inner: IrType) -> Self {
        Self {
            namespace,
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
