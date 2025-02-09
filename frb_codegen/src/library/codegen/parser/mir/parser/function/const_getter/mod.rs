use crate::codegen::ir::hir::flat::constant::HirFlatConstant;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::misc::skip::MirFuncOrSkip;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use crate::codegen::parser::mir::ParseMode;
use std::collections::HashMap;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    src_constants: &[HirFlatConstant],
    type_parser: &mut TypeParser,
    parse_mode: ParseMode,
) -> anyhow::Result<Vec<MirFuncOrSkip>> {
    todo!()
}
