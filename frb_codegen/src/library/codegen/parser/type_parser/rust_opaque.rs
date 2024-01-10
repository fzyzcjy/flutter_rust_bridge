use crate::codegen::ir::namespace::Namespace;
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
        let namespace = (self.inner.rust_opaque_parser_info)
            .get_or_insert(ty.safe_ident(), self.context.initiated_namespace.clone());
        RustOpaque(IrTypeRustOpaque::new(namespace, ty.clone(), false))
    }
}

pub(super) type RustOpaqueParserInfo = SimpleNamespaceMap;

#[derive(Clone, Debug, Default)]
pub(super) struct SimpleNamespaceMap(HashMap<String, Namespace>);

impl SimpleNamespaceMap {
    pub fn get_or_insert(&mut self, ty: IrType, insert_namespace: Namespace) -> Namespace {
        (self.0.entry(ty.safe_ident()).or_insert(insert_namespace)).clone()
    }
}
