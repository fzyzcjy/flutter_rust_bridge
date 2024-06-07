pub(crate) mod internal_config;
pub(crate) mod parser;
pub(crate) mod sanity_checker;
pub(crate) mod transformer;

use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::skip::MirSkip;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::function::auto_accessor::parse;
use crate::codegen::parser::mir::parser::function::real::structs::ParseFunctionOutput;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use crate::codegen::parser::mir::sanity_checker::opaque_inside_translatable_checker::check_opaque_inside_translatable;
use crate::codegen::parser::mir::sanity_checker::unused_checker::get_unused_types;
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
        parser::function::parse(config, &hir_flat.functions, &mut type_parser, &structs_map)?;
    let trait_impls = parser::trait_impl::parse(
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
