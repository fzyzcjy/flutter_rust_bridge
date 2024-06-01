use crate::codegen::hir::hierarchical::module::{HirModuleInfo, HirVisibility};
use crate::codegen::hir::hierarchical::struct_or_enum::{HirEnum, HirStruct, HirStructOrEnum};
use crate::codegen::hir_parser::hierarchical::mirror_ident::{
    parse_mirror_ident, ParseMirrorIdentOutput,
};
use log::debug;
use proc_macro2::Ident;
use syn::{Attribute, ItemEnum, ItemStruct};

pub(crate) fn parse_syn_item_struct(
    info: &HirModuleInfo,
    item: &ItemStruct,
) -> anyhow::Result<Vec<HirStruct>> {
    parse_syn_item_struct_or_enum(info, item, &item.ident, &item.attrs, &item.vis, HirStruct)
}

pub(crate) fn parse_syn_item_enum(
    info: &HirModuleInfo,
    item: &ItemEnum,
) -> anyhow::Result<Vec<HirEnum>> {
    parse_syn_item_struct_or_enum(info, item, &item.ident, &item.attrs, &item.vis, HirEnum)
}

fn parse_syn_item_struct_or_enum<I: Clone, F, T>(
    info: &HirModuleInfo,
    item: &I,
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
                visibility: HirVisibility::from_syn(item_vis),
                path: {
                    let mut path = info.module_path.clone();
                    path.push(ident_str);
                    path
                },
                mirror,
            })
        })
        .collect_vec())
}
