use crate::codegen::ir::ty::ownership::{IrTypeOwnership, IrTypeOwnershipMode};
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_reference(
        &mut self,
        type_reference: &syn::TypeReference,
    ) -> anyhow::Result<IrType> {
        let mode = if type_reference.mutability.is_some() {
            IrTypeOwnershipMode::RefMut
        } else {
            IrTypeOwnershipMode::Ref
        };

        Ok(IrType::Ownership(IrTypeOwnership {
            mode,
            inner: Box::new(self.parse_type(&*type_reference.elem)?),
        }))
    }
}
