use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{
    HirFlatEnum, HirFlatStruct, HirFlatStructOrEnum,
};
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;

pub(crate) trait BaseMerger {
    fn merge_functions(
        &self,
        base: &HirFlatFunction,
        overrider: &HirFlatFunction,
    ) -> Option<HirFlatFunction>;

    fn merge_structs(
        &self,
        base: &HirFlatStruct,
        overrider: &HirFlatStruct,
    ) -> Option<HirFlatStruct>;

    fn merge_enums(&self, base: &HirFlatEnum, overrider: &HirFlatEnum) -> Option<HirFlatEnum>;
}
