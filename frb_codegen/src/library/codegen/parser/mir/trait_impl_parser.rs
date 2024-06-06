use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::mir::func::MirFuncOwnerInfo;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::attribute_parser::FrbAttributes;
use crate::codegen::parser::mir::type_parser::{TypeParser, TypeParserParsingContext};
use crate::if_then_some;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::Namespace;
use crate::utils::syn_utils::ty_to_string;
use itertools::Itertools;

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

    Ok((hir_trait_impls.iter())
        .map(|x| {
            let trait_ty_raw = type_parser.parse_type(&syn::parse_str(&x.trait_name)?, &context)?;
            let impl_ty = type_parser.parse_type(&x.impl_ty, &context)?;

            Ok(
                if_then_some!(let MirType::TraitDef(trait_ty) = trait_ty_raw, trait_ty)
                    .map(|trait_ty| MirTraitImpl { trait_ty, impl_ty }),
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .filter_map(|x| x)
        .collect_vec())
}
