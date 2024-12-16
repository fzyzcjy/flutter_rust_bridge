use crate::codegen::ir::mir::ty::general_list::mir_list;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_slice(
        &mut self,
        type_slice: &syn::TypeSlice,
    ) -> anyhow::Result<MirType> {
        Ok(mir_list(self.parse_type(&type_slice.elem)?, true))
    }
}
