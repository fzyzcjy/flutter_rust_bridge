use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegateArray};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Delegate, Primitive};
use crate::codegen::parser::type_parser::TypeParser;
use syn::Expr;

impl<'a> TypeParser<'a> {
    pub(crate) fn parse_type_array(&mut self, type_array: &syn::TypeArray) -> IrType {
        let length: usize = match &type_array.len {
            Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Int(x) => x.base10_parse().unwrap(),
                _ => panic!("Cannot parse array length"),
            },
            _ => panic!("Cannot parse array length"),
        };
        match self.parse_type(&type_array.elem) {
            Primitive(primitive) => {
                Delegate(IrTypeDelegate::Array(IrTypeDelegateArray::PrimitiveArray {
                    length,
                    primitive,
                }))
            }
            others => Delegate(IrTypeDelegate::Array(IrTypeDelegateArray::GeneralArray {
                length,
                general: Box::new(others),
            })),
        }
    }
}
