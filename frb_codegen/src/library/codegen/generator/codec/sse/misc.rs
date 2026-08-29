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

#[cfg(test)]
mod tests {
    use super::*;

    /// Appends required SSE bookkeeping types once while preserving input order.
    #[test]
    fn extra_types_are_unique_and_keep_existing_order() {
        let output = with_sse_extra_types(&[
            MirType::Primitive(MirTypePrimitive::Bool),
            MirType::Primitive(MirTypePrimitive::U8),
            MirType::Primitive(MirTypePrimitive::I32),
        ]);

        assert_eq!(
            output,
            [
                MirType::Primitive(MirTypePrimitive::Bool),
                MirType::Primitive(MirTypePrimitive::U8),
                MirType::Primitive(MirTypePrimitive::I32),
            ]
        );
    }
}
