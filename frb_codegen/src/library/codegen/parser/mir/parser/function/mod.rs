use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::skip::MirSkip;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::function::func_or_skip::MirFuncOrSkip;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use itertools::{concat, Itertools};
use std::collections::HashMap;

pub(crate) mod auto_accessor;
pub(crate) mod func_or_skip;
pub(crate) mod real;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
    src_structs: &HashMap<String, &HirFlatStruct>,
) -> anyhow::Result<(Vec<MirFunc>, Vec<MirSkip>)> {
    let items = concat([
        real::parse(src_fns, type_parser, config)?,
        auto_accessor::parse(config, src_structs, type_parser)?,
    ]);
    let (funcs, skips) = split_func_and_skip(items);
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

fn split_func_and_skip(items: Vec<MirFuncOrSkip>) -> (Vec<MirFunc>, Vec<MirSkip>) {
    let (funcs, skips): (Vec<_>, Vec<_>) =
        (items.into_iter()).partition(|item| matches!(item, MirFuncOrSkip::Ok(_)));
    let funcs = funcs.into_iter().map(|x| x.ok()).collect_vec();
    let skips = skips.into_iter().map(|x| x.skip()).collect_vec();
    (funcs, skips)
}
