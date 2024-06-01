use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use crate::codegen::ir::hir::hierarchical::module::{HirModuleMeta, HirVisibility};
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::raw::HirRawPack;
use crate::codegen::ir::mir::namespace::Namespace;
use crate::codegen::parser::hir::hierarchical::crates::parse_crate;
use crate::codegen::parser::hir::hierarchical::module::parse_module;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) mod crates;
pub(crate) mod function;
pub(crate) mod item_type;
pub(crate) mod mirror_ident;
pub(crate) mod module;
pub(crate) mod struct_or_enum;
pub(crate) mod visibility;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_raw: &HirRawPack,
) -> anyhow::Result<HirPack> {
    let crates = hir_raw
        .crates
        .iter()
        .map(|(crate_name, syn_file)| Ok((crate_name.to_owned(), parse_crate(config, syn_file)?)))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .collect();
    Ok(HirPack { crates })
}
