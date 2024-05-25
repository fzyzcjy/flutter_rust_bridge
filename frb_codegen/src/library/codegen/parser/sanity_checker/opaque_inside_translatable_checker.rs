use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn check_opaque_inside_translatable(pack: &IrPack) {
    (pack.distinct_types(None).into_iter())
        .map(handle_type)
        .collect()
}

fn handle_type(ty: IrType) {
    TODO
}
