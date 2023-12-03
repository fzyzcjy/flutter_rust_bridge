use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeRustAutoOpaque {
    pub namespace: Namespace,
    pub inner: Box<IrType>,
}
}

impl IrTypeRustAutoOpaque {
    pub fn new(namespace: Namespace, inner: IrType) -> Self {
        Self {
            namespace,
            inner: Box::new(inner),
        }
    }
}

impl IrTypeTrait for IrTypeRustAutoOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_pack: &IrPack) {}

    fn safe_ident(&self) -> String {
        format!("RustAutoOpaque_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        self.inner.rust_api_type()
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.namespace.clone())
    }
}
