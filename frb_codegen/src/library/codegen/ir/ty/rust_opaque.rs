use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use regex::Regex;

crate::ir! {
pub struct IrTypeRustOpaqueRef {
    pub ident: IrRustOpaqueIdent,
}

pub struct IrRustOpaqueIdent(pub NamespacedName);

pub struct IrRustOpaque {
    pub name: NamespacedName,
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeRustOpaqueRef {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_pack: &IrPack) {}

    fn safe_ident(&self) -> String {
        format!("RustOpaque_{}", self.ident.0.name)
    }

    fn rust_api_type(&self) -> String {
        format!(
            "flutter_rust_bridge::RustOpaque<{}>",
            self.inner.rust_api_type()
        )
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.ident.0.namespace.clone())
    }
}

impl IrRustOpaque {
    pub fn new(namespace: Namespace, inner: IrType) -> Self {
        Self {
            name: NamespacedName::new(namespace, inner.safe_ident()),
            inner: Box::new(inner),
        }
    }
}
