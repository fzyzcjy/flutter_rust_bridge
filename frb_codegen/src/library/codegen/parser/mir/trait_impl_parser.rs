use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::mir::func::MirFuncOwnerInfo;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::parser::mir::attribute_parser::FrbAttributes;
use crate::codegen::parser::mir::type_parser::{TypeParser, TypeParserParsingContext};
use crate::utils::syn_utils::ty_to_string;
use itertools::Itertools;

pub(super) fn parse(
    hir_trait_impls: &[HirFlatTraitImpl],
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirTraitImpl>> {
    let context = TypeParserParsingContext {
        initiated_namespace: TODO,
        func_attributes: FrbAttributes::parse(&[])?,
        default_stream_sink_codec: TODO,
        default_rust_opaque_codec: TODO,
        owner: TODO,
    };

    (hir_trait_impls.iter())
        .map(|x| {
            Ok(MirTraitImpl {
                trait_ty: type_parser.parse_type(&syn::parse_str(&x.trait_name)?, &context)?,
                impl_ty: type_parser.parse_type(&x.impl_ty, &context)?,
            })
        })
        .collect()
}
