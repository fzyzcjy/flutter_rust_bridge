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

#[allow(clippy::large_enum_variant)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn function(owner: HirFlatFunctionOwner, item_fn: GeneralizedItemFn) -> HirFlatFunction {
        HirFlatFunction {
            namespace: Namespace::new_raw("crate::api".to_owned()),
            owner,
            sources: vec![],
            item_fn,
        }
    }

    /// Uses the FRB rename and namespace for a free-function deduplication key.
    #[test]
    fn deduplication_key_uses_free_function_namespace_and_rename() {
        let function = function(
            HirFlatFunctionOwner::Function,
            GeneralizedItemFn::ItemFn(
                syn::parse_str("#[frb(name = \"exposed\")] pub async fn original() {}").unwrap(),
            ),
        );

        assert_eq!(
            function.owner_and_name_for_dedup(),
            ("crate::api".to_owned(), "exposed".to_owned())
        );
        assert_eq!(function.sort_key(), function.owner_and_name_for_dedup());
        assert_eq!(function.is_public(), Some(true));
        assert!(function.is_async());
    }

    /// Uses the implementation type and raw visibility for inherent methods.
    #[test]
    fn inherent_method_deduplication_and_visibility_follow_implementation() {
        let function = function(
            HirFlatFunctionOwner::StructOrEnum {
                impl_ty: syn::parse_str("Widget").unwrap(),
                trait_def_name: None,
            },
            GeneralizedItemFn::ImplItemFn(syn::parse_str("fn build() {}").unwrap()),
        );

        assert_eq!(function.owner_for_dedup(), "Widget");
        assert_eq!(function.name_for_dedup(), "build");
        assert_eq!(function.is_public(), Some(false));
        assert!(!function.is_async());
    }

    /// Hides visibility for trait declarations and trait implementation methods.
    #[test]
    fn trait_owned_functions_do_not_report_public_visibility() {
        let trait_definition = function(
            HirFlatFunctionOwner::TraitDef {
                trait_def_name: NamespacedName::new(
                    Namespace::new_raw("crate::api".to_owned()),
                    "Service".to_owned(),
                ),
            },
            GeneralizedItemFn::TraitItemFn(syn::parse_str("async fn call();").unwrap()),
        );
        let trait_implementation = function(
            HirFlatFunctionOwner::StructOrEnum {
                impl_ty: syn::parse_str("ServiceImpl").unwrap(),
                trait_def_name: Some("Service".to_owned()),
            },
            GeneralizedItemFn::ImplItemFn(syn::parse_str("pub fn call() {}").unwrap()),
        );

        assert_eq!(trait_definition.owner_for_dedup(), "Service");
        assert_eq!(trait_definition.is_public(), None);
        assert!(trait_definition.is_async());
        assert_eq!(trait_implementation.is_public(), None);
    }
}
