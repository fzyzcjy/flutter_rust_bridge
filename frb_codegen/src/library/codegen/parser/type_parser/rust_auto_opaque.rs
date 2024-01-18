use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::rust_auto_opaque::{IrTypeRustAutoOpaque, OwnershipMode};
use crate::codegen::ir::ty::rust_opaque::{IrTypeRustOpaque, RustOpaqueCodecMode};
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::rust_opaque::{
    GeneralizedRustOpaqueParserInfo, RustOpaqueParserTypeInfo,
};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::library::codegen::ir::ty::IrTypeTrait;
use syn::Type;
use IrType::RustAutoOpaque;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_rust_auto_opaque(
        &mut self,
        namespace: Option<Namespace>,
        ty: &Type,
    ) -> IrType {
        let (ownership_mode, inner) = match ty {
            Type::Reference(_) => (TODO, TODO),
            _ => (OwnershipMode::Owned, TODO),
        };

        let info = self.get_or_insert_rust_auto_opaque_info(&inner, namespace, None);

        RustAutoOpaque(IrTypeRustAutoOpaque {
            ownership_mode,
            raw: TODO,
            inner: IrTypeRustOpaque {
                namespace: info.namespace,
                inner: self.create_rust_opaque_type_for_rust_auto_opaque(&inner),
                codec: info.codec,
                brief_name: true,
            },
        })
    }

    pub(super) fn create_rust_opaque_type_for_rust_auto_opaque(&self, inner: &str) -> String {
        // TODO when all usages of a type do not require `&mut`, can drop this Mutex
        // TODO similarly, can use std instead of `tokio`'s lock
        format!("flutter_rust_bridge::for_generated::rust_async::RwLock<{inner}>")
    }

    pub(super) fn get_or_insert_rust_auto_opaque_info(
        &mut self,
        inner: &IrType,
        namespace: Option<Namespace>,
        codec: Option<RustOpaqueCodecMode>,
    ) -> RustOpaqueParserTypeInfo {
        self.inner.rust_auto_opaque_parser_info.get_or_insert(
            inner,
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
