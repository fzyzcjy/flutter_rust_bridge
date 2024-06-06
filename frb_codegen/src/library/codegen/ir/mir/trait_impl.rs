#[derive(Clone, serde::Serialize, Debug)]
pub struct MirTraitImpl {
    pub(crate) trait_name: String,
    pub(crate) impl_ty: String,
}
