use crate::codegen::ir::hir::hierarchical::module::{HirModule, HirModuleContent, HirModuleMeta};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

fn parse_module(
    items: &[syn::Item],
    meta: HirModuleMeta,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirModule> {
    let mut scope = HirModuleContent::default();
    for item in items.iter() {
        parse_syn_item(item, &mut scope, config, &meta.namespace, &meta.parent_vis)?;
    }

    Ok(HirModule {
        meta,
        content: scope,
        raw: (items.iter())
            .filter(|item| !matches!(item, syn::Item::Mod(_)))
            .map(|item| quote::quote!(#item).to_string())
            .collect(),
    })
}
