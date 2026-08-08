use crate::codegen::ir::hir::misc::visibility::is_visibility_accessible_from;
use crate::codegen::ir::hir::tree::module::{HirTreeModule, HirTreeModuleMeta};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
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
            syn::Item::Mod(item_mod) => {
                output_modules.extend(parse_syn_item_mod(item_mod, config, &meta)?)
            }
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
    parent_meta: &HirTreeModuleMeta,
) -> anyhow::Result<Option<HirTreeModule>> {
    if let Some((_, items)) = item_mod.content {
        if !FrbAttributes::parse(&item_mod.attrs)?.ignore() {
            let info = HirTreeModuleMeta {
                parent_vis: parent_meta.parent_and_self_vis(),
                vis: (&item_mod.vis).into(),
                namespace: parent_meta.namespace.join(&item_mod.ident.to_string()),
                is_accessible_from_rust_output: parent_meta.is_accessible_from_rust_output
                    && is_visibility_accessible_from(
                        &item_mod.vis,
                        &parent_meta.namespace,
                        &config.rust_input_namespace_pack.rust_output_path_namespace,
                    ),
            };
            return Ok(Some(parse_module(items, info, config)?));
        }
    }
    Ok(None)
}
