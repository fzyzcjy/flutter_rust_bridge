use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;

#[derive(Debug, Clone, Default)]
pub(crate) struct HirFlatPack {
    // components
    pub functions: Vec<HirFlatFunction>,
    pub enums: Vec<HirFlatEnum>,
    pub structs: Vec<HirFlatStruct>,

    // non-components
    pub traits: Vec<HirFlatTrait>,
    pub trait_impls: Vec<HirFlatTraitImpl>,
    pub type_alias: Vec<HirFlatTypeAlias>,
}
