use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegateArray};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Delegate, Primitive};
use crate::codegen::parser::type_parser::TypeParser;
use quote::ToTokens;

impl<'a> TypeParser<'a> {
    pub(crate) fn parse_type(&mut self, ty: &syn::Type) -> IrType {
        let resolve_ty = self.resolve_alias(ty).clone();

        match resolve_ty.clone() {
            syn::Type::Path(path) => self.convert_path_to_ir_type(&path).unwrap(),
            syn::Type::Array(syn::TypeArray { elem, len, .. }) => {
                let length: usize = match len {
                    syn::Expr::Lit(lit) => match &lit.lit {
                        syn::Lit::Int(x) => x.base10_parse().unwrap(),
                        _ => panic!("Cannot parse array length"),
                    },
                    _ => panic!("Cannot parse array length"),
                };
                match self.parse_type(&elem) {
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
            syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
                if elems.is_empty() {
                    Primitive(IrTypePrimitive::Unit)
                } else {
                    self.convert_tuple_to_ir_type(elems)
                }
            }
            _ => IrType::Unencodable(IrTypeUnencodable {
                string: resolve_ty.to_token_stream().to_string(),
                segments: vec![],
            }),
        }
    }
}
