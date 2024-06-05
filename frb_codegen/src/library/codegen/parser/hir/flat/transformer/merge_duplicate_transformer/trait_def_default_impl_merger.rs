use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::struct_or_enum::{
    HirFlatEnum, HirFlatStruct, HirFlatStructOrEnum,
};
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;

/// Merge the "default implementation in trait definition" with the overriden implementation in trait impl.
pub(crate) struct TraitDefDefaultImplMerger;

impl BaseMerger for TraitDefDefaultImplMerger {
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

    fn merge_structs(
        &self,
        base: &HirFlatStruct,
        overrider: &HirFlatStruct,
    ) -> Option<HirFlatStruct> {
        None
    }

    fn merge_enums(&self, base: &HirFlatEnum, overrider: &HirFlatEnum) -> Option<HirFlatEnum> {
        None
    }
}
