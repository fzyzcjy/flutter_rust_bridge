use std::hash::Hash;

pub(crate) trait HirFlatComponent<SK: Eq + Hash> {
    fn sort_key(&self) -> SK;
}
