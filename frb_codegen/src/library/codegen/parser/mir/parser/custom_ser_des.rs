use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::mir::custom_ser_des::{MirCustomSerDes, MirCustomSerDesHalf};
use crate::codegen::parser::mir::parser::attribute::{FrbAttributeSerDes, FrbAttributes};
use crate::codegen::parser::mir::parser::function;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use itertools::Itertools;

pub(crate) fn parse(
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirCustomSerDes>> {
    TODO
}

fn parse_function(func: &HirFlatFunction) -> anyhow::Result<Option<MirCustomSerDesHalf>> {
    let attrs = FrbAttributes::parse(func.item_fn.attrs())?;

    if let Some(info) = attrs.dart2rust() {
        return Ok(Some(parse_function_inner(func, info)?));
    }
    if let Some(info) = attrs.rust2dart() {
        return Ok(Some(parse_function_inner(func, info)?));
    }

    Ok(None)
}

fn parse_function_inner(
    function: &HirFlatFunction,
    attr_ser_des: FrbAttributeSerDes,
) -> anyhow::Result<MirCustomSerDesHalf> {
    TODO
}
