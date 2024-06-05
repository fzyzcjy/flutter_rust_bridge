use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;

/// Merge the "default implementation in trait definition" with the overriden implementation in trait impl.
pub(crate) struct TraitDefDefaultImplMerger;

impl BaseMerger for TraitDefDefaultImplMerger {
    fn merge_functions(
        &self,
        base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction> {
        if let (HirFlatFunctionSource::CopyFromTraitDef, HirFlatFunctionSource::Normal) =
            (&base.source, &overrider.source)
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
}
