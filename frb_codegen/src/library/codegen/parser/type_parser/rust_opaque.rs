use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::optional::IrTypeOptional;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{
    Boxed, DartOpaque, Delegate, Dynamic, EnumRef, GeneralList, Optional, OptionalList, Primitive,
    PrimitiveList, Record, RustOpaque, StructRef, Unencodable,
};
use crate::codegen::parser::type_parser::unencodable::ArgsRefs::Generic;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;
use quote::ToTokens;
use std::collections::HashMap;
use syn::TypePath;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_rust_opaque(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("RustOpaque", Some(Generic([ty]))) => self.parse_rust_opaque(ty),

            _ => return Ok(None),
        }))
    }

    fn parse_rust_opaque(&mut self, ty: &IrType) -> IrType {
        let ty_safe_ident: String = ty.safe_ident();

        // NOTE when meeting the *same* type (same safe_ident), reuse the existing parsed
        // result. Especially, when the same type is seen in two different files
        // (thus `namespace`s), this can ensure they both point to one namespace.
        let ans = (self.inner.rust_opaque_parser_info.parsed_types)
            .entry(ty_safe_ident)
            .or_insert_with(|| {
                IrTypeRustOpaque::new(self.context.initiated_namespace.clone(), ty.clone())
            });

        RustOpaque(ans.clone())
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct RustOpaqueParserInfo {
    parsed_types: HashMap<String, IrTypeRustOpaque>,
}
