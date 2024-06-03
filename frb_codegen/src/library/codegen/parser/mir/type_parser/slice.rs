use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_slice(
        &mut self,
        type_slice: &syn::TypeSlice,
    ) -> anyhow::Result<MirType> {
        log::info!("parse_type_slice {type_slice:?}");
        unimplemented!()
    }
}
