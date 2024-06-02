use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::misc::THIRD_PARTY_DIR_NAME;
use crate::utils::crate_name::CrateName;

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    let module_third_party_root = remove_module_third_party_root(&mut pack);
    TODO;
    Ok(pack)
}

fn remove_module_third_party_root(pack: &mut HirPack) -> HirModule {
    let hir_crate = pack.crates.get_mut(&CrateName::self_crate()).unwrap();
    hir_crate.root_module.content.get_module_nested(&[THIRD_PARTY_DIR_NAME])
}
