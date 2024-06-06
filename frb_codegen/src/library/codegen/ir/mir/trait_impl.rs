use crate::codegen::ir::mir::ty::trait_def::MirTypeTraitDef;
use crate::codegen::ir::mir::ty::MirType;

#[derive(Clone, serde::Serialize, Debug)]
pub struct MirTraitImpl {
    pub(crate) trait_ty: MirTypeTraitDef,
    pub(crate) impl_ty: MirType,
}
