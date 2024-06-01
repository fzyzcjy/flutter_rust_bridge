use crate::codegen::ir::hir::hierarchical::module::HirVisibility;
use crate::codegen::ir::mir::namespace::NamespacedName;
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
pub struct HirStructOrEnum<Item> {
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) ident: Ident,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub(crate) src: Item,
    pub(crate) visibility: HirVisibility,
    pub(crate) namespaced_name: NamespacedName,
    pub(crate) mirror: bool,
}
// frb-coverage:ignore-end

#[derive(Clone, Debug, Serialize)]
pub struct HirStruct(pub HirStructOrEnum<ItemStruct>);

#[derive(Clone, Debug, Serialize)]
pub struct HirEnum(pub HirStructOrEnum<ItemEnum>);

pub trait HirStructOrEnumWrapper<Item> {
    fn inner(&self) -> &HirStructOrEnum<Item>;

    fn attrs(&self) -> &[Attribute];
}

macro_rules! struct_or_enum_wrapper {
    ($name:ident, $item:ident) => {
        impl HirStructOrEnumWrapper<$item> for $name {
            fn inner(&self) -> &HirStructOrEnum<$item> {
                &self.0
            }

            fn attrs(&self) -> &[Attribute] {
                &self.0.src.attrs
            }
        }
    };
}

struct_or_enum_wrapper!(HirStruct, ItemStruct);
struct_or_enum_wrapper!(HirEnum, ItemEnum);

pub(super) fn serialize_syn<T: ToTokens, S: Serializer>(
    value: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    quote::quote!(#value).to_string().serialize(s)
}
