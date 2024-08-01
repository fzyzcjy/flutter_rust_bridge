use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::if_then_some;
use anyhow::{bail, Context};
use syn::{PathSegment, TypeImplTrait, TypeParamBound};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_impl_trait(
        &mut self,
        type_impl_trait: &TypeImplTrait,
    ) -> anyhow::Result<MirType> {
        let trait_bound = (type_impl_trait.bounds.iter())
            .filter_map(
                |x| if_then_some!(let TypeParamBound::Trait(trait_bound) = x, trait_bound.clone()),
            )
            .next()
            .context("cannot find trait_bound")?;

        let last_segment = (trait_bound.path.segments.last()).context("cannot get segment")?;
        let segment_ident = last_segment.ident.to_string();

        if let Some(out) = self.parse_type_impl_trait_concrete(&segment_ident, last_segment)? {
            return Ok(out);
        }

        if let Some(out) = self.parse_type_impl_trait_core(&segment_ident, last_segment)? {
            return Ok(out);
        }

        bail!("Fail to parse impl trait {segment_ident}")
        // frb-coverage:ignore-end
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn parse_type_impl_trait_core(
        &mut self,
        name: &str,
        _segment: &PathSegment,
    ) -> anyhow::Result<Option<MirType>> {
        bail!("Fail to parse impl trait {name}")
    }
}
