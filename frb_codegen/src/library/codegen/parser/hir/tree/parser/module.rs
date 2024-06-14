use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::codegen::ir::hir::tree::module::{HirTreeModule, HirTreeModuleMeta};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::utils::namespace::Namespace;
use syn::ItemMod;

pub(super) fn parse_module(
    items: Vec<syn::Item>,
    meta: HirTreeModuleMeta,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirTreeModule> {
    let mut output_items = vec![];
    let mut output_modules = vec![];

    for item in items.into_iter() {
        match item {
            syn::Item::Mod(item_mod) => output_modules.extend(parse_syn_item_mod(
                item_mod,
                config,
                &meta.namespace,
                &meta.parent_vis,
            )?),
            _ => output_items.push(item),
        }
    }

    Ok(HirTreeModule {
        meta,
        items: output_items,
        modules: output_modules,
    })
}

fn parse_syn_item_mod(
    item_mod: ItemMod,
    config: &ParserHirInternalConfig,
    namespace: &Namespace,
    parent_vis: &[HirVisibility],
) -> anyhow::Result<Option<HirTreeModule>> {
    if let Some((_, items)) = item_mod.content {
        if !FrbAttributes::parse(&item_mod.attrs)?.ignore() {
            let info = HirTreeModuleMeta {
                parent_vis: parent_vis.to_owned(),
                vis: (&item_mod.vis).into(),
                namespace: namespace.join(&item_mod.ident.to_string()),
            };
            return Ok(Some(parse_module(items, info, config)?));
        }
    }
    Ok(None)
}
