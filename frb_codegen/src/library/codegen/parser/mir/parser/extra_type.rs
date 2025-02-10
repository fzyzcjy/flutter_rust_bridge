use crate::codegen::ir::hir::flat::struct_or_enum::{
    HirFlatEnum, HirFlatStruct, HirFlatStructOrEnum,
};
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::ir::mir::extra_type::MirExtraType;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::auto_accessor::create_simplified_parsing_context;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use crate::codegen::parser::mir::ParseMode;
use itertools::concat;
use std::collections::HashMap;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    src_structs: &HashMap<String, &HirFlatStruct>,
    src_enums: &HashMap<String, &HirFlatEnum>,
    type_parser: &mut TypeParser,
    parse_mode: ParseMode,
) -> anyhow::Result<Vec<MirExtraType>> {
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
) -> anyhow::Result<Vec<MirExtraType>> {
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
) -> anyhow::Result<MirExtraType> {
    let context =
        create_simplified_parsing_context(item.name.namespace.clone(), config, parse_mode)?;
    type_parser.parse_type(&syn::parse_str(&item.name.name)?, &context)
}
