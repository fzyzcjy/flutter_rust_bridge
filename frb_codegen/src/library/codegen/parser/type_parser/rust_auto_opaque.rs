use crate::codegen::ir::pack::DistinctTypeGatherer;
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType::RustOpaque;
use crate::codegen::ir::ty::{IrType, IrTypeModifier};
use crate::codegen::parser::type_parser::rust_opaque::SimpleParsedTypesParserInfo;
use crate::codegen::parser::type_parser::unencodable::ArgsRefs::Generic;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::library::codegen::ir::ty::IrTypeTrait;
use quote::ToTokens;
use std::collections::HashMap;
use syn::Type;
use IrType::RustAutoOpaque;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn transform_type_rust_auto_opaque(&mut self, ty_raw: &IrType) -> IrType {
        let subtree_types_except_rust_opaque = {
            let mut gatherer = DistinctTypeGatherer::new();
            ty_raw.visit_types(
                &mut |ty| {
                    gatherer.add(ty) ||
                        // skip subtrees inside RustOpaque
                        matches!(ty, IrType::RustOpaque(_))
                },
                self.inner,
            );
            gatherer.gather()
        };

        if (subtree_types_except_rust_opaque.iter()).any(|x| matches!(x, IrType::Unencodable(_))) {
            return self.parse_rust_auto_opaque(&ty_raw);
        }

        ty_raw.clone()
    }

    fn parse_rust_auto_opaque(&mut self, ty: &IrType) -> IrType {
        let (modifier, inner) = parse_ir_type_modifier(ty);
        let new_ir = IrTypeRustAutoOpaque {
            modifier,
            inner: IrTypeRustOpaque {
                namespace: self.context.initiated_namespace.clone(),
                inner: Box::new(inner),
            },
        };

        RustAutoOpaque((self.inner.rust_auto_opaque_parser_info).get_or_insert(ty, new_ir))
    }
}

pub(super) type RustAutoOpaqueParserInfo = SimpleParsedTypesParserInfo<IrTypeRustAutoOpaque>;

fn parse_ir_type_modifier(ty: &IrType) -> (IrTypeModifier, IrType) {
    if let IrType::Unencodable(IrTypeUnencodable { string, .. }) = ty {
        let ast: Type = syn::parse_str(string).unwrap();
        if let Type::Reference(r) = &ast {
            let modifier = if r.mutability.is_some() {
                IrTypeModifier::RefMut
            } else {
                IrTypeModifier::Ref
            };

            let ty_without_modifier = IrType::Unencodable(IrTypeUnencodable {
                string: r.elem.to_token_stream().to_string(),
                segments: vec![],
            });

            return (modifier, ty_without_modifier);
        }
    }

    (IrTypeModifier::Owned, ty.clone())
}
