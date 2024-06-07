mod field;

use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::mir::func::{MirFunc, MirFuncAccessorMode, OwnershipMode};
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::{MirContext, MirType};
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::misc::extract_src_types_in_paths;
use crate::codegen::parser::mir::sanity_checker::auto_accessor_checker;
use crate::codegen::parser::mir::parser::ty::{
    TypeParser, TypeParserParsingContext, TypeParserWithContext,
};
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::NamespacedName;
use field::parse_auto_accessor_of_field;
use itertools::Itertools;
use std::collections::HashMap;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    src_structs: &HashMap<String, &HirFlatStruct>,
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirFunc>> {
    let src_structs_in_paths =
        extract_src_types_in_paths(src_structs, &config.rust_input_namespace_pack)?;

    let infos = src_structs_in_paths
        .iter()
        .map(|struct_name| parse_auto_accessors_of_struct(config, struct_name, type_parser))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    auto_accessor_checker::report(
        &infos
            .iter()
            .flat_map(|x| x.sanity_check_hint.clone())
            .collect_vec(),
    );

    Ok(infos.into_iter().map(|x| x.mir_func).collect_vec())
}

fn parse_auto_accessors_of_struct(
    config: &ParserMirInternalConfig,
    struct_name: &NamespacedName,
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirFuncAndSanityCheckInfo>> {
    let context = create_parsing_context(
        struct_name,
        config.default_stream_sink_codec,
        config.default_rust_opaque_codec,
    )?;

    let ty_direct_parse =
        match type_parser.parse_type(&syn::parse_str(&struct_name.name)?, &context) {
            Ok(value) => value,
            // We do not care about parsing errors here (e.g. some type that we do not support)
            Err(_) => return Ok(vec![]),
        };
    if !matches!(ty_direct_parse, MirType::RustAutoOpaqueImplicit(_)) {
        return Ok(vec![]);
    }
    if ty_direct_parse.should_ignore(type_parser) {
        return Ok(vec![]);
    }

    let ty_struct_ref = TypeParserWithContext::new(type_parser, &context)
        .parse_type_path_data_struct(&(&struct_name.name, &[]), Some(false));
    let ty_struct_ident = if let Ok(Some(MirType::StructRef(mir))) = ty_struct_ref {
        mir.ident
    } else {
        return Ok(vec![]);
    };
    let ty_struct = &type_parser.struct_pool()[&ty_struct_ident].to_owned();

    (ty_struct.fields.iter())
        .filter(|field| field.is_rust_public.unwrap() && !is_ty_opaque_reference_type(&field.ty))
        .flat_map(|field| {
            [MirFuncAccessorMode::Getter, MirFuncAccessorMode::Setter]
                .into_iter()
                .map(|accessor_mode| {
                    parse_auto_accessor_of_field(
                        config,
                        struct_name,
                        field,
                        accessor_mode,
                        &ty_direct_parse,
                        type_parser,
                        &context,
                    )
                })
                .collect_vec()
        })
        .collect()
}

fn create_parsing_context(
    struct_name: &NamespacedName,
    default_stream_sink_codec: CodecMode,
    default_rust_opaque_codec: RustOpaqueCodecMode,
) -> anyhow::Result<TypeParserParsingContext> {
    Ok(TypeParserParsingContext {
        initiated_namespace: struct_name.namespace.to_owned(),
        func_attributes: FrbAttributes::parse(&[])?,
        default_stream_sink_codec,
        default_rust_opaque_codec,
        owner: None,
    })
}

struct MirFuncAndSanityCheckInfo {
    mir_func: MirFunc,
    sanity_check_hint: Option<auto_accessor_checker::SanityCheckHint>,
}

fn is_ty_opaque_reference_type(ty: &MirType) -> bool {
    if let MirType::RustAutoOpaqueImplicit(inner) = ty {
        inner.ownership_mode != OwnershipMode::Owned
    } else {
        false
    }
}
