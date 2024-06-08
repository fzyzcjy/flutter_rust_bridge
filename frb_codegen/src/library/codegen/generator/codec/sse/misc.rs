use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirType;
use itertools::Itertools;

pub(crate) fn with_sse_extra_types(raw: &[MirType]) -> Vec<MirType> {
    [
        raw.to_owned(),
        vec![
            // Many SSE codec needs these types to encode length, existence, etc
            // So we unconditionally generate code for these
            MirType::Primitive(MirTypePrimitive::I32),
            MirType::Primitive(MirTypePrimitive::Bool),
        ],
    ]
    .concat()
    .into_iter()
    .unique()
    .collect_vec()
}
