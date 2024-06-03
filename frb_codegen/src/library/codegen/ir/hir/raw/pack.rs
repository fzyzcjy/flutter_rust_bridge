use crate::codegen::ir::hir::raw::crates::HirRawCrate;

pub(crate) struct HirRawPack {
    pub crates: Vec<HirRawCrate>,
}
