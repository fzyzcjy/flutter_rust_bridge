use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_maybe_lifetimeable(
        &mut self,
        raw: MirTypeRustAutoOpaqueImplicit,
    ) -> anyhow::Result<MirType> {
        todo!()
    }
}
