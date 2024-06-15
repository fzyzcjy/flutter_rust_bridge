use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateLifetimeable};
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
    MirTypeRustAutoOpaqueImplicit, MirTypeRustAutoOpaqueImplicitReason,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::lifetime_extractor::LifetimeExtractor;
use crate::codegen::parser::mir::parser::lifetime_replacer::replace_lifetime_to_static;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::utils::namespace::Namespace;
use syn::Type;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_maybe_lifetimeable(
        &mut self,
        original: MirTypeRustAutoOpaqueImplicit,
        namespace: Option<Namespace>,
    ) -> anyhow::Result<MirType> {
        let ty_str = &original.raw.string;
        let ty: Type = syn::parse_str(ty_str)?;
        let lifetimes = LifetimeExtractor::extract_skipping_static(&ty);

        if !lifetimes.is_empty() {
            let delegate_ty_str = replace_lifetime_to_static(ty_str, &lifetimes);
            return Ok(MirType::Delegate(MirTypeDelegate::Lifetimeable(
                MirTypeDelegateLifetimeable {
                    api_type: Box::new(MirType::RustAutoOpaqueImplicit(original)),
                    delegate: Box::new(self.parse_type_rust_auto_opaque_implicit(
                        namespace,
                        syn::parse_str(&delegate_ty_str)?,
                        None,
                        None,
                    )?),
                },
            )));
        }

        Ok(MirType::RustAutoOpaqueImplicit(original))
    }
}