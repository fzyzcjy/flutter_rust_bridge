use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeRustOpaque {
    pub namespace: Namespace,
    pub inner: Box<IrType>,
    pub brief_name: bool,
}
}

impl IrTypeRustOpaque {
    pub fn new(namespace: Namespace, inner: IrType, brief_name: bool) -> Self {
        Self {
            namespace,
            inner: Box::new(inner),
            brief_name,
        }
    }

    pub(crate) fn get_delegate(&self) -> IrType {
        Self::DELEGATE_TYPE.clone()
    }

    pub(crate) const DELEGATE_TYPE: IrType = IrType::Primitive(IrTypePrimitive::Usize);
}

impl IrTypeTrait for IrTypeRustOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.get_delegate().visit_types(f, ir_context)
    }

    fn safe_ident(&self) -> String {
        format!("RustOpaque_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!(
            "flutter_rust_bridge::RustOpaque<{}>",
            self.inner.rust_api_type()
        )
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.namespace.clone())
    }
}
