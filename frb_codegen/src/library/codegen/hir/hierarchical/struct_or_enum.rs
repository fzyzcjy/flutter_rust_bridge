use crate::codegen::ir::namespace::Namespace;
use crate::codegen::parser::source_graph::modules::Visibility;
use derivative::Derivative;
use proc_macro2::Ident;
use quote::ToTokens;
use serde::{Serialize, Serializer};
use syn::{Attribute, ItemEnum, ItemStruct};

// This struct is surely used many times, but coverage tool thinks it is never used
// (possibly because of the macro?), so we manually exclude it from coverage report
// frb-coverage:ignore-start
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
// frb-coverage:ignore-end

impl<Item> StructOrEnum<Item> {
    pub(crate) fn namespace(&self) -> Namespace {
        let mut p = self.path.clone();
        p.pop();
        Namespace::new(p)
    }
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
                &self.0.src.attrs
            }
        }
    };
}

struct_or_enum_wrapper!(Struct, ItemStruct);
struct_or_enum_wrapper!(Enum, ItemEnum);

pub(super) fn serialize_syn<T: ToTokens, S: Serializer>(
    value: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    quote::quote!(#value).to_string().serialize(s)
}
