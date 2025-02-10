use crate::codegen::ir::hir::flat::struct_or_enum::{
    HirFlatEnum, HirFlatStruct, HirFlatStructOrEnum,
};
use crate::codegen::ir::hir::misc::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::ir::misc::skip::MirFuncOrSkip;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
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
) -> anyhow::Result<Vec<MirFuncOrSkip>> {
    Ok(concat([
        parse_structs_or_enums(src_structs)?,
        parse_structs_or_enums(src_enums)?,
    ]))
}

fn parse_structs_or_enums<Item: SynItemStructOrEnum>(
    items: &[HirFlatStructOrEnum<Item>],
) -> anyhow::Result<Vec<MirFuncOrSkip>> {
    TODO
}
