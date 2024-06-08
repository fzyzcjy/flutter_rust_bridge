use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::ty::trait_def::parse_type_trait;
use crate::codegen::parser::mir::parser::ty::{TypeParser, TypeParserParsingContext};
use crate::utils::crate_name::CrateName;
use itertools::Itertools;

pub(crate) fn parse(
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

    Ok((hir_trait_impls.iter())
        .map(|x| {
            let trait_ty = parse_type_trait(&x.trait_name, type_parser);
            let impl_ty = type_parser.parse_type(&x.impl_ty, &context).ok();

            if let (Some(trait_ty), Some(impl_ty)) = (trait_ty, impl_ty) {
                if !impl_ty.should_ignore(context) {
                    return Ok(Some(MirTraitImpl { trait_ty, impl_ty }));
                }
            }

            None
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec())
}
