use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;

#[derive(Debug, Clone, Default)]
pub(crate) struct HirFlatPack {
    pub functions: Vec<HirFlatFunction>,
    pub enums: Vec<HirFlatEnum>,
    pub structs: Vec<HirFlatStruct>,
    pub type_alias: Vec<HirFlatTypeAlias>,
}
