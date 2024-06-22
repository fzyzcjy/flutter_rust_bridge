pub(crate) mod attribute;
pub(crate) mod custom_ser_des;
pub(crate) mod function;
pub(crate) mod lifetime_extractor;
pub(crate) mod lifetime_replacer;
pub(crate) mod misc;
pub(crate) mod trait_impl;
pub(crate) mod ty;

use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::misc::skip::{IrSkip, IrSkipReason};
use crate::codegen::parser::mir::internal_config::{
    ParserMirInternalConfig, RustInputNamespacePack,
};
use crate::codegen::parser::mir::parser::ty::TypeParser;
use crate::codegen::parser::mir::sanity_checker::opaque_inside_translatable_checker::check_opaque_inside_translatable;
use crate::codegen::parser::mir::sanity_checker::unused_checker::get_unused_types;
use crate::codegen::parser::mir::ParseMode;
use itertools::{concat, Itertools};
use std::collections::HashMap;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    ir_pack: &IrEarlyGeneratorPack,
    parse_mode: ParseMode,
) -> anyhow::Result<MirPack> {
    let hir_flat = &ir_pack.hir_flat_pack;
    let structs_map = hir_flat.structs_map();
    let enums_map = hir_flat.enums_map();

    let mut type_parser = TypeParser::new_from_pack(ir_pack);

    let trait_impls = trait_impl::parse(
        &hir_flat.trait_impls,
        &mut type_parser,
        config
            .rust_input_namespace_pack
            .rust_output_path_namespace
            .clone(),
        config.default_stream_sink_codec,
        config.default_rust_opaque_codec,
        config.enable_lifetime,
        config.type_64bit_int,
        parse_mode,
    )?;

    let custom_ser_des_infos = custom_ser_des::parse(
        &hir_flat.functions,
        &mut type_parser,
        &custom_ser_des::PartialContext {
            rust_output_path_namespace: config
                .rust_input_namespace_pack
                .rust_output_path_namespace
                .clone(),
            default_stream_sink_codec: config.default_stream_sink_codec,
            default_rust_opaque_codec: config.default_rust_opaque_codec,
            enable_lifetime: config.enable_lifetime,
            type_64bit_int: config.type_64bit_int,
            parse_mode,
        },
    )?;
    type_parser
        .custom_ser_des_infos
        .extend(custom_ser_des_infos);

    let (funcs_all, funcs_skip) = function::parse(
        config,
        &hir_flat.functions,
        &mut type_parser,
        &structs_map,
        parse_mode,
    )?;

    let (struct_pool, enum_pool, dart_code_of_type) = type_parser.consume();

    let mut ans = MirPack {
        funcs_all,
        struct_pool,
        enum_pool,
        dart_code_of_type,
        existing_handler: hir_flat.existing_handler.clone(),
        skips: vec![],
        trait_impls,
        extra_rust_output_code: hir_flat.extra_rust_output_code.clone(),
        extra_dart_output_code: hir_flat.extra_dart_output_code.clone(),
    };

    ans.skips = compute_skips(
        &ans,
        ir_pack,
        structs_map,
        enums_map,
        &config.rust_input_namespace_pack,
        funcs_skip,
    )?;

    check_opaque_inside_translatable(&ans);

    Ok(ans)
}

fn compute_skips(
    pack: &MirPack,
    ir_pack: &IrEarlyGeneratorPack,
    structs_map: HashMap<String, &HirFlatStruct>,
    enums_map: HashMap<String, &HirFlatEnum>,
    rust_input_namespace_pack: &RustInputNamespacePack,
    funcs_skip: Vec<IrSkip>,
) -> anyhow::Result<Vec<IrSkip>> {
    let unused_types = get_unused_types(pack, &structs_map, &enums_map, rust_input_namespace_pack)?;
    let unused_types_skip = (unused_types.into_iter())
        .map(|name| IrSkip {
            name: name.clone(),
            reason: IrSkipReason::IgnoreBecauseTypeNotUsedByPub,
        })
        .collect_vec();

    Ok(concat([
        ir_pack.hir_flat_pack.skips.clone(),
        unused_types_skip,
        funcs_skip,
    ]))
}

// TODO rm
// pub(crate) fn tentative_parse_trait_impls(
//     hir_flat: &HirFlatPack,
// ) -> anyhow::Result<Vec<MirTraitImpl>> {
//     let mut type_parser = TypeParser::new_from_hir_flat_pack(hir_flat);
//     trait_impl::parse(
//         &hir_flat.trait_impls,
//         &mut type_parser,
//         // randomly pick a value, which does not matter for this "tentative" purpose
//         CodecMode::Sse,
//         RustOpaqueCodecMode::Moi,
//     )
// }
