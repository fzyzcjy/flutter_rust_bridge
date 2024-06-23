use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::misc::skip::{IrSkip, IrValueOrSkip};
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use crate::codegen::parser::mir::ParseMode;
use itertools::{concat, Itertools};
use std::collections::HashMap;

pub(crate) mod auto_accessor;
pub(crate) mod real;
pub(crate) mod ui_related;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
    src_structs: &HashMap<String, &HirFlatStruct>,
    parse_mode: ParseMode,
) -> anyhow::Result<(Vec<MirFunc>, Vec<IrSkip>)> {
    let items = concat([
        real::parse(src_fns, type_parser, config, parse_mode)?,
        auto_accessor::parse(config, src_structs, type_parser, parse_mode)?,
    ]);
    let (funcs, skips) = IrValueOrSkip::split(items);
    let funcs = sort_and_add_func_id(funcs);
    Ok((funcs, skips))
}

fn sort_and_add_func_id(funcs: Vec<MirFunc>) -> Vec<MirFunc> {
    (funcs.into_iter())
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .enumerate()
        .map(|(index, f)| MirFunc {
            id: Some((index + 1) as _),
            ..f
        })
        .collect_vec()
}
