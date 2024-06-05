use crate::codegen::ir::hir::hierarchical::misc::HirCommon;
use crate::utils::namespace::Namespace;
use quote::ToTokens;
use serde::{Serialize, Serializer};
use syn::{ItemImpl, ItemTrait};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirTrait {
    pub(crate) namespace: Namespace,
    #[serde(serialize_with = "serialize_item_trait")]
    pub(crate) item_trait: ItemTrait,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirTraitImpl {
    pub(crate) namespace: Namespace,
    #[serde(serialize_with = "serialize_item_impl")]
    pub(crate) item_impl: ItemImpl,
}

impl HirCommon for HirTrait {
    fn with_namespace(&self, namespace: Namespace) -> Self {
        Self {
            namespace,
            ..self.to_owned()
        }
    }

    fn name_for_use_stmt(&self) -> String {
        self.item_trait.ident.to_string()
    }
}

impl HirCommon for HirTraitImpl {
    fn with_namespace(&self, namespace: Namespace) -> Self {
        Self {
            namespace,
            ..self.to_owned()
        }
    }

    fn name_for_use_stmt(&self) -> String {
        ty_to_string(&self.item_impl.self_ty)
    }
}

fn ty_to_string<T: ToTokens>(ty: &T) -> String {
    quote::quote!(#ty).to_string()
}
