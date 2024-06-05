use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;

pub(crate) trait BaseMerger {
    fn merge_functions(
        &self,
        base: HirFlatFunction,
        overrider: HirFlatFunction,
    ) -> Option<HirFlatFunction>;

    fn merge_struct_or_enums<Item: SynItemStructOrEnum>(
        &self,
        base: HirFlatStructOrEnum<Item>,
        overrider: HirFlatStructOrEnum<Item>,
    ) -> Vec<HirFlatStructOrEnum<Item>>;
}
