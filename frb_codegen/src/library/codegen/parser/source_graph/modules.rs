use derivative::Derivative;
use quote::ToTokens;
use serde::{Serialize, Serializer};
use std::path::PathBuf;
use syn::{Attribute, Ident, ItemEnum, ItemStruct, Type};

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

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct StructOrEnum<Item> {
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) ident: Ident,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub(crate) src: Item,
    pub(crate) visibility: Visibility,
    pub(crate) path: Vec<String>,
    pub(crate) mirror: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Struct(pub StructOrEnum<ItemStruct>);

#[derive(Clone, Debug, Serialize)]
pub struct Enum(pub StructOrEnum<ItemEnum>);

pub trait StructOrEnumWrapper<Item> {
    fn inner(&self) -> &StructOrEnum<Item>;

    fn attrs(&self) -> &[Attribute];
}

macro_rules! struct_or_enum_wrapper {
    ($name:ident, $item:ident) => {
        impl StructOrEnumWrapper<$item> for $name {
            fn inner(&self) -> &StructOrEnum<$item> {
                &self.0
            }

            fn attrs(&self) -> &[Attribute] {
                self.0.attrs
            }
        }
    };
}

struct_or_enum_wrapper!(Struct, ItemStruct);
struct_or_enum_wrapper!(Enum, ItemEnum);

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
