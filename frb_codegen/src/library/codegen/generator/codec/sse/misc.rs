use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

pub(crate) fn with_sse_extra_types(raw: &[IrType]) -> Vec<IrType> {
    [
        raw.to_owned(),
        vec![
            // Many SSE codec needs these types to encode length, existence, etc
            // So we unconditionally generate code for these
            IrType::Primitive(IrTypePrimitive::I32),
            IrType::Primitive(IrTypePrimitive::Bool),
        ],
    ]
    .concat()
    .into_iter()
    .unique()
    .collect_vec()
}
