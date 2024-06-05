#[derive(Debug, Clone, serde::Serialize)]
pub struct HirTreePack {
    pub(crate) crates: Vec<HirTreeCrate>,
}
