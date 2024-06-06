use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;

pub(crate) struct FunctionFrbOverrideMerger;

impl BaseMerger for FunctionFrbOverrideMerger {
    fn merge_functions(
        &self,
        base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        TODO
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
}
