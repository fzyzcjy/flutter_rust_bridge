mod parser;
mod transformer;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::raw::pack::HirRawPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::ConfigDumpContent;
use ConfigDumpContent::Hir;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_raw: HirRawPack,
    dumper: &Dumper,
) -> anyhow::Result<HirTreePack> {
    let pack = parser::pack::parse_pack(config, hir_raw)?;
    dumper.dump(Hir, "hir_tree/1_parse_pack.json", &pack)?;

    let pack = transformer::pub_use_transformer::transform(pack)?;
    dumper.dump(Hir, "hir_tree/2_pub_use_transformer.json", &pack)?;

    Ok(pack)
}
