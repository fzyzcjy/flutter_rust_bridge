use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;

pub(crate) fn with_sse_extra_types(raw: &[IrType]) -> Vec<IrType> {
    [
        raw.to_owned(),
        vec![
            // SSE's
            IrType::Primitive(IrTypePrimitive::I32),
        ],
    ]
}
