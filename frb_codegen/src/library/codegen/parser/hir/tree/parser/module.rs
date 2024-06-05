use crate::codegen::ir::hir::tree::module::{HirTreeModule, HirTreeModuleContent};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

fn parse_module(
    items: &[syn::Item],
    meta: HirModuleMeta,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirTreeModule> {
    let mut content = HirTreeModuleContent::default();
    for item in items.iter() {
        parse_syn_item(item, &mut content, config, &meta.namespace, &meta.parent_vis)?;
    }

    Ok(HirTreeModule {
        meta,
        content,
        raw: (items.iter())
            .filter(|item| !matches!(item, syn::Item::Mod(_)))
            .map(|item| quote::quote!(#item).to_string())
            .collect(),
    })
}
