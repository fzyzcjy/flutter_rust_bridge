use crate::codegen::ir::pack::DistinctTypeGatherer;
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::RustOpaque;
use crate::codegen::parser::type_parser::unencodable::ArgsRefs::Generic;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::library::codegen::ir::ty::IrTypeTrait;
use std::collections::HashMap;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn transform_type_rust_auto_opaque(
        &mut self,
        ty_raw: IrType,
    ) -> anyhow::Result<IrType> {
        let subtree_types_except_rust_opaque = {
            let mut gatherer = DistinctTypeGatherer::new();
            ty_raw.visit_types(
                &mut |ty| {
                    gatherer.add(ty);

                    // skip subtrees inside RustOpaque
                    matches!(ty, IrType::RustOpaque(_))
                },
                self,
            );
            gatherer.gather()
        };

        if (subtree_types_except_rust_opaque.iter()).any(|x| matches!(x, IrType::Unencodable(_))) {
            return Ok(self.parse_rust_auto_opaque(ty_raw));
        }

        Ok(ty_raw)
    }

    fn parse_rust_auto_opaque(&self, ty_raw: IrType) -> IrType {
        IrType::RustAutoOpaque(IrTypeRustAutoOpaque::new(TODO, ty_raw))
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct RustAutoOpaqueParserInfo {
    parsed_types: HashMap<String, IrTypeRustAutoOpaque>,
}
