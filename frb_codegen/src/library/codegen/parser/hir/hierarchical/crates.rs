use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use crate::codegen::ir::hir::hierarchical::module::{HirModuleMeta, HirVisibility};
use crate::utils::namespace::Namespace;
use crate::codegen::parser::hir::hierarchical::module::parse_module;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) fn parse_crate(
    config: &ParserHirInternalConfig,
    file: &syn::File,
    crate_name: &str,
) -> anyhow::Result<HirCrate> {
    let info = HirModuleMeta {
        visibility: HirVisibility::Public,
        namespace: Namespace::new(vec![crate_name.to_owned()]),
    };
    let root_module = parse_module(&file.items, info, config)?;
    Ok(HirCrate { root_module })
}
