pub(crate) mod item_const;
pub(crate) mod item_fn;
pub(crate) mod item_impl;
pub(crate) mod item_struct_or_enum;
pub(crate) mod item_trait;
pub(crate) mod item_type;

use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use crate::codegen::parser::hir::flat::parser::syn_item::item_const::parse_syn_item_const;
use crate::codegen::parser::hir::flat::parser::syn_item::item_fn::parse_syn_item_fn;
use crate::codegen::parser::hir::flat::parser::syn_item::item_impl::parse_syn_item_impl;
use crate::codegen::parser::hir::flat::parser::syn_item::item_struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::parser::hir::flat::parser::syn_item::item_trait::parse_syn_item_trait;
use crate::codegen::parser::hir::flat::parser::syn_item::item_type::parse_syn_item_type;

pub(crate) fn parse_syn_item(
    item: syn::Item,
    meta: &HirNaiveFlatItemMeta,
    target: &mut HirFlatPack,
    parse_const: bool,
) -> anyhow::Result<()> {
    match item {
        syn::Item::Struct(x) => (target.structs).extend(parse_syn_item_struct(&x, meta)?),
        syn::Item::Enum(x) => (target.enums).extend(parse_syn_item_enum(&x, meta)?),
        syn::Item::Type(x) => target.types.extend(parse_syn_item_type(x)),
        syn::Item::Fn(x) => target.functions.push(parse_syn_item_fn(x, meta)),
        syn::Item::Const(x) if parse_const => target.constants.push(parse_syn_item_const(x, meta)),
        syn::Item::Impl(x) => parse_syn_item_impl(target, x, meta),
        syn::Item::Trait(x) => parse_syn_item_trait(target, x, meta),
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
    use crate::utils::namespace::Namespace;
    use syn::parse_quote;

    fn meta() -> HirNaiveFlatItemMeta {
        HirNaiveFlatItemMeta {
            namespace: Namespace::new_raw("crate::api".to_owned()),
            sources: vec![HirGenerationSource::Normal],
            is_module_public: true,
        }
    }

    /// Dispatches supported item kinds and gates constants by configuration.
    #[test]
    fn dispatches_items_and_honors_constant_flag() {
        let mut pack = HirFlatPack::default();
        parse_syn_item(
            parse_quote!(
                pub struct Widget;
            ),
            &meta(),
            &mut pack,
            false,
        )
        .unwrap();
        parse_syn_item(
            parse_quote!(
                pub fn run() {}
            ),
            &meta(),
            &mut pack,
            false,
        )
        .unwrap();
        parse_syn_item(
            parse_quote!(
                pub const ID: u8 = 1;
            ),
            &meta(),
            &mut pack,
            false,
        )
        .unwrap();
        parse_syn_item(
            parse_quote!(
                pub const ENABLED: u8 = 2;
            ),
            &meta(),
            &mut pack,
            true,
        )
        .unwrap();

        assert_eq!(pack.structs.len(), 1);
        assert_eq!(pack.functions.len(), 1);
        assert_eq!(pack.constants.len(), 1);
        assert_eq!(pack.constants[0].item_const.ident, "ENABLED");
    }
}
