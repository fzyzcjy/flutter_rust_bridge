use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;

/// Merge the "default implementation in trait definition" with the overriden implementation in trait impl.
pub(crate) struct TraitDefDefaultImplMerger;

impl BaseMerger for TraitDefDefaultImplMerger {
    fn merge_functions(
        &self,
        base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        if (base.sources).contains(&HirGenerationSource::CopyFromTraitDef)
            && overrider.sources.contains(&HirGenerationSource::Normal)
        {
            Some(overrider.to_owned())
        } else {
            None
        }
    }

    fn merge_structs(
        &self,
        _base: &HirFlatStruct,
        _overrider: &HirFlatStruct,
    ) -> Option<HirFlatStruct> {
        None
    }

    fn merge_enums(&self, _base: &HirFlatEnum, _overrider: &HirFlatEnum) -> Option<HirFlatEnum> {
        None
    }

    // Does not care about this empty impl, since it does nothing
    // frb-coverage:ignore-start
    fn merge_traits(
        &self,
        _base: &HirFlatTrait,
        _overrider: &HirFlatTrait,
    ) -> Option<HirFlatTrait> {
        None
    }
    // frb-coverage:ignore-end
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::flat::function::HirFlatFunctionOwner;
    use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
    use crate::utils::namespace::Namespace;

    fn function(sources: Vec<HirGenerationSource>) -> HirFlatFunction {
        HirFlatFunction {
            namespace: Namespace::default(),
            owner: HirFlatFunctionOwner::Function,
            sources,
            item_fn: GeneralizedItemFn::ItemFn(syn::parse_str("fn implementation() {}").unwrap()),
        }
    }

    /// Replaces a copied trait default with the normal implementation.
    #[test]
    fn merges_copied_trait_default_with_normal_implementation() {
        let base = function(vec![HirGenerationSource::CopyFromTraitDef]);
        let overrider = function(vec![HirGenerationSource::Normal]);

        let merged = TraitDefDefaultImplMerger.merge_functions(&base, &overrider);

        assert_eq!(merged.unwrap().sources, overrider.sources);
    }

    /// Rejects a default implementation when the replacement is not normal source code.
    #[test]
    fn does_not_merge_non_normal_replacement() {
        let base = function(vec![HirGenerationSource::CopyFromTraitDef]);
        let overrider = function(vec![HirGenerationSource::FromFrbOverride]);

        assert!(TraitDefDefaultImplMerger
            .merge_functions(&base, &overrider)
            .is_none());
    }
}
