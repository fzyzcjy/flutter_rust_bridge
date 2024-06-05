use crate::codegen::ir::hir::hierarchical::struct_or_enum::{HirFlatEnum, HirFlatStruct, HirFlatStructOrEnum};
use crate::codegen::ir::hir::hierarchical::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::parser::hir::hierarchical::mirror_ident::{
    parse_mirror_ident, ParseMirrorIdentOutput,
};
use crate::utils::namespace::{Namespace, NamespacedName};
use itertools::Itertools;
use log::debug;
use proc_macro2::Ident;
use syn::{Attribute, ItemEnum, ItemStruct};

pub(crate) fn parse_syn_item_struct(
    item: &ItemStruct,
    namespace: &Namespace,
) -> anyhow::Result<Vec<HirFlatStruct>> {
    parse_syn_item_struct_or_enum(item, namespace, &item.ident, &item.attrs, &item.vis)
}

pub(crate) fn parse_syn_item_enum(
    item: &ItemEnum,
    namespace: &Namespace,
) -> anyhow::Result<Vec<HirFlatEnum>> {
    parse_syn_item_struct_or_enum(item, namespace, &item.ident, &item.attrs, &item.vis)
}

fn parse_syn_item_struct_or_enum<I: SynItemStructOrEnum>(
    item: &I,
    namespace: &Namespace,
    item_ident: &Ident,
    item_attrs: &[Attribute],
    item_vis: &syn::Visibility,
) -> anyhow::Result<Vec<HirFlatStructOrEnum<I>>> {
    debug!("parse_syn_item_struct_or_enum item_ident={item_ident:?}");

    let ParseMirrorIdentOutput {
        idents,
        mirror: mirror_by_ident,
    } = parse_mirror_ident(item_ident, item_attrs)?;

    Ok(idents
        .into_iter()
        .map(|ident| HirFlatStructOrEnum {
            src: item.clone(),
            visibility: item_vis.into(),
            name: NamespacedName::new(namespace.to_owned(), ident.to_string()),
            mirror: mirror_by_ident || !namespace.crate_name().is_self_crate(),
        })
        .collect_vec())
}
