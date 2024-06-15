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
