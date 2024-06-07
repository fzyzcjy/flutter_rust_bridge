pub(crate) mod attribute_parser;
mod auto_accessor_parser;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod misc;
pub(crate) mod reader;
pub(crate) mod sanity_checker;
mod trait_impl_parser;
pub(crate) mod type_parser;

use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::skip::MirSkip;
use crate::codegen::parser::mir::auto_accessor_parser::parse_auto_accessors;
use crate::codegen::parser::mir::function_parser::structs::ParseFunctionOutput;
use crate::codegen::parser::mir::function_parser::FunctionParser;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::sanity_checker::opaque_inside_translatable_checker::check_opaque_inside_translatable;
use crate::codegen::parser::mir::sanity_checker::unused_checker::get_unused_types;
use crate::codegen::parser::mir::type_parser::TypeParser;
use itertools::{concat, Itertools};
use std::collections::HashMap;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    hir_flat: &HirFlatPack,
) -> anyhow::Result<MirPack> {
    let structs_map = hir_flat.structs_map();
    let enums_map = hir_flat.enums_map();
    let traits_map = hir_flat.traits_map();
    let mut type_parser = TypeParser::new(
        structs_map.clone(),
        enums_map.clone(),
        traits_map,
        hir_flat.types_map(),
    );

    let (mir_funcs, mir_skips) =
        parse_mir_funcs(config, &hir_flat.functions, &mut type_parser, &structs_map)?;
    let trait_impls = trait_impl_parser::parse(
        &hir_flat.trait_impls,
        &mut type_parser,
        config.default_stream_sink_codec,
        config.default_rust_opaque_codec,
    )?;

    let (struct_pool, enum_pool, dart_code_of_type) = type_parser.consume();

    let mut ans = MirPack {
        funcs_all: mir_funcs,
        struct_pool,
        enum_pool,
        dart_code_of_type,
        existing_handler: hir_flat.existing_handler.clone(),
        unused_types: vec![],
        skipped_functions: mir_skips,
        trait_impls,
    };

    ans.unused_types = get_unused_types(
        &ans,
        &structs_map,
        &enums_map,
        &config.rust_input_namespace_pack,
    )?;

    check_opaque_inside_translatable(&ans);

    Ok(ans)
}

fn parse_mir_funcs(
    config: &ParserMirInternalConfig,
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
    src_structs: &HashMap<String, &HirFlatStruct>,
) -> anyhow::Result<(Vec<MirFunc>, Vec<MirSkip>)> {
    let mut function_parser = FunctionParser::new(type_parser);

    let (mir_funcs_normal, mir_skips): (Vec<_>, Vec<_>) = src_fns
        .iter()
        // Sort to make things stable. The order of parsing functions will affect things like, e.g.,
        // which file an opaque type is put in.
        .sorted_by_key(|f| f.owner_and_name_for_dedup())
        .map(|f| {
            function_parser.parse_function(
                f,
                &config.force_codec_mode_pack,
                config.default_stream_sink_codec,
                config.default_rust_opaque_codec,
                config.stop_on_error,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .partition(|item| matches!(item, ParseFunctionOutput::Ok(_)));
    let mir_funcs_normal = mir_funcs_normal.into_iter().map(|x| x.ok()).collect_vec();
    let mir_skips = (mir_skips.into_iter()).map(|x| x.skip()).collect_vec();

    let mir_funcs_auto_accessor = parse_auto_accessors(config, src_structs, type_parser)?;

    let mir_funcs = concat([mir_funcs_normal, mir_funcs_auto_accessor]);
    // let mir_funcs = dedup_funcs(mir_funcs);
    let mir_funcs = (mir_funcs.into_iter())
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .enumerate()
        .map(|(index, f)| MirFunc {
            id: Some((index + 1) as _),
            ..f
        })
        .collect_vec();

    Ok((mir_funcs, mir_skips))
}

// fn dedup_funcs(funcs: Vec<MirFunc>) -> Vec<MirFunc> {
//     funcs
//         .into_iter()
//         // Higher priority goes first
//         .sorted_by_key(|f| -f.override_priority.0)
//         .unique_by(|f| f.locator_dart_api())
//         .collect_vec()
// }
