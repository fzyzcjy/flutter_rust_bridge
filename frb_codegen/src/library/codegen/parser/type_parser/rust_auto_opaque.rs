use crate::codegen::ir::pack::DistinctTypeGatherer;
use crate::codegen::ir::ty::ownership::IrTypeOwnershipMode;
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::rust_opaque::{
    GeneralizedRustOpaqueParserInfo, RustOpaqueParserTypeInfo,
};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::library::codegen::ir::ty::IrTypeTrait;
use log::debug;
use IrType::RustAutoOpaque;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn transform_type_rust_auto_opaque(&mut self, ty_raw: &IrType) -> IrType {
        if self.check_candidate_rust_auto_opaque(ty_raw) {
            let ty_ans = self.parse_rust_auto_opaque(ty_raw);
            debug!("transform_type_rust_auto_opaque convert {ty_raw:?} -> {ty_ans:?}");
            return ty_ans;
        }

        ty_raw.clone()
    }

    pub(crate) fn check_candidate_rust_auto_opaque(&self, ty_raw: &IrType) -> bool {
        let subtree_types_except_rust_opaque = {
            let mut gatherer = DistinctTypeGatherer::new();
            ty_raw.visit_types(
                &mut |ty| {
                    gatherer.add(ty) ||
                        // skip subtrees inside RustOpaque
                        matches!(ty, IrType::RustOpaque(_))
                },
                self.inner,
            );
            gatherer.gather()
        };

        (subtree_types_except_rust_opaque.iter()).any(|x| matches!(x, IrType::Unencodable(_)))
    }

    fn parse_rust_auto_opaque(&mut self, ty: &IrType) -> IrType {
        let (ownership_mode, inner) = match ty {
            IrType::Ownership(o) => (o.mode.clone(), *o.inner.clone()),
            _ => (IrTypeOwnershipMode::Owned, ty.clone()),
        };

        let info = self.get_or_insert_rust_auto_opaque_info(&inner);

        RustAutoOpaque(IrTypeRustAutoOpaque {
            ownership_mode,
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
    ) -> RustOpaqueParserTypeInfo {
        self.inner.rust_auto_opaque_parser_info.get_or_insert(
            inner,
            RustOpaqueParserTypeInfo::new(
                self.context.initiated_namespace.clone(),
                (self.context.func_attributes.rust_opaque_codec())
                    .unwrap_or(self.context.default_rust_opaque_codec),
            ),
        )
    }
}

pub(super) type RustAutoOpaqueParserInfo = GeneralizedRustOpaqueParserInfo;
