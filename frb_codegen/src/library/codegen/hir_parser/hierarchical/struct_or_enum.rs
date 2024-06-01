use crate::codegen::hir::hierarchical::module::{ModuleInfo, Visibility};
use crate::codegen::hir::hierarchical::struct_or_enum::{Enum, Struct, StructOrEnum};
use crate::codegen::hir_parser::hierarchical::mirror_ident::{
    parse_mirror_ident, ParseMirrorIdentOutput,
};
use log::debug;
use proc_macro2::Ident;
use syn::{Attribute, ItemEnum, ItemStruct};

pub(crate) fn parse_syn_item_struct(
    info: &ModuleInfo,
    item: &ItemStruct,
) -> anyhow::Result<Vec<Struct>> {
    parse_syn_item_struct_or_enum(info, item, &item.ident, &item.attrs, &item.vis, Struct)
}

pub(crate) fn parse_syn_item_enum(info: &ModuleInfo, item: &ItemEnum) -> anyhow::Result<Vec<Enum>> {
    parse_syn_item_struct_or_enum(info, item, &item.ident, &item.attrs, &item.vis, Enum)
}

fn parse_syn_item_struct_or_enum<I: Clone, F, T>(
    info: &ModuleInfo,
    item: &I,
    item_ident: &Ident,
    item_attrs: &[Attribute],
    item_vis: &syn::Visibility,
    constructor: F,
) -> anyhow::Result<Vec<T>>
where
    F: Fn(StructOrEnum<I>) -> T,
{
    debug!("parse_syn_item_struct_or_enum item_ident={item_ident:?}");

    let ParseMirrorIdentOutput { idents, mirror } = parse_mirror_ident(item_ident, item_attrs)?;

    Ok(idents
        .into_iter()
        .map(|ident| {
            let ident_str = ident.to_string();
            constructor(StructOrEnum {
                ident,
                src: item.clone(),
                visibility: Visibility::from_syn(item_vis),
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
