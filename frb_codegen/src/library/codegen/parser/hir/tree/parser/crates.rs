use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::codegen::ir::hir::tree::crates::HirTreeCrate;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::hir::tree::parser::module::parse_module;
use crate::utils::crate_name::CrateName;

pub(crate) fn parse_crate(
    config: &ParserHirInternalConfig,
    file: syn::File,
    crate_name: &CrateName,
) -> anyhow::Result<HirTreeCrate> {
    let info = HirTreeModuleMeta {
        parent_vis: vec![],
        vis: HirVisibility::Public,
        namespace: crate_name.namespace(),
    };
    let root_module = parse_module(file.items, info, config)?;
    Ok(HirTreeCrate {
        name: crate_name.to_owned(),
        root_module,
    })
}
