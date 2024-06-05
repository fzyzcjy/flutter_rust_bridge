use crate::utils::namespace::NamespacedName;

#[derive(Clone, Derivative, Serialize, Debug)]
pub struct HirFlatTraitImpl {
    pub(crate) trait_name: String,
    pub(crate) impl_ty: syn::Type,
}
