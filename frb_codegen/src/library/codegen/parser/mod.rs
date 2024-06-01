use crate::codegen::dumper::Dumper;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::parser;
use crate::codegen::parser::mir::internal_config::ParserInternalConfig;
use crate::codegen::parser::mir::reader::CachedRustReader;

pub(crate) mod hir;
pub(crate) mod mir;

pub(crate) fn parse(
    config: &ParserInternalConfig,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<MirPack> {
    let pb = progress_bar_pack.parse_read.start();
    let mut cached_rust_reader = CachedRustReader::default();
    let file = cached_rust_reader.read_rust_crate(&config.rust_crate_dir, dumper)?;
    drop(pb);

    let pb = progress_bar_pack.parse_hir.start();
    let hir_hierarchical = hir::hierarchical::parse(config, file)?;
    let hir_flat = hir::flat::parse(&hir_hierarchical.root_module)?;
    drop(pb);

    let pb = progress_bar_pack.parse_mir.start();
    let mir_pack = mir::parse(config, &hir_flat)?;
    drop(pb);

    Ok(mir_pack)
}
