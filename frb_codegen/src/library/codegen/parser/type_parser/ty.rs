use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Delegate, Primitive};
use crate::codegen::parser::type_parser::TypeParser;
use quote::ToTokens;
use syn::Type;

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
}
