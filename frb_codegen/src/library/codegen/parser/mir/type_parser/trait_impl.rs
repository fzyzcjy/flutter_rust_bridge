use crate::codegen::ir::mir::ty::delegate::MirTypeDelegateDynTrait;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_trait_impls(&mut self) -> anyhow::Result<Vec<MirTypeDelegateDynTrait>> {
        todo!()
    }
}
