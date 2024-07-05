use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::misc::serializers::serialize_generalized_item_fn;
use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::utils::namespace::{Namespace, NamespacedName};
use crate::utils::syn_utils::ty_to_string;
use serde::Serialize;
use syn::Visibility;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFlatFunction {
    pub(crate) namespace: Namespace,
    pub(crate) owner: HirFlatFunctionOwner,
    pub(crate) sources: Vec<HirGenerationSource>,
    #[serde(serialize_with = "serialize_generalized_item_fn")]
    pub(crate) item_fn: GeneralizedItemFn,
}

impl HirFlatComponent<SimpleOwnerAndName> for HirFlatFunction {
    fn sort_key(&self) -> SimpleOwnerAndName {
        self.owner_and_name_for_dedup()
    }
}

impl HirFlatFunction {
    pub(crate) fn owner_and_name_for_dedup(&self) -> SimpleOwnerAndName {
        (self.owner_for_dedup(), self.name_for_dedup())
    }

    pub(crate) fn owner_for_dedup(&self) -> String {
        match &self.owner {
            HirFlatFunctionOwner::Function => self.namespace.joined_path.clone(),
            HirFlatFunctionOwner::StructOrEnum { impl_ty, .. } => ty_to_string(impl_ty),
            HirFlatFunctionOwner::TraitDef { trait_def_name } => trait_def_name.name.clone(),
        }
    }

    pub(crate) fn name_for_dedup(&self) -> String {
        let attributes = FrbAttributes::parse(self.item_fn.attrs()).unwrap();
        attributes.name().unwrap_or_else(|| self.item_fn.name())
    }

    pub(crate) fn is_public(&self) -> Option<bool> {
        match self.owner {
            HirFlatFunctionOwner::Function
            | HirFlatFunctionOwner::StructOrEnum {
                trait_def_name: None,
                ..
            } => (self.item_fn.vis_raw()).map(|vis| matches!(vis, Visibility::Public(_))),
            HirFlatFunctionOwner::TraitDef { .. }
            | HirFlatFunctionOwner::StructOrEnum {
                trait_def_name: Some(_),
                ..
            } => None,
        }
    }

    pub(crate) fn is_async(&self) -> bool {
        self.item_fn.sig().asyncness.is_some()
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
