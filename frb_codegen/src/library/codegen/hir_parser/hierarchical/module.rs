use crate::codegen::dumper::Dumper;
use crate::codegen::hir::hierarchical::module::{Module, ModuleInfo, ModuleScope, ModuleSource};
use crate::codegen::parser::reader::CachedRustReader;
use log::debug;

/// Maps out modules, structs and enums within the scope of this module
pub(crate) fn parse_module(
    info: ModuleInfo,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) -> anyhow::Result<Module> {
    debug!("parse_module START info={info:?}");

    let mut scope = ModuleScope::default();

    for item in info.source.items().iter() {
        parse_syn_item(item, &mut scope)?;
    }

    let ans = Module {
        info: info.clone(),
        scope,
    };

    debug!("parse_module END info={info:?}");
    Ok(ans)
}

fn parse_syn_item(item: &syn::Item, scope: &mut ModuleScope) -> anyhow::Result<()> {
    match item {
        syn::Item::Struct(item_struct) => {
            (scope.structs).extend(parse_syn_item_struct(&info, item_struct)?);
        }
        syn::Item::Enum(item_enum) => {
            scope.enums.extend(parse_syn_item_enum(&info, item_enum)?);
        }
        syn::Item::Type(item_type) => {
            scope.types.extend(parse_syn_item_type(item_type));
        }
        syn::Item::Mod(item_mod) => {
            scope.modules.extend(parse_syn_item_mod(
                &info,
                item_mod,
                cached_rust_reader,
                dumper,
            )?);
        }
        _ => {}
    }
    Ok(())
}
