use crate::codegen::ir::hir::flat::struct_or_enum::{
    HirFlatEnum, HirFlatStruct, HirFlatStructOrEnum,
};
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use crate::codegen::parser::hir::flat::parser::mirror_ident::{
    parse_mirror_ident, ParseMirrorIdentOutput,
};
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;
use log::debug;
use proc_macro2::Ident;
use syn::{Attribute, ItemEnum, ItemStruct};

pub(crate) fn parse_syn_item_struct(
    item: &ItemStruct,
    meta: &HirNaiveFlatItemMeta,
) -> anyhow::Result<Vec<HirFlatStruct>> {
    parse_syn_item_struct_or_enum(item, meta, &item.ident, &item.attrs, &item.vis)
}

pub(crate) fn parse_syn_item_enum(
    item: &ItemEnum,
    meta: &HirNaiveFlatItemMeta,
) -> anyhow::Result<Vec<HirFlatEnum>> {
    parse_syn_item_struct_or_enum(item, meta, &item.ident, &item.attrs, &item.vis)
}

fn parse_syn_item_struct_or_enum<I: SynItemStructOrEnum>(
    item: &I,
    meta: &HirNaiveFlatItemMeta,
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
            name: NamespacedName::new(meta.namespace.to_owned(), ident.to_string()),
            visibility: item_vis.into(),
            mirror: mirror_by_ident || !meta.namespace.crate_name().is_self_crate(),
            sources: meta.sources.clone(),
            src: item.to_owned(),
        })
        .collect_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
    use crate::codegen::ir::hir::misc::visibility::HirVisibility;
    use crate::utils::namespace::Namespace;
    use syn::parse_quote;

    /// Preserves struct visibility and metadata in the flat item.
    #[test]
    fn parses_struct_with_namespace_and_sources() {
        let meta = HirNaiveFlatItemMeta {
            namespace: Namespace::new_raw("crate::api".to_owned()),
            sources: vec![HirGenerationSource::Normal],
            is_module_public: true,
        };
        let parsed = parse_syn_item_struct(
            &parse_quote!(
                pub struct Widget;
            ),
            &meta,
        )
        .unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name.rust_style(), "crate::api::Widget");
        assert_eq!(parsed[0].visibility, HirVisibility::Public);
        assert_eq!(parsed[0].sources, meta.sources);
        assert!(!parsed[0].mirror);
    }

    /// Mirrors items from a non-self crate namespace.
    #[test]
    fn mirrors_external_enum() {
        let meta = HirNaiveFlatItemMeta {
            namespace: Namespace::new_raw("dependency::api".to_owned()),
            sources: vec![],
            is_module_public: true,
        };
        let parsed = parse_syn_item_enum(
            &parse_quote!(
                pub enum Status {
                    Ready,
                }
            ),
            &meta,
        )
        .unwrap();

        assert!(parsed[0].mirror);
    }
}
