use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateLifetimeable};
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
    MirTypeRustAutoOpaqueImplicit, MirTypeRustAutoOpaqueImplicitReason,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::lifetime_extractor::LifetimeExtractor;
use crate::codegen::parser::mir::parser::lifetime_replacer::replace_lifetimes_to_static;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use syn::Type;
use crate::utils::namespace::Namespace;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_maybe_lifetimeable(
        &mut self,
        original: MirTypeRustAutoOpaqueImplicit,
        namespace: Option<Namespace>,
    ) -> anyhow::Result<MirType> {
        let ty_str = &original.raw.string.with_original_lifetime();
        let ty: Type = syn::parse_str(ty_str)?;
        let lifetimes = LifetimeExtractor::extract_skipping_static(&ty);

        if !lifetimes.is_empty() {
            let delegate_ty_str = format!(
                "flutter_rust_bridge::for_generated::Lifetimeable<{}>",
                replace_lifetimes_to_static(ty_str, &lifetimes)
            );

            let inner_dart_api_type = original.inner.sanitized_type();

            return Ok(MirType::Delegate(MirTypeDelegate::Lifetimeable(
                MirTypeDelegateLifetimeable {
                    api_type: Box::new(MirType::RustAutoOpaqueImplicit(original)),
                    delegate: self.parse_rust_auto_opaque_explicit_typed(
                        &syn::parse_str(&delegate_ty_str)?,
                        namespace,
                        None,
                        Some(inner_dart_api_type),
                    )?,
                },
            )));
        }

        Ok(MirType::RustAutoOpaqueImplicit(original))
    }
}