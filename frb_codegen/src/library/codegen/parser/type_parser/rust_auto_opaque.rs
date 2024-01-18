use crate::codegen::ir::ty::ownership::IrTypeOwnershipMode;
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::rust_opaque::{IrTypeRustOpaque, RustOpaqueCodecMode};
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::rust_opaque::{
    GeneralizedRustOpaqueParserInfo, RustOpaqueParserTypeInfo,
};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::library::codegen::ir::ty::IrTypeTrait;
use IrType::RustAutoOpaque;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_rust_auto_opaque(&mut self, ty: &IrType) -> IrType {
        let (ownership_mode, inner) = match ty {
            IrType::Ownership(o) => (o.mode.clone(), *o.inner.clone()),
            _ => (IrTypeOwnershipMode::Owned, ty.clone()),
        };

        let info = self.get_or_insert_rust_auto_opaque_info(&inner, None);

        RustAutoOpaque(IrTypeRustAutoOpaque {
            ownership_mode,
            raw: Box::new(ty.to_owned()),
            inner: IrTypeRustOpaque {
                namespace: info.namespace,
                inner: Box::new(self.create_rust_opaque_type_for_rust_auto_opaque(&inner)),
                codec: info.codec,
                brief_name: true,
            },
        })
    }

    pub(super) fn create_rust_opaque_type_for_rust_auto_opaque(&self, inner: &IrType) -> IrType {
        IrType::Unencodable(IrTypeUnencodable {
            namespace: None,
            // TODO when all usages of a type do not require `&mut`, can drop this Mutex
            // TODO similarly, can use std instead of `tokio`'s lock
            string: format!(
                "flutter_rust_bridge::for_generated::rust_async::RwLock<{}>",
                inner.rust_api_type()
            ),
            segments: vec![],
        })
    }

    pub(super) fn get_or_insert_rust_auto_opaque_info(
        &mut self,
        inner: &IrType,
        codec: Option<RustOpaqueCodecMode>,
    ) -> RustOpaqueParserTypeInfo {
        self.inner.rust_auto_opaque_parser_info.get_or_insert(
            inner,
            RustOpaqueParserTypeInfo::new(
                self.context.initiated_namespace.clone(),
                codec
                    .or(self.context.func_attributes.rust_opaque_codec())
                    .unwrap_or(self.context.default_rust_opaque_codec),
            ),
        )
    }
}

pub(super) type RustAutoOpaqueParserInfo = GeneralizedRustOpaqueParserInfo;
