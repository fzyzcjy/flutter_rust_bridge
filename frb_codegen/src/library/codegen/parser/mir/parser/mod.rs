pub(crate) mod attribute;
pub(crate) mod function;
pub(crate) mod misc;
pub(crate) mod trait_impl;
pub(crate) mod ty;

use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use crate::codegen::parser::mir::sanity_checker::opaque_inside_translatable_checker::check_opaque_inside_translatable;
use crate::codegen::parser::mir::sanity_checker::unused_checker::get_unused_types;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    hir_flat: &HirFlatPack,
) -> anyhow::Result<MirPack> {
    let structs_map = hir_flat.structs_map();
    let enums_map = hir_flat.enums_map();

    let mut type_parser = create_type_parser(hir_flat);

    let trait_impls = trait_impl::parse(
        &hir_flat.trait_impls,
        &mut type_parser,
        config.default_stream_sink_codec,
        config.default_rust_opaque_codec,
    )?;

    let (funcs_all, skipped_functions) =
        function::parse(config, &hir_flat.functions, &mut type_parser, &structs_map)?;

    let (struct_pool, enum_pool, dart_code_of_type) = type_parser.consume();

    let mut ans = MirPack {
        funcs_all,
        struct_pool,
        enum_pool,
        dart_code_of_type,
        existing_handler: hir_flat.existing_handler.clone(),
        unused_types: vec![],
        skipped_functions,
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

fn create_type_parser(hir_flat: &HirFlatPack) -> TypeParser {
    TypeParser::new(
        hir_flat.structs_map(),
        hir_flat.enums_map(),
        hir_flat.traits_map(),
        hir_flat.types_map(),
    )
}
