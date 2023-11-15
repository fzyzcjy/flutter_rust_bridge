use derivative::Derivative;
use std::path::PathBuf;
use syn::{Ident, ItemEnum, ItemStruct, Type};

#[derive(Clone, Debug)]
pub struct Module {
    pub(super) info: ModuleInfo,
    pub(super) scope: ModuleScope,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct ModuleInfo {
    pub(super) visibility: Visibility,
    pub(super) file_path: PathBuf,
    pub(super) module_path: Vec<String>,
    #[derivative(Debug = "ignore")]
    pub(super) source: ModuleSource,
}

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Restricted,
    // Not supported
    Inherited, // Usually means private
}

#[derive(Debug, Clone)]
pub struct Import {
    path: Vec<String>,
    visibility: Visibility,
}

#[derive(Debug, Clone)]
pub enum ModuleSource {
    File(syn::File),
    ModuleInFile(Vec<syn::Item>),
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Struct {
    pub(super) ident: Ident,
    #[derivative(Debug = "ignore")]
    pub(super) src: ItemStruct,
    pub(super) visibility: Visibility,
    pub(super) path: Vec<String>,
    pub(super) mirror: bool,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Enum {
    pub(super) ident: Ident,
    #[derivative(Debug = "ignore")]
    pub(super) src: ItemEnum,
    pub(super) visibility: Visibility,
    pub(super) path: Vec<String>,
    pub(super) mirror: bool,
}

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub(super) ident: String,
    pub(super) target: Type,
}

#[derive(Debug, Clone)]
pub struct ModuleScope {
    pub(super) modules: Vec<Module>,
    pub(super) enums: Vec<Enum>,
    pub(super) structs: Vec<Struct>,
    pub(super) imports: Vec<Import>,
    pub(super) type_alias: Vec<TypeAlias>,
}
