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
use crate::codegen::ir::mir::namespace::NamespacedName;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::parser::mir::auto_accessor_parser::parse_auto_accessors;
use crate::codegen::parser::mir::function_parser::FunctionParser;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::sanity_checker::opaque_inside_translatable_checker::check_opaque_inside_translatable;
use crate::codegen::parser::mir::sanity_checker::unused_checker::get_unused_types;
use crate::codegen::parser::mir::type_parser::TypeParser;
use itertools::{concat, Itertools};
use std::collections::HashMap;
use syn::Visibility;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    hir_flat_crate: &HirFlatCrate,
) -> anyhow::Result<MirPack> {
    let (src_fns_interest, src_fns_skipped): (Vec<_>, Vec<_>) = (hir_flat_crate.functions.iter())
        .partition(|item| matches!(item.inner.vis(), Visibility::Public(_)));

    let mut type_parser = TypeParser::new(
        hir_flat_crate.structs.clone(),
        hir_flat_crate.enums.clone(),
        hir_flat_crate.types.clone(),
    );

    let mir_funcs = parse_mir_funcs(
        config,
        &src_fns_interest,
        &mut type_parser,
        &hir_flat_crate.structs,
    )?;

    let existing_handlers = existing_handler::parse_existing_handlers(&hir_flat_crate.modules)?;

    let (struct_pool, enum_pool, dart_code_of_type) = type_parser.consume();

    let mut ans = MirPack {
        funcs: mir_funcs,
        struct_pool,
        enum_pool,
        dart_code_of_type,
        existing_handler: existing_handlers.first().cloned(),
        unused_types: vec![],
        skipped_functions: compute_skipped_functions(&src_fns_skipped)?,
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
) -> anyhow::Result<Vec<MirFunc>> {
    let mut function_parser = FunctionParser::new(type_parser);

    let mir_funcs_normal = src_fns
        .iter()
        .map(|f| {
            function_parser.parse_function(
                &f.inner,
                &f.namespace,
                &config.force_codec_mode_pack,
                config.default_stream_sink_codec,
                config.default_rust_opaque_codec,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    let mir_funcs_auto_accessor = parse_auto_accessors(config, src_structs, type_parser)?;

    Ok(concat([mir_funcs_normal, mir_funcs_auto_accessor])
        .into_iter()
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .enumerate()
        .map(|(index, f)| MirFunc {
            id: Some((index + 1) as _),
            ..f
        })
        .collect_vec())
}

fn compute_skipped_functions(
    src_fns_skipped: &[&HirFunction],
) -> anyhow::Result<Vec<NamespacedName>> {
    src_fns_skipped
        .iter()
        .map(|x| {
            Ok(NamespacedName::new(
                x.namespace.to_owned(),
                x.inner.sig().ident.to_string(),
            ))
        })
        .collect()
}
