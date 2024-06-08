use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;

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

    fn merge_traits(&self, base: &HirFlatTrait, overrider: &HirFlatTrait) -> Option<HirFlatTrait>;
}
