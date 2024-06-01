use derivative::Derivative;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct Module {
    pub(super) info: ModuleInfo,
    pub(super) scope: ModuleScope,
}

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct ModuleInfo {
    pub(super) visibility: Visibility,
    pub(super) file_path: PathBuf,
    pub(super) module_path: Vec<String>,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub(super) source: ModuleSource,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModuleScope {
    pub(super) modules: Vec<Module>,
    pub(super) enums: Vec<Enum>,
    pub(super) structs: Vec<Struct>,
    // pub(super) imports: Vec<Import>, // not implemented yet
    pub(super) type_alias: Vec<TypeAlias>,
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
