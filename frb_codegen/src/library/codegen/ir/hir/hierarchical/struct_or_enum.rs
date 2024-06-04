use crate::codegen::ir::hir::hierarchical::misc::HirCommon;
use crate::codegen::ir::hir::hierarchical::module::HirVisibility;
use crate::codegen::ir::hir::hierarchical::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::utils::namespace::{Namespace, NamespacedName};
use derivative::Derivative;
use proc_macro2::Ident;
use quote::ToTokens;
use serde::{Serialize, Serializer};
use syn::{ItemEnum, ItemStruct};

// This struct is surely used many times, but coverage tool thinks it is never used
// (possibly because of the macro?), so we manually exclude it from coverage report
// frb-coverage:ignore-start
#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct HirStructOrEnum<Item: SynItemStructOrEnum> {
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

pub type HirStruct = HirStructOrEnum<ItemStruct>;
pub type HirEnum = HirStructOrEnum<ItemEnum>;

impl<Item: SynItemStructOrEnum> HirCommon for HirStructOrEnum<Item> {
    fn with_namespace(&self, namespace: Namespace) -> Self {
        Self {
            namespaced_name: NamespacedName::new(namespace, self.namespaced_name.name.clone()),
            ..self.to_owned()
        }
    }

    fn ident(&self) -> String {
        self.ident.to_string()
    }
}

pub(super) fn serialize_syn<T: ToTokens, S: Serializer>(
    value: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    quote::quote!(#value).to_string().serialize(s)
}
