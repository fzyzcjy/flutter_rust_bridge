use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::skip::MirSkip;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use itertools::{concat, Itertools};
use std::collections::HashMap;

pub(crate) mod auto_accessor;
pub(crate) mod real;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
    src_structs: &HashMap<String, &HirFlatStruct>,
) -> anyhow::Result<(Vec<MirFunc>, Vec<MirSkip>)> {
    let (mir_funcs_normal, mir_skips) =
        real::parse(src_fns, type_parser, config)?;
    let mir_funcs_auto_accessor = auto_accessor::parse(config, src_structs, type_parser)?;

    let mir_funcs = concat([mir_funcs_normal, mir_funcs_auto_accessor]);
    let mir_funcs = (mir_funcs.into_iter())
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .enumerate()
        .map(|(index, f)| MirFunc {
            id: Some((index + 1) as _),
            ..f
        })
        .collect_vec();

    Ok((mir_funcs, mir_skips))
}
