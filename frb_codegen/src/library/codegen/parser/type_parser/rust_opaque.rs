use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::rust_auto_opaque::{
    IrRustAutoOpaqueRaw, IrTypeRustAutoOpaque, IrTypeRustAutoOpaqueSub,
    IrTypeRustAutoOpaqueSubExplicit,
};
use crate::codegen::ir::ty::rust_opaque::{
    IrRustOpaqueInner, IrTypeRustOpaque, RustOpaqueCodecMode,
};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{RustAutoOpaque, RustOpaque};
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use quote::ToTokens;
use std::collections::HashMap;
use std::fmt::Debug;
use syn::Type;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_rust_opaque(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("RustOpaque", [ty]) => self.parse_rust_opaque(ty, None),
            ("RustOpaqueNom", [ty]) => self.parse_rust_opaque(ty, Some(RustOpaqueCodecMode::Nom)),
            ("RustOpaqueMoi", [ty]) => self.parse_rust_opaque(ty, Some(RustOpaqueCodecMode::Moi)),

            ("RustAutoOpaque", [ty]) => self.parse_rust_auto_opaque_explicit(ty, None),
            ("RustAutoOpaqueNom", [ty]) => {
                self.parse_rust_auto_opaque_explicit(ty, Some(RustOpaqueCodecMode::Nom))
            }
            ("RustAutoOpaqueMoi", [ty]) => {
                self.parse_rust_auto_opaque_explicit(ty, Some(RustOpaqueCodecMode::Moi))
            }

            _ => return Ok(None),
        }))
    }

    fn parse_rust_opaque(&mut self, ty: &Type, codec: Option<RustOpaqueCodecMode>) -> IrType {
        let ty_str = ty.to_token_stream().to_string();
        let info = self.inner.rust_opaque_parser_info.get_or_insert(
            ty_str.clone(),
            RustOpaqueParserTypeInfo::new(
                self.context.initiated_namespace.clone(),
                codec
                    .or(self.context.func_attributes.rust_opaque_codec())
                    .unwrap_or(self.context.default_rust_opaque_codec),
            ),
        );
        RustOpaque(IrTypeRustOpaque::new(
            info.namespace,
            IrRustOpaqueInner(ty_str),
            info.codec,
            false,
        ))
    }

    // TODO move
    fn parse_rust_auto_opaque_explicit(
        &mut self,
        inner: &Type,
        codec: Option<RustOpaqueCodecMode>,
    ) -> IrType {
        let inner_str = inner.to_token_stream().to_string();
        let info = self.get_or_insert_rust_auto_opaque_info(&inner_str, None, codec);

        RustAutoOpaque(IrTypeRustAutoOpaque {
            sub: IrTypeRustAutoOpaqueSub::Explicit(IrTypeRustAutoOpaqueSubExplicit {}),
            inner: IrTypeRustOpaque {
                namespace: info.namespace,
                inner: self.create_rust_opaque_type_for_rust_auto_opaque(&inner_str),
                codec: info.codec,
                brief_name: true,
            },
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
        type_safe_ident: String,
        insert_value: RustOpaqueParserTypeInfo,
    ) -> RustOpaqueParserTypeInfo {
        (self.0.entry(type_safe_ident).or_insert(insert_value)).clone()
    }
}
