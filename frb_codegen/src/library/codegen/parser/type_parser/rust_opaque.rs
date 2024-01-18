use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::rust_opaque::{IrTypeRustOpaque, RustOpaqueCodecMode};
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
            ("RustOpaque", Some(Generic([ty]))) => self.parse_rust_opaque(ty, None),
            ("RustOpaqueNom", Some(Generic([ty]))) => {
                self.parse_rust_opaque(ty, Some(RustOpaqueCodecMode::Nom))
            }
            ("RustOpaqueMoi", Some(Generic([ty]))) => {
                self.parse_rust_opaque(ty, Some(RustOpaqueCodecMode::Moi))
            }
            ("RustAutoOpaque", Some(Generic([ty]))) => self.parse_rust_auto_opaque_explicit(ty),

            _ => return Ok(None),
        }))
    }

    fn parse_rust_opaque(&mut self, ty: &IrType, codec: Option<RustOpaqueCodecMode>) -> IrType {
        let info = self.inner.rust_opaque_parser_info.get_or_insert(
            ty,
            RustOpaqueParserTypeInfo::new(
                self.context.initiated_namespace.clone(),
                codec
                    .or(self.context.func_attributes.rust_opaque_codec())
                    .unwrap_or(self.context.default_rust_opaque_codec),
            ),
        );
        RustOpaque(IrTypeRustOpaque::new(
            info.namespace,
            ty.clone(),
            info.codec,
            false,
        ))
    }

    fn parse_rust_auto_opaque_explicit(&mut self, inner: &IrType) -> IrType {
        let info = self.get_or_insert_rust_auto_opaque_info(&inner);

        RustOpaque(IrTypeRustOpaque {
            namespace: info.namespace,
            inner: Box::new(self.create_rust_opaque_type_for_rust_auto_opaque(&inner)),
            codec: info.codec,
            brief_name: true,
        })
    }
}

pub(super) type RustOpaqueParserInfo = GeneralizedRustOpaqueParserInfo;

#[derive(Clone, Debug)]
pub(super) struct RustOpaqueParserTypeInfo {
    pub namespace: Namespace,
    pub codec: RustOpaqueCodecMode,
}

impl RustOpaqueParserTypeInfo {
    pub fn new(namespace: Namespace, codec: RustOpaqueCodecMode) -> Self {
        Self { namespace, codec }
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct GeneralizedRustOpaqueParserInfo(HashMap<String, RustOpaqueParserTypeInfo>);

impl GeneralizedRustOpaqueParserInfo {
    pub fn get_or_insert(
        &mut self,
        ty: &IrType,
        insert_value: RustOpaqueParserTypeInfo,
    ) -> RustOpaqueParserTypeInfo {
        (self.0.entry(ty.safe_ident()).or_insert(insert_value)).clone()
    }
}
