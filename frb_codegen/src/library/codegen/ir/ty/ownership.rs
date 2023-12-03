use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
use serde::Serialize;

crate::ir! {
pub struct IrTypeOwnership {
    pub mode: IrTypeOwnershipMode,
    pub inner: Box<IrType>,
}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub enum IrTypeOwnershipMode {
    /// "T"
    Owned,
    /// "&T"
    Ref,
    /// "&mut T"
    RefMut,
}

impl IrTypeTrait for IrTypeOwnership {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.inner.visit_types(f, ir_context)
    }

    fn safe_ident(&self) -> String {
        unreachable!()
    }

    fn rust_api_type(&self) -> String {
        unreachable!()
    }
}
