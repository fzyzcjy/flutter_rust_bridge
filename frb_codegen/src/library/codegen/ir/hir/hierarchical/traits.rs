use crate::codegen::ir::hir::hierarchical::misc::HirCommon;
use crate::utils::namespace::Namespace;
use proc_macro2::Ident;
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

    fn ident(&self) -> Ident {
        self.item_trait.ident.clone()
    }
}

fn serialize_item_trait<S: Serializer>(x: &ItemTrait, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&format!("ItemTrait(ident={})", x.ident))
}

pub(super) fn serialize_item_impl<S: Serializer>(x: &ItemImpl, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&format!("ItemImpl(self_ty={})", ty_to_string(&x.self_ty)))
}

fn ty_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string()
}
