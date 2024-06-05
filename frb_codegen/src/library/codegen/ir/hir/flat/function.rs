use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::misc::serializers::serialize_generalized_item_fn;
use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use crate::utils::namespace::{Namespace, NamespacedName};
use crate::utils::syn_utils::ty_to_string;
use serde::Serialize;
use syn::Visibility;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFlatFunction {
    pub(crate) namespace: Namespace,
    pub(crate) owner: HirFlatFunctionOwner,
    pub(crate) source: HirFlatFunctionSource,
    #[serde(serialize_with = "serialize_generalized_item_fn")]
    pub(crate) item_fn: GeneralizedItemFn,
}

impl HirFlatFunction {
    pub(crate) fn owner_and_name_for_dedup(&self) -> SimpleOwnerAndName {
        (self.owner_for_dedup(), self.item_fn.name())
    }

    pub(crate) fn owner_for_dedup(&self) -> String {
        match self {
            Self::Function => self.namespace.joined_path,
            Self::StructOrEnum { impl_ty, .. } => ty_to_string(impl_ty),
            Self::TraitDef { trait_def_name } => trait_def_name.name.clone(),
        }
    }

    pub(crate) fn is_public(&self) -> Option<bool> {
        match self.owner {
            HirFlatFunctionOwner::Function
            | HirFlatFunctionOwner::StructOrEnum {
                trait_def_name: None,
                ..
            } => (self.item_fn.vis()).map(|vis| matches!(vis, Visibility::Public(_))),
            HirFlatFunctionOwner::TraitDef { .. }
            | HirFlatFunctionOwner::StructOrEnum {
                trait_def_name: Some(_),
                ..
            } => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) enum HirFlatFunctionOwner {
    Function,
    StructOrEnum {
        #[serde(serialize_with = "serialize_syn")]
        impl_ty: syn::Type,
        trait_def_name: Option<String>,
    },
    TraitDef {
        trait_def_name: NamespacedName,
    },
}

pub(crate) type SimpleOwnerAndName = (String, String);

#[derive(Debug, Clone, Copy, Serialize)]
pub(crate) enum HirFlatFunctionSource {
    Normal,
    CopyFromTraitDef,
}
