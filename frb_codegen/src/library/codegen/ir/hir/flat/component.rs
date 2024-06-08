pub(crate) trait HirFlatComponent<SK: Ord> {
    fn sort_key(&self) -> SK;
}
