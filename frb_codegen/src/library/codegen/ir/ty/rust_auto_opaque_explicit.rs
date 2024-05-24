use super::rust_opaque::RUST_OPAQUE_AS_PRIMITIVE;
use crate::codegen::ir::func::OwnershipMode;
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::rust_opaque::{IrTypeRustOpaque, NameComponent};
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeRustAutoOpaqueExplicit {
    pub inner: IrTypeRustOpaque,
}
}

impl IrTypeTrait for IrTypeRustAutoOpaqueExplicit {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        IrType::RustOpaque(self.inner.clone()).visit_types(f, ir_context)
    }

    fn safe_ident(&self) -> String {
        format!("Auto_Explicit_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        "TODO_IrTypeRustAutoOpaqueExplicit_rust_api_type".to_owned()
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.inner.namespace.clone())
    }

    fn as_primitive(&self) -> Option<&IrTypePrimitive> {
        Some(&RUST_OPAQUE_AS_PRIMITIVE)
    }
}
