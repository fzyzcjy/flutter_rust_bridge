use crate::codegen::hir::hierarchical::struct_or_enum::HirEnum;
use crate::codegen::hir::hierarchical::struct_or_enum::HirStruct;
use crate::codegen::hir::hierarchical::type_alias::HirTypeAlias;
use derivative::Derivative;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct HirModule {
    pub info: HirModuleInfo,
    pub scope: HirModuleScope,
}

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct HirModuleInfo {
    pub visibility: HirVisibility,
    pub file_path: PathBuf,
    pub module_path: Vec<String>,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub source: HirModuleSource,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct HirModuleScope {
    pub modules: Vec<HirModule>,
    pub enums: Vec<HirEnum>,
    pub structs: Vec<HirStruct>,
    // pub imports: Vec<Import>, // not implemented yet
    pub type_alias: Vec<HirTypeAlias>,
}

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone, Serialize)]
pub enum HirVisibility {
    Public,
    Restricted,
    // Not supported
    Inherited, // Usually means private
}

#[derive(Debug, Clone)]
pub enum HirModuleSource {
    File(syn::File),
    ModuleInFile(Vec<syn::Item>),
}

impl HirModuleSource {
    pub(crate) fn items(&self) -> &[syn::Item] {
        match self {
            HirModuleSource::File(file) => &file.items,
            HirModuleSource::ModuleInFile(items) => items,
        }
    }
}
