use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateLifetimeable};
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{MirTypeRustAutoOpaqueImplicit, MirTypeRustAutoOpaqueImplicitReason};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::lifetime_extractor::LifetimeExtractor;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use syn::Type;
use crate::utils::namespace::Namespace;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_maybe_lifetimeable(
        &mut self,
        original: MirTypeRustAutoOpaqueImplicit,
        namespace: Option<Namespace>,
    ) -> anyhow::Result<MirType> {
        let ty: Type = syn::parse_str(&original.raw.string)?;
        let lifetimes = LifetimeExtractor::extract_skipping_static(&ty);

        if !lifetimes.is_empty() {
            return Ok(MirType::Delegate(MirTypeDelegate::Lifetimeable(
                MirTypeDelegateLifetimeable {
                    api_type: Box::new(MirType::RustAutoOpaqueImplicit(original)),
                    delegate: Box::new(self.parse_type_rust_auto_opaque_implicit(TODO)?),
                },
            )));
        }

        Ok(MirType::RustAutoOpaqueImplicit(original))
    }
}
