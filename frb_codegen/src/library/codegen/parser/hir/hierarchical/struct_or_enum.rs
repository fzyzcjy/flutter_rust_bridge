use crate::codegen::ir::hir::hierarchical::struct_or_enum::{HirEnum, HirStruct, HirStructOrEnum};
use crate::utils::namespace::{Namespace, NamespacedName};
use crate::codegen::parser::hir::hierarchical::mirror_ident::{
    parse_mirror_ident, ParseMirrorIdentOutput,
};
use itertools::Itertools;
use log::debug;
use proc_macro2::Ident;
use syn::{Attribute, ItemEnum, ItemStruct};

pub(crate) fn parse_syn_item_struct(
    item: &ItemStruct,
    namespace: &Namespace,
) -> anyhow::Result<Vec<HirStruct>> {
    parse_syn_item_struct_or_enum(
        item,
        namespace,
        &item.ident,
        &item.attrs,
        &item.vis,
        HirStruct,
    )
}

pub(crate) fn parse_syn_item_enum(
    item: &ItemEnum,
    namespace: &Namespace,
) -> anyhow::Result<Vec<HirEnum>> {
    parse_syn_item_struct_or_enum(
        item,
        namespace,
        &item.ident,
        &item.attrs,
        &item.vis,
        HirEnum,
    )
}

fn parse_syn_item_struct_or_enum<I: Clone, F, T>(
    item: &I,
    namespace: &Namespace,
    item_ident: &Ident,
    item_attrs: &[Attribute],
    item_vis: &syn::Visibility,
    constructor: F,
) -> anyhow::Result<Vec<T>>
where
    F: Fn(HirStructOrEnum<I>) -> T,
{
    debug!("parse_syn_item_struct_or_enum item_ident={item_ident:?}");

    let ParseMirrorIdentOutput { idents, mirror } = parse_mirror_ident(item_ident, item_attrs)?;

    Ok(idents
        .into_iter()
        .map(|ident| {
            let ident_str = ident.to_string();
            constructor(HirStructOrEnum {
                ident,
                src: item.clone(),
                visibility: item_vis.into(),
                namespaced_name: NamespacedName::new(namespace.to_owned(), ident_str),
                mirror,
            })
        })
        .collect_vec())
}
