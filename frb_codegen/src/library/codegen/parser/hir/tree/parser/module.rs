use crate::codegen::ir::hir::tree::module::{
    HirTreeModule, HirTreeModuleContent, HirTreeModuleMeta,
};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::namespace::Namespace;
use syn::ItemMod;

pub(super) fn parse_module(
    items: Vec<syn::Item>,
    meta: HirTreeModuleMeta,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirTreeModule> {
    let raw = (items.iter())
        .filter(|item| !matches!(item, syn::Item::Mod(_)))
        .map(|item| quote::quote!(#item).to_string())
        .collect();

    let mut output_items = vec![];
    let mut output_modules = vec![];

    for item in items.into_iter() {
        match item {
            syn::Item::Mod(item_mod) => output_modules.extend(parse_syn_item_mod()?),
            _ => output_items.push(item),
        }
    }

    Ok(HirTreeModule {
        meta,
        items: output_items,
        modules: output_modules,
        raw,
    })
}

fn parse_syn_item_mod(
    item_mod: &ItemMod,
    namespace: &Namespace,
    config: &ParserHirInternalConfig,
    parent_vis: &[HirVisibility],
) -> anyhow::Result<Option<HirTreeModule>> {
    Ok(if let Some((_, items)) = item_mod.content {
        let info = HirTreeModuleMeta {
            parent_vis: parent_vis.to_owned(),
            vis: (&item_mod.vis).into(),
            namespace: namespace.join(&item_mod.ident.to_string()),
        };
        Some(parse_module(items, info, config)?)
    } else {
        None
    })
}
