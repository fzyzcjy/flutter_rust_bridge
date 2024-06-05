use crate::utils::namespace::NamespacedName;

#[derive(Clone, Derivative, Serialize, Debug)]
pub struct HirFlatTrait {
    pub(crate) name: NamespacedName,
}
