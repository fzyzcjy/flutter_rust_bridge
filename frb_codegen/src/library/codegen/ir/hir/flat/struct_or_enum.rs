use crate::utils::namespace::{Namespace, NamespacedName};
use derivative::Derivative;
use proc_macro2::Ident;
use quote::ToTokens;
use serde::{Serialize, Serializer};
use syn::{ItemEnum, ItemStruct};

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct HirFlatStructOrEnum<Item: SynItemStructOrEnum> {
    pub(crate) name: NamespacedName,
    pub(crate) visibility: HirVisibility,
    pub(crate) mirror: bool,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub(crate) src: Item,
}

pub type HirFlatStruct = HirFlatStructOrEnum<ItemStruct>;
pub type HirFlatEnum = HirFlatStructOrEnum<ItemEnum>;
