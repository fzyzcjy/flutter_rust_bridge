use crate::codegen::mir::func::OwnershipMode;
use crate::codegen::mir::namespace::Namespace;
use crate::codegen::mir::ty::rust_auto_opaque_implicit::{
    IrRustAutoOpaqueRaw, IrTypeRustAutoOpaqueImplicit,
};
use crate::codegen::mir::ty::rust_opaque::{
    IrRustOpaqueInner, IrTypeRustOpaque, RustOpaqueCodecMode,
};
use crate::codegen::mir::ty::{IrType, IrTypeTrait};
use crate::codegen::parser::type_parser::external_impl;
use crate::codegen::parser::type_parser::path_data::extract_path_data;
use crate::codegen::parser::type_parser::rust_opaque::{
    GeneralizedRustOpaqueParserInfo, RustOpaqueParserTypeInfo,
};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::Result;
use quote::ToTokens;
use syn::Type;
use IrType::RustAutoOpaqueImplicit;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_rust_auto_opaque_implicit(
        &mut self,
        namespace: Option<Namespace>,
        ty: &Type,
    ) -> Result<IrType> {
        let (inner, ownership_mode) = split_ownership_from_ty(ty);
        let (ans_raw, ans_inner) =
            self.parse_type_rust_auto_opaque_common(inner, namespace, None)?;
        Ok(RustAutoOpaqueImplicit(IrTypeRustAutoOpaqueImplicit {
            ownership_mode,
            raw: ans_raw,
            inner: ans_inner,
        }))
    }

    pub(crate) fn parse_type_rust_auto_opaque_common(
        &mut self,
        inner: Type,
        namespace: Option<Namespace>,
        codec: Option<RustOpaqueCodecMode>,
    ) -> Result<(IrRustAutoOpaqueRaw, IrTypeRustOpaque)> {
        let inner = external_impl::parse_type(inner)?;

        let inner_str = inner.to_token_stream().to_string();
        let info = self.get_or_insert_rust_auto_opaque_info(&inner_str, namespace, codec);

        let raw_segments = match inner {
            Type::Path(inner) => extract_path_data(&inner.path)?,
            _ => vec![],
        };

        Ok((
            IrRustAutoOpaqueRaw {
                string: inner_str.clone(),
                segments: raw_segments,
            },
            IrTypeRustOpaque {
                namespace: info.namespace,
                // TODO when all usages of a type do not require `&mut`, can drop this Mutex
                // TODO similarly, can use std instead of `tokio`'s lock
                inner: IrRustOpaqueInner(format!(
                    "flutter_rust_bridge::for_generated::RustAutoOpaqueInner<{inner_str}>"
                )),
                codec: info.codec,
                brief_name: true,
            },
        ))
    }

    fn get_or_insert_rust_auto_opaque_info(
        &mut self,
        inner: &str,
        namespace: Option<Namespace>,
        codec: Option<RustOpaqueCodecMode>,
    ) -> RustOpaqueParserTypeInfo {
        self.inner.rust_auto_opaque_parser_info.get_or_insert(
            inner.to_owned(),
            RustOpaqueParserTypeInfo::new(
                namespace.unwrap_or(self.context.initiated_namespace.clone()),
                codec
                    .or(self.context.func_attributes.rust_opaque_codec())
                    .unwrap_or(self.context.default_rust_opaque_codec),
            ),
        )
    }

    pub(crate) fn transform_rust_auto_opaque(
        &mut self,
        ty_raw: &IrTypeRustAutoOpaqueImplicit,
        transform: impl FnOnce(&str) -> String,
    ) -> Result<IrType> {
        self.parse_type_rust_auto_opaque_implicit(
            ty_raw.self_namespace(),
            &syn::parse_str(&transform(&ty_raw.raw.string))?,
        )
    }
}

pub(super) type RustAutoOpaqueParserInfo = GeneralizedRustOpaqueParserInfo;

pub(crate) fn split_ownership_from_ty(ty: &Type) -> (Type, OwnershipMode) {
    match ty {
        Type::Reference(ty) => (
            (*ty.elem).to_owned(),
            if ty.mutability.is_some() {
                OwnershipMode::RefMut
            } else {
                OwnershipMode::Ref
            },
        ),
        _ => (ty.clone(), OwnershipMode::Owned),
    }
}
