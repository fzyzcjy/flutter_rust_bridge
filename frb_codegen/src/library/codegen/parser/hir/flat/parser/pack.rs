use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::parser::hir::flat::parser::syn_item::parse_syn_item;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) fn parse_pack(
    config: &ParserHirInternalConfig,
    hir_naive_flat: HirNaiveFlatPack,
) -> anyhow::Result<HirFlatPack> {
    let mut pack = HirFlatPack {
        existing_handler: super::existing_handler::parse_existing_handler(
            &hir_naive_flat.items,
            config,
        )?,
        ..HirFlatPack::default()
    };

    for item in hir_naive_flat.items {
        parse_syn_item(item.item, &item.meta, &mut pack)?;
    }

    Ok(pack)
}
