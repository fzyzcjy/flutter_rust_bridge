use proc_macro2::Ident;
use crate::codegen::ir::hir::hierarchical::misc::HirCommon;
use crate::utils::namespace::Namespace;
use serde::Serialize;
use syn::{ItemImpl, ItemTrait};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirTrait {
    pub(crate) namespace: Namespace,
    #[serde(skip_serializing)]
    pub(crate) item_trait: ItemTrait,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirTraitImpl {
    pub(crate) namespace: Namespace,
    #[serde(skip_serializing)]
    pub(crate) item_impl: ItemImpl,
}

impl HirCommon for HirTrait {
    fn with_namespace(&self, namespace: Namespace) -> Self {
        Self {
            namespace,
            ..self.to_owned()
        }
    }

    fn ident(&self) -> Ident {
        self.item_trait.ident.clone()
    }
}
