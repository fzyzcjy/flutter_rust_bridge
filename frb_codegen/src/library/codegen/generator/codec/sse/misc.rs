use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
use crate::codegen::ir::mir::ty::optional::MirTypeOptional;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::{Boxed, DartFn, Optional};
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
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

pub(crate) fn rust_sse_codec_type(ty: &MirType) -> String {
    rust_sse_codec_mir_type(ty.clone()).rust_api_type()
}

pub(crate) fn rust_sse_codec_mir_type(ty: MirType) -> MirType {
    match ty {
        DartFn(mir) => mir.get_delegate(),
        Optional(mir) => Optional(MirTypeOptional::new(rust_sse_codec_mir_type(*mir.inner))),
        Boxed(mir) if mir.exist_in_real_api => Boxed(MirTypeBoxed {
            exist_in_real_api: true,
            inner: Box::new(rust_sse_codec_mir_type(*mir.inner)),
        }),
        Boxed(mir) => rust_sse_codec_mir_type(*mir.inner),
        _ => ty,
    }
}

pub(crate) fn contains_dart_fn(ty: &MirType) -> bool {
    match ty {
        DartFn(_) => true,
        Optional(mir) => contains_dart_fn(&mir.inner),
        Boxed(mir) => contains_dart_fn(&mir.inner),
        _ => false,
    }
}
