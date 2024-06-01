use crate::codegen::dumper::Dumper;
use crate::codegen::hir::hierarchical::module::{
    HirModule, HirModuleInfo, HirModuleScope, HirVisibility,
};
use crate::codegen::hir_parser::hierarchical::function_extractor::extract_generalized_functions_from_syn_items;
use crate::codegen::hir_parser::hierarchical::item_type::parse_syn_item_type;
use crate::codegen::hir_parser::hierarchical::struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::parser::reader::CachedRustReader;
use log::debug;
use syn::ItemMod;

pub(crate) fn parse_module(items: &[syn::Item], info: HirModuleInfo) -> anyhow::Result<HirModule> {
    let mut scope = HirModuleScope::default();
    for item in items.iter() {
        parse_syn_item(item, &mut scope, &info.namespace)?;
    }

    extract_generalized_functions_from_syn_items(items)?;

    Ok(HirModule { info, scope })
}

fn parse_syn_item(
    item: &syn::Item,
    scope: &mut HirModuleScope,
    namespace: &Namespace,
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
                .extend(parse_syn_item_mod(item_mod, namespace)?);
        }
        _ => {}
    }
    Ok(())
}

fn parse_syn_item_mod(
    item_mod: &ItemMod,
    namespace: &Namespace,
) -> anyhow::Result<Option<HirModule>> {
    Ok(if let Some((_, items)) = &item_mod.content {
        let info = HirModuleInfo {
            visibility: (&item_mod.vis).into(),
            namespace: namespace.join(&item_mod.ident.to_string()),
        };
        Some(parse_module(items, info)?)
    } else {
        None
    })
}
