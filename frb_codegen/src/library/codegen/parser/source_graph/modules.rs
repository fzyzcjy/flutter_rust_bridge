use crate::codegen::ir::namespace::Namespace;
use derivative::Derivative;
use quote::ToTokens;
use serde::{Serialize, Serializer};
use std::path::PathBuf;
use syn::{Attribute, Ident, ItemEnum, ItemStruct, Type};

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone, Serialize)]
pub enum Visibility {
    Public,
    Restricted,
    // Not supported
    Inherited, // Usually means private
}

#[derive(Debug, Clone, Serialize)]
pub struct Import {
    path: Vec<String>,
    visibility: Visibility,
}

#[derive(Debug, Clone)]
pub enum ModuleSource {
    File(syn::File),
    ModuleInFile(Vec<syn::Item>),
}

#[derive(Clone, Debug, Serialize)]
pub struct TypeAlias {
    pub(super) ident: String,
    #[serde(serialize_with = "serialize_syn")]
    pub(super) target: Type,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModuleScope {
    pub(super) modules: Vec<Module>,
    pub(super) enums: Vec<Enum>,
    pub(super) structs: Vec<Struct>,
    // pub(super) imports: Vec<Import>, // not implemented yet
    pub(super) type_alias: Vec<TypeAlias>,
}

fn serialize_syn<T: ToTokens, S: Serializer>(value: &T, s: S) -> Result<S::Ok, S::Error> {
    quote::quote!(#value).to_string().serialize(s)
}
