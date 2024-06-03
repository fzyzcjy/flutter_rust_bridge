use crate::codegen::ir::hir::hierarchical::module::{
    HirModule, HirModuleContent, HirModuleMeta, HirVisibility,
};
use crate::codegen::parser::hir::hierarchical::function::{parse_syn_item_fn, parse_syn_item_impl};
use crate::codegen::parser::hir::hierarchical::item_type::parse_syn_item_type;
use crate::codegen::parser::hir::hierarchical::module::parse_module;
use crate::codegen::parser::hir::hierarchical::struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::parser::hir::hierarchical::traits::{parse_syn_item_trait, parse_trait_impl};
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
            (scope.enums).extend(parse_syn_item_enum(item_enum, namespace)?);
        }
        syn::Item::Type(item_type) => {
            scope.type_alias.extend(parse_syn_item_type(item_type));
        }
        syn::Item::Fn(item_fn) => {
            scope.functions.push(parse_syn_item_fn(item_fn, namespace));
        }
        syn::Item::Impl(item_impl) => {
            if item_impl.trait_.is_some() {
                (scope.trait_impls).push(parse_trait_impl(item_impl, namespace));
            } else {
                (scope.functions).extend(parse_syn_item_impl(item_impl, namespace));
            }
        }
        syn::Item::Trait(item_trait) => {
            (scope.traits).push(parse_syn_item_trait(item_trait, namespace));
        }
        syn::Item::Mod(item_mod) => {
            (scope.modules).extend(parse_syn_item_mod(item_mod, namespace, config, parent_vis)?);
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
