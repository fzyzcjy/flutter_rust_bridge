use crate::codegen::ir::hir::tree::module::{
    HirTreeModule, HirTreeModuleContent, HirTreeModuleMeta,
};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(super) fn parse_module(
    items: &[syn::Item],
    meta: HirTreeModuleMeta,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirTreeModule> {
    let mut output_items = vec![];
    let mut output_modules = vec![];

    for item in items.iter() {
        match item {
            syn::Item::Mod(item_mod) => TODO,
            _ => items.push(item.to_owned()),
        }
    }

    Ok(HirTreeModule {
        meta,
        items: output_items,
        modules: output_modules,
        raw: (items.iter())
            .filter(|item| !matches!(item, syn::Item::Mod(_)))
            .map(|item| quote::quote!(#item).to_string())
            .collect(),
    })
}
