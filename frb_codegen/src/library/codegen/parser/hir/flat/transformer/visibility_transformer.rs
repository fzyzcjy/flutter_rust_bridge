use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::utils::crate_name::CrateName;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    TODO;
    Ok(pack)
}

fn is_interest_mod(meta: &HirTreeModuleMeta) -> bool {
    // If it is third party crate, then we only scan the `pub` mods,
    // since for non-pub modes, it is impossible to use them even if we scanned them.
    meta.namespace.path()[0] == CrateName::SELF_CRATE || meta.is_public()
}
