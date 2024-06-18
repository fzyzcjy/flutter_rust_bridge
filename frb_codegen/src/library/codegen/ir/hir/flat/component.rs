pub(crate) trait HirFlatComponentTrait<SK: Ord> {
    fn sort_key(&self) -> SK;
}
