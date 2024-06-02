use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use crate::codegen::ir::hir::hierarchical::module::HirModuleMeta;
use crate::codegen::ir::hir::hierarchical::module::HirVisibility;
use crate::codegen::parser::hir::hierarchical::module::parse_module;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::crate_name::CrateName;

pub(crate) fn parse_crate(
    config: &ParserHirInternalConfig,
    file: &syn::File,
    crate_name: &CrateName,
) -> anyhow::Result<HirCrate> {
    let info = HirModuleMeta {
        parent_vis: vec![],
        vis: HirVisibility::Public,
        namespace: crate_name.namespace(),
    };
    let root_module = parse_module(&file.items, info, config)?;
    Ok(HirCrate {
        name: crate_name.to_owned(),
        root_module,
    })
}
