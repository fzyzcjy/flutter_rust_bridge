use crate::utils::namespace::NamespacedName;

#[derive(Clone, Derivative, Serialize, Debug)]
pub struct HirFlatTraitImpl {
    pub(crate) trait_name: NamespacedName,
    pub(crate) struct_or_enum_name: NamespacedName,
}
