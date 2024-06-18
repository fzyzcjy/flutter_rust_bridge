use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::real::is_struct_or_enum_or_opaque_from_them;
use crate::codegen::parser::mir::parser::ty::trait_def::parse_type_trait;
use crate::codegen::parser::mir::parser::ty::{TypeParser, TypeParserParsingContext};
use crate::codegen::parser::mir::ParseMode;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

#[allow(clippy::too_many_arguments)]
pub(crate) fn parse(
    hir_trait_impls: &[HirFlatTraitImpl],
    type_parser: &mut TypeParser,
    rust_output_path_namespace: Namespace,
    default_stream_sink_codec: CodecMode,
    default_rust_opaque_codec: RustOpaqueCodecMode,
    enable_lifetime: bool,
    type_64bit_int: bool,
    parse_mode: ParseMode,
) -> anyhow::Result<Vec<MirTraitImpl>> {
    let context = TypeParserParsingContext {
        initiated_namespace: CrateName::self_crate().namespace(), // just a dummy value
        func_attributes: FrbAttributes::parse(&[])?,
        struct_or_enum_attributes: None,
        rust_output_path_namespace,
        default_stream_sink_codec,
        default_rust_opaque_codec,
        owner: None,
        enable_lifetime,
        type_64bit_int,
        forbid_type_self: false,
        parse_mode,
    };

    Ok((hir_trait_impls.iter())
        .map(|x| {
            let trait_ty = parse_type_trait(&x.trait_name, type_parser);
            let impl_ty = type_parser.parse_type(&x.impl_ty, &context).ok();

            if let (Some(trait_ty), Some(impl_ty)) = (trait_ty, impl_ty) {
                return Ok(Some(MirTraitImpl { trait_ty, impl_ty }));
            }

            Ok(None)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .filter(|ty| {
            !ty.impl_ty.should_ignore(type_parser)
                && is_struct_or_enum_or_opaque_from_them(&ty.impl_ty)
        })
        .collect_vec())
}
