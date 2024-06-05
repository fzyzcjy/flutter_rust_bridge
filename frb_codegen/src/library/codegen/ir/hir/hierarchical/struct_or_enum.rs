use crate::codegen::ir::hir::hierarchical::misc::HirCommon;
use crate::codegen::ir::hir::hierarchical::module::HirVisibility;
use crate::codegen::ir::hir::hierarchical::syn_item_struct_or_enum::SynItemStructOrEnum;
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

impl<Item: SynItemStructOrEnum> HirCommon for HirFlatStructOrEnum<Item> {
    fn with_namespace(&self, namespace: Namespace) -> Self {
        Self {
            name: NamespacedName::new(namespace, self.name.name.clone()),
            ..self.to_owned()
        }
    }

    fn name_for_use_stmt(&self) -> String {
        self.ident.to_string()
    }
}

pub(super) fn serialize_syn<T: ToTokens, S: Serializer>(
    value: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    quote::quote!(#value).to_string().serialize(s)
}
