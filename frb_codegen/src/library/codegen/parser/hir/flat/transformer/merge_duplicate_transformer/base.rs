use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;

pub(crate) trait BaseMerger {
    fn merge_functions(&self, a: HirFlatFunction, b: HirFlatFunction) -> Option<HirFlatFunction>;

    fn merge_struct_or_enums<Item: SynItemStructOrEnum>(
        &self,
        a: HirFlatStructOrEnum<Item>,
        b: HirFlatStructOrEnum<Item>,
    ) -> Vec<HirFlatStructOrEnum<Item>>;
}
