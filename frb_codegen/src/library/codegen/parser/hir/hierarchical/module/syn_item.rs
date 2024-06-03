use crate::codegen::ir::hir::hierarchical::module::{
    HirModule, HirModuleContent, HirModuleMeta, HirVisibility,
};
use crate::codegen::parser::hir::hierarchical::item_type::parse_syn_item_type;
use crate::codegen::parser::hir::hierarchical::module::parse_module;
use crate::codegen::parser::hir::hierarchical::struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::namespace::Namespace;
use syn::ItemMod;

pub(super) fn parse_syn_item(
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
