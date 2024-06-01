use crate::codegen::hir::hierarchical::struct_or_enum::Enum;
use crate::codegen::hir::hierarchical::struct_or_enum::Struct;
use crate::codegen::hir::hierarchical::type_alias::TypeAlias;
use derivative::Derivative;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct Module {
    pub info: ModuleInfo,
    pub scope: ModuleScope,
}

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct ModuleInfo {
    pub visibility: Visibility,
    pub file_path: PathBuf,
    pub module_path: Vec<String>,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub source: ModuleSource,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModuleScope {
    pub modules: Vec<Module>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    // pub imports: Vec<Import>, // not implemented yet
    pub type_alias: Vec<TypeAlias>,
}

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone, Serialize)]
pub enum Visibility {
    Public,
    Restricted,
    // Not supported
    Inherited, // Usually means private
}

#[derive(Debug, Clone)]
pub enum ModuleSource {
    File(syn::File),
    ModuleInFile(Vec<syn::Item>),
}
