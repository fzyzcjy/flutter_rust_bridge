use crate::codegen::ir::hir::hierarchical::misc::HirCommon;
use crate::codegen::ir::hir::misc::item_fn::{serialize_generalized_item_fn, GeneralizedItemFn};
use crate::utils::namespace::{Namespace, NamespacedName};
use proc_macro2::Span;
use serde::{Serialize, Serializer};
use syn::spanned::Spanned;
use syn::{Attribute, ImplItemFn, ItemFn, ItemImpl, Signature, TraitItemFn, Visibility};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFlatFunction {
    pub(crate) namespace: Namespace,
    pub(crate) owner: HirFlatFunctionOwner,
    #[serde(serialize_with = "serialize_generalized_item_fn")]
    pub(crate) item_fn: GeneralizedItemFn,
}

impl HirFlatFunction {
    pub(crate) fn owner_and_name(&self) -> SimpleOwnerAndName {
        (self.owner.simple_name(), self.item_fn.name())
    }

    pub(crate) fn is_public(&self) -> Option<bool> {
        match self.owner {
            HirFlatFunctionOwner::Function
            | HirFlatFunctionOwner::Method {
                trait_def_name: None,
                ..
            } => (self.item_fn.vis()).map(|vis| matches!(vis, Visibility::Public(_))),
            HirFlatFunctionOwner::Method {
                trait_def_name: Some(_),
                ..
            } => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) enum HirFlatFunctionOwner {
    Function,
    Method {
        #[serde(serialize_with = "serialize_item_impl")]
        item_impl: ItemImpl,
        trait_def_name: Option<NamespacedName>,
    },
}

impl HirFlatFunctionOwner {
    pub(crate) fn simple_name(&self) -> Option<String> {
        match self {
            Self::Function => None,
            Self::Method { item_impl, .. } => Some(ty_to_string(&item_impl.self_ty)),
        }
    }
}

pub(crate) type SimpleOwnerAndName = (Option<String>, String);

fn ty_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string()
}