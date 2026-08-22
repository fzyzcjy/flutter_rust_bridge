use crate::codegen::ir::hir::flat::constant::HirFlatConstant;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use syn::ItemConst;

pub(crate) fn parse_syn_item_const(
    item_const: ItemConst,
    meta: &HirNaiveFlatItemMeta,
) -> HirFlatConstant {
    HirFlatConstant {
        namespace: meta.namespace.clone(),
        item_const,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
    use crate::utils::namespace::Namespace;
    use syn::parse_quote;

    /// Preserves the namespace and parsed constant.
    #[test]
    fn preserves_constant_metadata() {
        let meta = HirNaiveFlatItemMeta {
            namespace: Namespace::new_raw("crate::api".to_owned()),
            sources: vec![HirGenerationSource::Normal],
            is_module_public: true,
        };
        let parsed = parse_syn_item_const(
            parse_quote!(
                pub const ANSWER: u8 = 42;
            ),
            &meta,
        );

        assert_eq!(parsed.namespace, meta.namespace);
        assert_eq!(parsed.item_const.ident, "ANSWER");
    }
}
