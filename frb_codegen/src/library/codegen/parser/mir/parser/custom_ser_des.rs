use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::mir::custom_ser_des::{MirCustomSerDes, MirCustomSerDesHalf};
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use itertools::Itertools;

pub(crate) fn parse(
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirCustomSerDes>> {
    TODO
}

fn parse_one(func: &HirFlatFunction) -> anyhow::Result<Option<MirCustomSerDesHalf>> {
    let attrs = FrbAttributes::parse(func.item_fn.attrs())?;

    attrs.dart2rust();
    attrs.rust2dart();

    TODO;
}
