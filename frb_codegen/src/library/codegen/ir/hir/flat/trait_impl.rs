use crate::utils::namespace::NamespacedName;

#[derive(Clone, Derivative, Serialize, Debug)]
pub struct HirFlatTraitImpl {
    pub(crate) def_name: NamespacedName,
    pub(crate) impl_name: NamespacedName,
}
