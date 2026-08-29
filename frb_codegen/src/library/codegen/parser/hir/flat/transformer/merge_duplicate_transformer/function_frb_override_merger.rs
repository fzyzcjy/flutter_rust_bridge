use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;

pub(crate) struct FunctionFrbOverrideMerger;

impl BaseMerger for FunctionFrbOverrideMerger {
    fn merge_functions(
        &self,
        _base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        if (overrider.sources).contains(&HirGenerationSource::FromFrbOverride) {
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
            item_fn: GeneralizedItemFn::ItemFn(syn::parse_str("fn original() {}").unwrap()),
        }
    }

    /// Selects an override function only when it carries the FRB override source.
    #[test]
    fn merges_function_from_frb_override_source() {
        let base = function(vec![HirGenerationSource::Normal]);
        let overrider = function(vec![HirGenerationSource::FromFrbOverride]);

        let merged = FunctionFrbOverrideMerger.merge_functions(&base, &overrider);

        assert!(merged.is_some());
        assert_eq!(merged.unwrap().sources, overrider.sources);
    }

    /// Leaves a normal function unmerged when it is not an FRB override.
    #[test]
    fn does_not_merge_function_without_frb_override_source() {
        let base = function(vec![HirGenerationSource::Normal]);
        let overrider = function(vec![HirGenerationSource::Normal]);

        assert!(FunctionFrbOverrideMerger
            .merge_functions(&base, &overrider)
            .is_none());
    }
}
