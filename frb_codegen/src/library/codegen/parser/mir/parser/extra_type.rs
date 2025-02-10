use crate::codegen::ir::hir::flat::struct_or_enum::{
    HirFlatEnum, HirFlatStruct, HirFlatStructOrEnum,
};
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncAccessorMode, MirFuncArgMode, MirFuncImplMode, MirFuncMode, MirFuncOutput,
    MirFuncOwnerInfo,
};
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::misc::skip::MirFuncOrSkip;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::auto_accessor::create_simplified_parsing_context;
use crate::codegen::parser::mir::parser::function::real::compute_codec_mode_pack;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use crate::codegen::parser::mir::ParseMode;
use itertools::concat;
use std::collections::HashMap;

use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    src_structs: &HashMap<String, &HirFlatStruct>,
    src_enums: &HashMap<String, &HirFlatEnum>,
    type_parser: &mut TypeParser,
    parse_mode: ParseMode,
) -> anyhow::Result<Vec<MirFuncOrSkip>> {
    Ok(concat([
        parse_structs_or_enums(src_structs, config, type_parser, parse_mode)?,
        parse_structs_or_enums(src_enums, config, type_parser, parse_mode)?,
    ]))
}

fn parse_structs_or_enums<Item: SynItemStructOrEnum>(
    items: &HashMap<String, &HirFlatStructOrEnum<Item>>,
    config: &ParserMirInternalConfig,
    type_parser: &mut TypeParser,
    parse_mode: ParseMode,
) -> anyhow::Result<Vec<MirFuncOrSkip>> {
    (items.values())
        .filter(|item| (config.rust_input_namespace_pack).is_interest(&item.name.namespace))
        .filter(|item| {
            let attrs = FrbAttributes::parse(item.src.attrs())
                .unwrap_or_else(|_| FrbAttributes::parse(&[]).unwrap());
            attrs.unignore()
        })
        .map(|item| parse_item(config, item, type_parser, parse_mode))
        .collect()
}

fn parse_item<Item: SynItemStructOrEnum>(
    config: &ParserMirInternalConfig,
    item: &HirFlatStructOrEnum<Item>,
    type_parser: &mut TypeParser,
    parse_mode: ParseMode,
) -> anyhow::Result<MirFuncOrSkip> {
    let context =
        create_simplified_parsing_context(item.name.namespace.clone(), config, parse_mode)?;
    let ty_direct_parse = type_parser.parse_type(&syn::parse_str(&item.name.name)?, &context)?;
    let fn_name = format!("dummy_for_unignore_{}", item.name.safe_ident());

    Ok(MirFuncOrSkip::Value(MirFunc {
        namespace: item.name.namespace.clone(),
        name: MirIdent::new(fn_name, None),
        id: None,
        inputs: vec![],
        output: MirFuncOutput {
            normal: ty_direct_parse,
            error: None,
        },
        owner: MirFuncOwnerInfo::Function,
        mode: MirFuncMode::Sync,
        stream_dart_await: false,
        rust_async: false,
        initializer: false,
        hidden: true,
        arg_mode: MirFuncArgMode::Positional,
        accessor: Some(MirFuncAccessorMode::Getter),
        comments: vec![],
        codec_mode_pack: compute_codec_mode_pack(
            &FrbAttributes::parse(&[])?,
            &config.force_codec_mode_pack,
        ),
        rust_call_code: None,
        rust_aop_after: None,
        impl_mode: MirFuncImplMode::Normal,
        src_lineno_pseudo: item.src.span().start().line,
    }))
}
