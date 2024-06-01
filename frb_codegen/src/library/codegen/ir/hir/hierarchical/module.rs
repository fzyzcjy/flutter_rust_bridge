use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirEnum;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::ir::hir::hierarchical::type_alias::HirTypeAlias;
use crate::codegen::ir::mir::namespace::Namespace;
use derivative::Derivative;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct HirModule {
    pub meta: HirModuleMeta,
    pub content: HirModuleContent,
    pub raw: Vec<String>,
}

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct HirModuleMeta {
    pub visibility: HirVisibility,
    pub namespace: Namespace,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct HirModuleContent {
    pub modules: Vec<HirModule>,
    pub enums: Vec<HirEnum>,
    pub structs: Vec<HirStruct>,
    // pub imports: Vec<Import>, // not implemented yet
    pub type_alias: Vec<HirTypeAlias>,
    pub functions: Vec<HirFunction>,
}

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone, Serialize)]
pub enum HirVisibility {
    Public,
    Restricted,
    // Not supported
    Inherited, // Usually means private
}
