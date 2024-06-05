pub(crate) mod item_fn;
pub(crate) mod item_impl;
pub(crate) mod item_struct_or_enum;
pub(crate) mod item_trait;
pub(crate) mod item_type;

use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::codegen::parser::hir::flat::parser::syn_item::item_fn::parse_syn_item_fn;
use crate::codegen::parser::hir::flat::parser::syn_item::item_impl::parse_syn_item_impl;
use crate::codegen::parser::hir::flat::parser::syn_item::item_struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::parser::hir::flat::parser::syn_item::item_trait::parse_syn_item_trait;
use crate::codegen::parser::hir::flat::parser::syn_item::item_type::parse_syn_item_type;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::namespace::Namespace;

pub(crate) fn parse_syn_item(
    item: syn::Item,
    meta: &HirTreeModuleMeta,
    target: &mut HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<()> {
    match item {
        syn::Item::Struct(x) => (target.structs).extend(parse_syn_item_struct(x, meta)?),
        syn::Item::Enum(x) => (target.enums).extend(parse_syn_item_enum(x, meta)?),
        syn::Item::Type(x) => target.type_alias.extend(parse_syn_item_type(x)),
        syn::Item::Fn(x) => target.functions.push(parse_syn_item_fn(x, meta)),
        syn::Item::Impl(x) => parse_syn_item_impl(target, x),
        syn::Item::Trait(x) => parse_syn_item_trait(target, x),
        _ => {}
    }
    Ok(())
}