mod parser;
pub(crate) mod transformer;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::raw::pack::HirRawPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_raw: HirRawPack,
    dumper: &Dumper,
) -> anyhow::Result<HirTreePack> {
    let pack = parser::pack::parse_pack(config, hir_raw)?;
    dumper.dump("1_parse_pack.json", &pack)?;

    let pack = transformer::pub_use_transformer::transform(pack)?;
    dumper.dump("2_pub_use_transformer.json", &pack)?;

    Ok(pack)
}
