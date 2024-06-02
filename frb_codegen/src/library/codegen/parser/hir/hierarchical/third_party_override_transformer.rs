use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::misc::THIRD_PARTY_DIR_NAME;
use crate::utils::crate_name::CrateName;

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    if let Some(module_third_party_root) = remove_module_third_party_root(&mut pack) {
        for module_third_party_crate in module_third_party_root.content.modules {
            transform_crate(&mut pack, module_third_party_crate)?;
        }
    }
    Ok(pack)
}

fn remove_module_third_party_root(pack: &mut HirPack) -> Option<HirModule> {
    let hir_crate = pack.crates.get_mut(&CrateName::self_crate()).unwrap();
    hir_crate
        .root_module
        .content
        .remove_module_by_name(THIRD_PARTY_DIR_NAME)
}

fn transform_crate(pack: &mut HirPack, src: HirModule) -> anyhow::Result<()> {
    if let Some(target_crate) = pack.crates.get_mut() {
        TODO;
    } else {
        log::warn!(
            "Skip `{}` since there is no corresponding scanned third party crate.",
            src.meta.namespace,
        );
    }
}
