pub(crate) mod item_fn;
pub(crate) mod item_struct_or_enum;
pub(crate) mod item_type;

use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::codegen::parser::hir::flat::parser::syn_item::item_fn::parse_syn_item_fn;
use crate::codegen::parser::hir::flat::parser::syn_item::item_struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::parser::hir::flat::parser::syn_item::item_type::parse_syn_item_type;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::namespace::Namespace;

pub(crate) fn parse_syn_item(
    item: syn::Item,
    meta: HirTreeModuleMeta,
    pack: &mut HirModuleContent,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<()> {
    match item {
        syn::Item::Struct(item_struct) => {
            (pack.structs).extend(parse_syn_item_struct(item_struct, namespace)?);
        }
        syn::Item::Enum(item_enum) => {
            (pack.enums).extend(parse_syn_item_enum(item_enum, namespace)?);
        }
        syn::Item::Type(item_type) => {
            pack.type_alias.extend(parse_syn_item_type(item_type));
        }
        syn::Item::Fn(item_fn) => {
            pack.functions.push(parse_syn_item_fn(item_fn, namespace));
        }
        syn::Item::Impl(item_impl) => {
            if item_impl.trait_.is_some() {
                (pack.trait_impls).push(parse_trait_impl(item_impl, namespace));
            } else {
                (pack.functions).extend(parse_syn_item_impl(item_impl, namespace, None));
            }
        }
        syn::Item::Trait(item_trait) => {
            (pack.traits).push(parse_syn_item_trait(item_trait, namespace));
        }
        _ => {}
    }
    Ok(())
}
