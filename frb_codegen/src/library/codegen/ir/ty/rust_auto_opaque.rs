use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeModifier, IrTypeTrait};

crate::ir! {
pub struct IrTypeRustAutoOpaque {
    pub namespace: Namespace,
    pub modifier: IrTypeModifier,
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeRustAutoOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        _f: &mut F,
        _ir_context: &impl IrContext,
    ) {
    }

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
