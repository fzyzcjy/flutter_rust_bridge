mod pub_use_transformer;
mod syn_item;

use crate::codegen::ir::hir::hierarchical::module::{HirModule, HirModuleContent, HirModuleMeta};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) fn parse_module(
    items: &[syn::Item],
    meta: HirModuleMeta,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirModule> {
    let module = parse_raw(items, meta, config)?;
    let module = pub_use_transformer::transform(module, items)?;
    Ok(module)
}

// moved
// fn parse_raw(
//     items: &[syn::Item],
//     meta: HirModuleMeta,
//     config: &ParserHirInternalConfig,
// ) -> anyhow::Result<HirModule> {
//     let mut scope = HirModuleContent::default();
//     for item in items.iter() {
//         syn_item::parse_syn_item(item, &mut scope, config, &meta.namespace, &meta.parent_vis)?;
//     }
//
//     Ok(HirModule {
//         meta,
//         content: scope,
//         raw: (items.iter())
//             .filter(|item| !matches!(item, syn::Item::Mod(_)))
//             .map(|item| quote::quote!(#item).to_string())
//             .collect(),
//     })
// }
