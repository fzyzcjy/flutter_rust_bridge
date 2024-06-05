use crate::codegen::ir::hir::hierarchical::function::HirFlatFunction;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use std::collections::HashMap;
use syn::Type;

pub(crate) struct HirFlatCrate<'a> {
    pub functions: Vec<&'a HirFlatFunction>,
    pub structs: HashMap<String, &'a HirFlatStruct>,
    pub enums: HashMap<String, &'a HirFlatEnum>,
    pub types: HashMap<String, Type>,
    pub modules: Vec<&'a HirModule>,
}
