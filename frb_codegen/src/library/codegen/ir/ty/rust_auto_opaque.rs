use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::rust_opaque::{IrTypeRustOpaque, NameComponent};
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
use serde::Serialize;

crate::ir! {
pub struct IrTypeRustAutoOpaque {
    pub ownership_mode: OwnershipMode,
    pub inner: IrTypeRustOpaque,
    pub raw: IrRustAutoOpaqueRaw,
}

/// Original type without any transformation
pub struct IrRustAutoOpaqueRaw {
    pub string: String,
    pub segments: Vec<NameComponent>,
}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, strum_macros::Display)]
pub enum OwnershipMode {
    /// "T"
    Owned,
    /// "&T"
    Ref,
    /// "&mut T"
    RefMut,
}

impl IrTypeTrait for IrTypeRustAutoOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        IrType::RustOpaque(self.inner.clone()).visit_types(f, ir_context)
    }

    fn safe_ident(&self) -> String {
        format!("Auto_{}_{}", self.ownership_mode, self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        match self.ownership_mode {
            // Different mechanisms for Owned vs Ref/RefMut
            OwnershipMode::Owned => self.raw.string.clone(),
            OwnershipMode::Ref | OwnershipMode::RefMut => self.inner.rust_api_type(),
        }
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.inner.namespace.clone())
    }
}

impl IrTypeRustAutoOpaque {
    pub(crate) fn needs_move(&self) -> bool {
        self.ownership_mode == OwnershipMode::Owned
    }
}
