use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::lifetime_extractor::LifetimeExtractor;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::if_then_some;
use itertools::Itertools;
use syn::fold::fold_lifetime;
use syn::{GenericArgument, Type};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_maybe_lifetimeable(
        &mut self,
        original: MirTypeRustAutoOpaqueImplicit,
    ) -> anyhow::Result<MirType> {
        Ok(self
            .parse_maybe_lifetimeable_raw(&original.raw.string)?
            .unwrap_or(MirType::RustAutoOpaqueImplicit(original)))
    }

    fn parse_maybe_lifetimeable_raw(&self, original: &str) -> anyhow::Result<Option<MirType>> {
        let ty: Type = syn::parse_str(original)?;
        let lifetimes = LifetimeExtractor::extract(&ty);
        Ok(TODO)
    }
}
