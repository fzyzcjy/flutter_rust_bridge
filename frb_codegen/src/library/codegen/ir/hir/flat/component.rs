use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFlatComponentBase {
    // TODO
}

pub(crate) trait HirFlatComponentTrait<SK: Ord> {
    fn base_mut(&mut self) -> &mut HirFlatComponentBase;

    fn sort_key(&self) -> SK;
}
