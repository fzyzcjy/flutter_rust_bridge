use crate::codegen::dumper::Dumper;
use crate::codegen::hir::hierarchical::module::{Module, ModuleInfo, ModuleScope, ModuleSource};
use crate::codegen::parser::reader::CachedRustReader;
use log::debug;

/// Maps out modules, structs and enums within the scope of this module
//
// Things this doesn't currently support that it might need to later:
//
// - Import parsing is unfinished and so is currently disabled
// - When import parsing is enabled:
//     - Import renames (use a::b as c) - these are silently ignored
//     - Imports that start with two colons (use ::a::b) - these are also silently ignored

pub(crate) fn parse_module(
    info: ModuleInfo,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) -> anyhow::Result<Module> {
    debug!("parse_module START info={info:?}");

    let mut scope_modules = Vec::new();
    let mut scope_structs = Vec::new();
    let mut scope_enums = Vec::new();
    let mut scope_types = Vec::new();

    let items = match &info.source {
        ModuleSource::File(file) => &file.items,
        ModuleSource::ModuleInFile(items) => items,
    };

    for item in items.iter() {
        match item {
            syn::Item::Struct(item_struct) => {
                scope_structs.extend(parse_syn_item_struct(&info, item_struct)?);
            }
            syn::Item::Enum(item_enum) => {
                scope_enums.extend(parse_syn_item_enum(&info, item_enum)?);
            }
            syn::Item::Type(item_type) => {
                scope_types.extend(parse_syn_item_type(item_type));
            }
            syn::Item::Mod(item_mod) => {
                scope_modules.extend(parse_syn_item_mod(
                    &info,
                    item_mod,
                    cached_rust_reader,
                    dumper,
                )?);
            }
            _ => {}
        }
    }

    let ans = Module {
        info: info.clone(),
        scope: ModuleScope {
            modules: scope_modules,
            enums: scope_enums,
            structs: scope_structs,
            // imports: vec![], // Will be filled in by resolve_imports()
            type_alias: scope_types,
        },
    };

    debug!("parse_module END info={info:?}");
    Ok(ans)
}
