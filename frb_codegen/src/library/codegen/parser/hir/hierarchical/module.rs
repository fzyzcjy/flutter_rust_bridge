use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::hierarchical::module::{
    HirModule, HirModuleContent, HirModuleMeta, HirVisibility,
};
use crate::codegen::ir::mir::namespace::Namespace;
use crate::codegen::parser::hir::hierarchical::function::parse_generalized_functions;
use crate::codegen::parser::hir::hierarchical::item_type::parse_syn_item_type;
use crate::codegen::parser::hir::hierarchical::struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::parser::mir::internal_config::ParserInternalConfig;
use crate::codegen::parser::mir::reader::CachedRustReader;
use log::debug;
use syn::ItemMod;

pub(crate) fn parse_module(
    items: &[syn::Item],
    info: HirModuleMeta,
    config: &ParserInternalConfig,
) -> anyhow::Result<HirModule> {
    let mut scope = HirModuleContent::default();

    if (config.rust_input_namespace_pack).is_interest(&info.namespace) {
        scope.functions = parse_generalized_functions(items, &info.namespace)?;
    }

    for item in items.iter() {
        parse_syn_item(item, &mut scope, &info.namespace, config)?;
    }

    Ok(HirModule {
        meta: info,
        content: scope,
        raw: (items.iter())
            .map(|item| quote::quote!(#item).to_string())
            .collect(),
    })
}

fn parse_syn_item(
    item: &syn::Item,
    scope: &mut HirModuleContent,
    namespace: &Namespace,
    config: &ParserInternalConfig,
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
                .extend(parse_syn_item_mod(item_mod, namespace, config)?);
        }
        _ => {}
    }
    Ok(())
}

fn parse_syn_item_mod(
    item_mod: &ItemMod,
    namespace: &Namespace,
    config: &ParserInternalConfig,
) -> anyhow::Result<Option<HirModule>> {
    Ok(if let Some((_, items)) = &item_mod.content {
        let info = HirModuleMeta {
            visibility: (&item_mod.vis).into(),
            namespace: namespace.join(&item_mod.ident.to_string()),
        };
        Some(parse_module(items, info, config)?)
    } else {
        None
    })
}
