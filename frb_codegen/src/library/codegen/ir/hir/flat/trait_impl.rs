use crate::utils::namespace::NamespacedName;

#[derive(Clone, serde::Serialize, Debug)]
pub struct HirFlatTraitImpl {
    pub(crate) trait_name: String,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) impl_ty: syn::Type,
}
