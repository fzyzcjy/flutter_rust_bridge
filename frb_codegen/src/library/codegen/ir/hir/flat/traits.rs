use crate::utils::namespace::NamespacedName;

#[derive(Clone, serde::Serialize, Debug)]
pub struct HirFlatTrait {
    pub(crate) name: NamespacedName,
}
