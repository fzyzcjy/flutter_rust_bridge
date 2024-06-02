mod pub_use_transformer;

use crate::codegen::ir::hir::hierarchical::module::{
    HirModule, HirModuleContent, HirModuleMeta, HirVisibility,
};
use crate::codegen::parser::hir::hierarchical::function::parse_generalized_functions;
use crate::codegen::parser::hir::hierarchical::item_type::parse_syn_item_type;
use crate::codegen::parser::hir::hierarchical::pub_use::transform_module_by_pub_use;
use crate::codegen::parser::hir::hierarchical::struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::namespace::Namespace;
use syn::ItemMod;

pub(crate) fn parse_module(
    items: &[syn::Item],
    meta: HirModuleMeta,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirModule> {
    let module = parse_module_raw(items, meta, config)?;
    let module = transform_module_by_pub_use(module, items)?;
    Ok(module)
}

fn parse_module_raw(
    items: &[syn::Item],
    meta: HirModuleMeta,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirModule> {
    let mut scope = HirModuleContent {
        functions: parse_generalized_functions(items, &meta.namespace)?,
        ..HirModuleContent::default()
    };

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

fn parse_syn_item(
    item: &syn::Item,
    scope: &mut HirModuleContent,
    config: &ParserHirInternalConfig,
    namespace: &Namespace,
    parent_vis: &[HirVisibility],
) -> anyhow::Result<()> {
    match item {
        syn::Item::Struct(item_struct) => {
            (scope.structs).extend(parse_syn_item_struct(item_struct, namespace)?);
        }
        syn::Item::Enum(item_enum) => {
            scope
                .enums
                .extend(parse_syn_item_enum(item_enum, namespace)?);
        }
        syn::Item::Type(item_type) => {
            scope.type_alias.extend(parse_syn_item_type(item_type));
        }
        syn::Item::Mod(item_mod) => {
            scope
                .modules
                .extend(parse_syn_item_mod(item_mod, namespace, config, parent_vis)?);
        }
        _ => {}
    }
    Ok(())
}

fn parse_syn_item_mod(
    item_mod: &ItemMod,
    namespace: &Namespace,
    config: &ParserHirInternalConfig,
    parent_vis: &[HirVisibility],
) -> anyhow::Result<Option<HirModule>> {
    Ok(if let Some((_, items)) = &item_mod.content {
        let info = HirModuleMeta {
            parent_vis: parent_vis.to_owned(),
            vis: (&item_mod.vis).into(),
            namespace: namespace.join(&item_mod.ident.to_string()),
        };
        Some(parse_module(
            items, info,
            config,
            // cumulated_visibility_pub && matches!(item_mod.vis, syn::Visibility::Public(_)),
        )?)
    } else {
        None
    })
}
