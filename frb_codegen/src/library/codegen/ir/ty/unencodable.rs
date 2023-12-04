use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
use lazy_static::lazy_static;
use regex::Regex;

crate::ir! {
pub struct IrTypeUnencodable {
    pub namespace: Option<Namespace>,
    pub string: String,
    pub segments: Vec<NameComponent>,
}

/// A component of a fully qualified name and any type arguments for it
pub struct NameComponent {
    pub ident: String,
    pub args: Option<Args>,
}

pub enum Args {
    Generic(Vec<IrType>),
    Signature(Vec<IrType>),
}
}

impl IrTypeTrait for IrTypeUnencodable {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        _f: &mut F,
        _ir_context: &impl IrContext,
    ) {
    }

    fn safe_ident(&self) -> String {
        lazy_static! {
            static ref NEG_FILTER: Regex = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
        }
        NEG_FILTER.replace_all(&self.string, "").into_owned()
    }

    fn rust_api_type(&self) -> String {
        self.string.clone()
    }

    fn self_namespace(&self) -> Option<Namespace> {
        self.namespace.clone()
    }
}
