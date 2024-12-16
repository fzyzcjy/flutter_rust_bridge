use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateLifetimeable};
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::lifetime_extractor::LifetimeExtractor;
use crate::codegen::parser::mir::parser::lifetime_replacer::replace_lifetimes_to_static;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::codegen::parser::mir::ParseMode;
use crate::utils::namespace::Namespace;
use syn::Type;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_maybe_lifetimeable(
        &mut self,
        original: MirTypeRustAutoOpaqueImplicit,
        namespace: Option<Namespace>,
    ) -> anyhow::Result<MirType> {
        if !self.context.enable_lifetime {
            self.maybe_log_not_enable_lifetime();
            return Ok(MirType::RustAutoOpaqueImplicit(original));
        }

        let ty_str = &original.raw.string.with_original_lifetime();
        let ty: Type = syn::parse_str(ty_str)?;

        let lifetimes = LifetimeExtractor::extract_skipping_static(&ty);
        if lifetimes.is_empty() {
            return Ok(MirType::RustAutoOpaqueImplicit(original));
        }

        let delegate_ty_str = format!(
            "Lifetimeable<{}>",
            replace_lifetimes_to_static(ty_str, &lifetimes)
        );

        let inner_dart_api_type = original.inner.sanitized_type();

        Ok(MirType::Delegate(MirTypeDelegate::Lifetimeable(
            MirTypeDelegateLifetimeable {
                api_type: original,
                delegate: self.parse_rust_auto_opaque_explicit_typed(
                    &syn::parse_str(&delegate_ty_str)?,
                    namespace,
                    None,
                    Some(inner_dart_api_type),
                )?,
            },
        )))
    }

    fn maybe_log_not_enable_lifetime(&mut self) {
        if !self.inner.has_logged_lifetimeable && self.context.parse_mode != ParseMode::Early {
            log::info!("To handle some types, `enable_lifetime: true` may need to be set. \
            Please visit https://fzyzcjy.github.io/flutter_rust_bridge/guides/lifetimes for more details");
        }
        self.inner.has_logged_lifetimeable = true;
    }
}
