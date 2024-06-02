use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::misc::THIRD_PARTY_DIR_NAME;
use crate::utils::crate_name::CrateName;

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    if let Some(module_third_party_root) = remove_module_third_party_root(&mut pack) {
        for module_third_party_crate in module_third_party_root.content.modules {
            let crate_name = (module_third_party_crate.meta.namespace.path().last()).unwrap();
            if let Some(target_crate) = pack.crates.get_mut(crate_name) {
                transform_crate(target_crate, module_third_party_crate)
            } else {
                log::warn!(
                    "Skip `{}` since there is no corresponding scanned third party crate.",
                    module_third_party_crate.meta.namespace,
                );
            }
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

fn transform_crate(target: &mut HirCrate, src: HirModule) -> anyhow::Result<()> {
    todo!()
}
