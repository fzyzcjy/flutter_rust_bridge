use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::codegen::parser::hir::flat::parser::syn_item::parse_syn_item;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) fn parse_pack(
    config: &ParserHirInternalConfig,
    hir_tree: HirTreePack,
) -> anyhow::Result<HirFlatPack> {
    let items = super::flattener::flatten(hir_tree)?;
    let items = super::visibility_filter::filter_visible_modules(items);

    let mut pack = HirFlatPack {
        existing_handler: super::existing_handler::parse_existing_handler(&items),
        ..HirFlatPack::default()
    };

    for item in items {
        parse_syn_item(item.item, &item.meta, &mut pack, config)?;
    }

    Ok(pack)
}
