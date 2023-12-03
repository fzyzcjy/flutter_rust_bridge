use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::RustOpaque;
use crate::codegen::parser::type_parser::unencodable::ArgsRefs::Generic;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::library::codegen::ir::ty::IrTypeTrait;
use std::collections::HashMap;
use std::fmt::Debug;

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
        let new_ir = IrTypeRustOpaque::new(self.context.initiated_namespace.clone(), ty.clone());
        let ans = (self.inner.rust_opaque_parser_info.parsed_types)
            .entry(ty_safe_ident)
            .or_insert(new_ir);

        RustOpaque(ans.clone())
    }
}

pub(super) type RustOpaqueParserInfo = SimpleParsedTypesParserInfo<IrTypeRustOpaque>;

#[derive(Clone, Debug, Default)]
pub(super) struct SimpleParsedTypesParserInfo<T: Clone + Debug> {
    parsed_types: HashMap<String, T>,
}

impl<T: Clone + Debug> SimpleParsedTypesParserInfo<T> {
    pub fn new() -> Self {
        Self {
            parsed_types: HashMap::new(),
        }
    }
}
