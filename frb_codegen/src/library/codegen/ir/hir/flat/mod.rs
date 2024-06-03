use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::{HirEnum, HirStruct};
use std::collections::HashMap;
use syn::Type;

pub(crate) struct HirFlatCrate<'a> {
    pub functions: Vec<&'a HirFunction>,
    pub structs: HashMap<String, &'a HirStruct>,
    pub enums: HashMap<String, &'a HirEnum>,
    pub types: HashMap<String, Type>,
    pub modules: Vec<&'a HirModule>,
}
