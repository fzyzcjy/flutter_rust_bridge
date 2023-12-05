use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use syn::ReturnType;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_return_type(&mut self, ty: &ReturnType) -> anyhow::Result<IrType> {
        match &ty {
            ReturnType::Default => Ok(Primitive(IrTypePrimitive::Unit)),
            ReturnType::Type(_, ret_ty) => self.parse_type(ret_ty)?,
        }
    }
}
