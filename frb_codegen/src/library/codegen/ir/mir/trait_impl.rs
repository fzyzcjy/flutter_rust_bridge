use crate::codegen::ir::mir::ty::MirType;

#[derive(Clone, serde::Serialize, Debug)]
pub struct MirTraitImpl {
    pub(crate) trait_ty: MirType,
    pub(crate) impl_ty: MirType,
}
