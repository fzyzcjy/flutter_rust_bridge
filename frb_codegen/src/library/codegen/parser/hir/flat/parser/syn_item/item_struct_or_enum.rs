use crate::codegen::ir::hir::flat::struct_or_enum::{
    HirFlatEnum, HirFlatStruct, HirFlatStructOrEnum,
};
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::ir::hir::misc::visibility::is_visibility_accessible_from;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use crate::codegen::parser::hir::flat::parser::mirror_ident::{
    parse_mirror_ident, ParseMirrorIdentOutput,
};
use crate::utils::namespace::{Namespace, NamespacedName};
use itertools::Itertools;
use log::debug;
use proc_macro2::Ident;
use syn::{Attribute, ItemEnum, ItemStruct};

pub(crate) fn parse_syn_item_struct(
    item: &ItemStruct,
    meta: &HirNaiveFlatItemMeta,
    rust_output_path_namespace: &Namespace,
) -> anyhow::Result<Vec<HirFlatStruct>> {
    parse_syn_item_struct_or_enum(
        item,
        meta,
        &item.ident,
        &item.attrs,
        &item.vis,
        rust_output_path_namespace,
    )
}

pub(crate) fn parse_syn_item_enum(
    item: &ItemEnum,
    meta: &HirNaiveFlatItemMeta,
    rust_output_path_namespace: &Namespace,
) -> anyhow::Result<Vec<HirFlatEnum>> {
    parse_syn_item_struct_or_enum(
        item,
        meta,
        &item.ident,
        &item.attrs,
        &item.vis,
        rust_output_path_namespace,
    )
}

fn parse_syn_item_struct_or_enum<I: SynItemStructOrEnum>(
    item: &I,
    meta: &HirNaiveFlatItemMeta,
    item_ident: &Ident,
    item_attrs: &[Attribute],
    item_vis: &syn::Visibility,
    rust_output_path_namespace: &Namespace,
) -> anyhow::Result<Vec<HirFlatStructOrEnum<I>>> {
    debug!("parse_syn_item_struct_or_enum item_ident={item_ident:?}");

    let ParseMirrorIdentOutput {
        idents,
        mirror: mirror_by_ident,
    } = parse_mirror_ident(item_ident, item_attrs)?;

    Ok(idents
        .into_iter()
        .map(|ident| HirFlatStructOrEnum {
            name: NamespacedName::new(meta.namespace.to_owned(), ident.to_string()),
            visibility: item_vis.into(),
            is_accessible_from_rust_output: meta.is_module_accessible_from_rust_output
                && is_visibility_accessible_from(
                    item_vis,
                    &meta.namespace,
                    rust_output_path_namespace,
                ),
            imports: meta.imports.clone(),
            mirror: mirror_by_ident || !meta.namespace.crate_name().is_self_crate(),
            sources: meta.sources.clone(),
            src: item.to_owned(),
        })
        .collect_vec())
}
