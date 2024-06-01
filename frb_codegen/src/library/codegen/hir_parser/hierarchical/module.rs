use crate::codegen::dumper::Dumper;
use crate::codegen::hir::hierarchical::module::{
    HirModule, HirModuleInfo, HirModuleScope, HirModuleSource, HirVisibility,
};
use crate::codegen::hir_parser::hierarchical::item_type::parse_syn_item_type;
use crate::codegen::hir_parser::hierarchical::struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::parser::reader::CachedRustReader;
use log::debug;
use syn::ItemMod;

pub(crate) fn parse_module(
    items: &[syn::Item],
    visibility: HirVisibility,
) -> anyhow::Result<HirModule> {
    let mut scope = HirModuleScope::default();
    for item in items.iter() {
        parse_syn_item(item, &mut scope)?;
    }

    Ok(HirModule {
        info: HirModuleInfo { visibility },
        scope,
    })
}

fn parse_syn_item(item: &syn::Item, scope: &mut HirModuleScope) -> anyhow::Result<()> {
    match item {
        syn::Item::Struct(item_struct) => {
            (scope.structs).extend(parse_syn_item_struct(&info, item_struct)?);
        }
        syn::Item::Enum(item_enum) => {
            scope.enums.extend(parse_syn_item_enum(&info, item_enum)?);
        }
        syn::Item::Type(item_type) => {
            scope.type_alias.extend(parse_syn_item_type(item_type));
        }
        syn::Item::Mod(item_mod) => {
            scope.modules.extend(parse_syn_item_mod(&info, item_mod)?);
        }
        _ => {}
    }
    Ok(())
}

fn parse_syn_item_mod(item_mod: &ItemMod) -> anyhow::Result<Option<HirModule>> {
    // TODO module_path := module_path + ident
    // let ident = item_mod.ident.clone();

    if let Some((_, items)) = &item_mod.content {
        parse_module(items, item_mod.vis.into())
    } else {
        Ok(None)
    }
}
