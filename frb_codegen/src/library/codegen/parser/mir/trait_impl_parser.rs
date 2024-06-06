use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::mir::func::MirFuncOwnerInfo;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::parser::mir::attribute_parser::FrbAttributes;
use crate::codegen::parser::mir::type_parser::{TypeParser, TypeParserParsingContext};
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::Namespace;
use crate::utils::syn_utils::ty_to_string;
use itertools::Itertools;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;

pub(super) fn parse(
    hir_trait_impls: &[HirFlatTraitImpl],
    type_parser: &mut TypeParser,
    default_stream_sink_codec: CodecMode,
    default_rust_opaque_codec: RustOpaqueCodecMode,
) -> anyhow::Result<Vec<MirTraitImpl>> {
    let context = TypeParserParsingContext {
        initiated_namespace: CrateName::self_crate().namespace(), // just a dummy value
        func_attributes: FrbAttributes::parse(&[])?,
        default_stream_sink_codec,
        default_rust_opaque_codec,
        owner: None,
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
