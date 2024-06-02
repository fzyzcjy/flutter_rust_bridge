use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::raw::pack::HirRawPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) mod crates;
pub(crate) mod function;
pub(crate) mod item_type;
pub(crate) mod mirror_ident;
pub(crate) mod module;
pub(crate) mod pack;
pub(crate) mod struct_or_enum;
pub(crate) mod visibility;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_raw: &HirRawPack,
) -> anyhow::Result<HirPack> {
    pack::parse_pack(config, hir_raw)
}
