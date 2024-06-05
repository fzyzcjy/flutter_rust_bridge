use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;

/// Merge the "default implementation in trait definition" with the overriden implementation in trait impl.
pub(crate) struct TraitDefDefaultImplMerger;

impl BaseMerger for TraitDefDefaultImplMerger {
    fn merge_functions(
        &self,
        base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        todo!()
    }

    fn merge_struct_or_enums<Item: SynItemStructOrEnum>(
        &self,
        base: &HirFlatStructOrEnum<Item>,
        overrider: &HirFlatStructOrEnum<Item>,
    ) -> Option<HirFlatStructOrEnum<Item>> {
        todo!()
    }
}
