use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::Merger;

/// Merge the "default implementation in trait definition" with the overriden implementation in trait impl.
pub(crate) struct TraitDefDefaultImplMerger;

impl Merger for TraitDefDefaultImplMerger {
    fn merge_functions(
        &self,
        base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        if let (HirFlatFunctionOwner::StructOrEnum { .. }, HirFlatFunctionOwner::TraitDef { .. }) =
            (base.owner, overrider.owner)
        {
            Some(overrider.to_owned())
        } else {
            None
        }
    }

    fn merge_struct_or_enums<Item: SynItemStructOrEnum>(
        &self,
        base: &HirFlatStructOrEnum<Item>,
        overrider: &HirFlatStructOrEnum<Item>,
    ) -> Option<HirFlatStructOrEnum<Item>> {
        None
    }
}
