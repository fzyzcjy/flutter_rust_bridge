use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::rust_auto_opaque::{IrTypeRustAutoOpaque, OwnershipMode};
use crate::codegen::ir::ty::rust_opaque::{
    IrRustOpaqueInner, IrTypeRustOpaque, RustOpaqueCodecMode,
};
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::rust_opaque::{
    GeneralizedRustOpaqueParserInfo, RustOpaqueParserTypeInfo,
};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::library::codegen::ir::ty::IrTypeTrait;
use quote::ToTokens;
use syn::Type;
use IrType::RustAutoOpaque;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_rust_auto_opaque(
        &mut self,
        namespace: Option<Namespace>,
        ty: &Type,
    ) -> IrType {
        let (ownership_mode, inner) = match ty {
            Type::Reference(ty) => (
                if ty.mutability.is_some() {
                    OwnershipMode::RefMut
                } else {
                    OwnershipMode::Ref
                },
                (*ty.elem).to_owned(),
            ),
            _ => (OwnershipMode::Owned, ty.clone()),
        };

        let inner_str = inner.to_token_stream().to_string();

        let info = self.get_or_insert_rust_auto_opaque_info(&inner_str, namespace, None);

        RustAutoOpaque(IrTypeRustAutoOpaque {
            ownership_mode,
            raw: inner_str,
            inner: IrTypeRustOpaque {
                namespace: info.namespace,
                inner: self.create_rust_opaque_type_for_rust_auto_opaque(&inner_str),
                codec: info.codec,
                brief_name: true,
            },
        })
    }

    pub(super) fn create_rust_opaque_type_for_rust_auto_opaque(
        &self,
        inner: &str,
    ) -> IrRustOpaqueInner {
        // TODO when all usages of a type do not require `&mut`, can drop this Mutex
        // TODO similarly, can use std instead of `tokio`'s lock
        IrRustOpaqueInner(format!(
            "flutter_rust_bridge::for_generated::rust_async::RwLock<{inner}>"
        ))
    }

    pub(super) fn get_or_insert_rust_auto_opaque_info(
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
}

pub(super) type RustAutoOpaqueParserInfo = GeneralizedRustOpaqueParserInfo;
