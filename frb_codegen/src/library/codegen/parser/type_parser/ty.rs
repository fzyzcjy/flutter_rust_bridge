use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegateArray};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::record::IrTypeRecord;
use crate::codegen::ir::ty::structure::{IrStruct, IrStructIdent, IrTypeStructRef};
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Delegate, Primitive};
use crate::codegen::parser::type_parser::TypeParser;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Expr, Token, Type};

impl<'a> TypeParser<'a> {
    pub(crate) fn parse_type(&mut self, ty: &Type) -> IrType {
        let resolve_ty = self.resolve_alias(ty).clone();

        match resolve_ty.clone() {
            Type::Path(path) => self.parse_type_path(&path).unwrap(),
            Type::Array(syn::TypeArray { elem, len, .. }) => self.parse_type_array(&elem, len),
            Type::Tuple(syn::TypeTuple { elems, .. }) => {
                if elems.is_empty() {
                    Primitive(IrTypePrimitive::Unit)
                } else {
                    self.parse_type_tuple(elems)
                }
            }
            _ => IrType::Unencodable(IrTypeUnencodable {
                string: resolve_ty.to_token_stream().to_string(),
                segments: vec![],
            }),
        }
    }

    fn parse_type_array(&mut self, elem: &Box<Type>, len: Expr) -> IrType {
        let length: usize = match len {
            Expr::Lit(lit) => match &lit.lit {
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

    fn parse_type_tuple(&mut self, elems: Punctuated<Type, Token![,]>) -> IrType {
        let values = elems
            .iter()
            .map(|elem| self.parse_type(elem))
            .collect::<Vec<_>>();
        let safe_ident = values
            .iter()
            .map(IrType::safe_ident)
            .collect::<Vec<_>>()
            .join("_");
        let safe_ident = format!("__record__{safe_ident}");
        self.struct_pool.insert(
            safe_ident.clone(),
            IrStruct {
                name: safe_ident.clone(),
                wrapper_name: None,
                path: None,
                is_fields_named: true,
                dart_metadata: vec![],
                comments: vec![],
                fields: values
                    .iter()
                    .enumerate()
                    .map(|(idx, ty)| IrField {
                        ty: ty.clone(),
                        name: IrIdent::new(format!("field{idx}")),
                        is_final: true,
                        comments: vec![],
                        default: None,
                        settings: Default::default(),
                    })
                    .collect(),
            },
        );
        IrType::Record(IrTypeRecord {
            inner: IrTypeStructRef {
                // name: safe_ident,
                // freezed: false,
                // empty: false,
                ident: IrStructIdent(safe_ident),
                is_exception: false,
            },
            values: values.into_boxed_slice(),
        })
    }
}
