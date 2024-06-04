pub(crate) mod attribute_parser;
mod auto_accessor_parser;
mod existing_handler;
mod file_reader;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod misc;
pub(crate) mod reader;
pub(crate) mod sanity_checker;
pub(crate) mod type_parser;

use crate::codegen::ir::hir::flat::HirFlatCrate;
use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStruct;
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
    hir_flat_crate: &HirFlatCrate,
) -> anyhow::Result<MirPack> {
    let mut type_parser = TypeParser::new(
        hir_flat_crate.structs.clone(),
        hir_flat_crate.enums.clone(),
        hir_flat_crate.types.clone(),
    );

    let (mir_funcs, mir_skips) = parse_mir_funcs(
        config,
        &hir_flat_crate.functions,
        &mut type_parser,
        &hir_flat_crate.structs,
    )?;

    let existing_handlers = existing_handler::parse_existing_handlers(
        &hir_flat_crate.modules,
        &config.rust_input_namespace_pack,
    )?;

    let (struct_pool, enum_pool, dart_code_of_type) = type_parser.consume();

    let mut ans = MirPack {
        funcs: mir_funcs,
        struct_pool,
        enum_pool,
        dart_code_of_type,
        existing_handler: existing_handlers.first().cloned(),
        unused_types: vec![],
        skipped_functions: mir_skips,
    };

    ans.unused_types = get_unused_types(
        &ans,
        &hir_flat_crate.structs,
        &hir_flat_crate.enums,
        &config.rust_input_namespace_pack,
    )?;

    check_opaque_inside_translatable(&ans);

    Ok(ans)
}

fn parse_mir_funcs(
    config: &ParserMirInternalConfig,
    src_fns: &[&HirFunction],
    type_parser: &mut TypeParser,
    src_structs: &HashMap<String, &HirStruct>,
) -> anyhow::Result<(Vec<MirFunc>, Vec<MirSkip>)> {
    let mut function_parser = FunctionParser::new(type_parser);

    let (mir_funcs_normal, mir_skips): (Vec<_>, Vec<_>) = src_fns
        .iter()
        .map(|f| {
            function_parser.parse_function(
                f,
                &config.force_codec_mode_pack,
                config.default_stream_sink_codec,
                config.default_rust_opaque_codec,
            )
        })
        .partition(|item| matches!(item, ParseFunctionOutput::Ok(_)));
    let mir_funcs_normal = mir_funcs_normal.into_iter().map(|x| x.ok()).collect_vec();
    let mir_skips = (mir_skips.into_iter()).map(|x| x.skip()).collect_vec();

    let mir_funcs_auto_accessor = parse_auto_accessors(config, src_structs, type_parser)?;

    let mir_funcs = concat([mir_funcs_normal, mir_funcs_auto_accessor]);
    let mir_funcs = dedup_funcs(mir_funcs);
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

fn dedup_funcs(funcs: Vec<MirFunc>) -> Vec<MirFunc> {
    funcs
        .into_iter()
        // Higher priority goes first
        .sorted_by_key(|f| -f.override_priority.0)
        .unique_by(|f| f.locator_dart_api())
        .collect_vec()
}
