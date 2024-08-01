use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateDynTrait, MirTypeDelegateDynTraitData,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::trait_def::parse_type_trait;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::codegen::parser::mir::ParseMode;
use crate::if_then_some;
use anyhow::Context;
use syn::TypeTraitObject;

use super::path_data::extract_path_data;
use super::unencodable::{splay_segments, SplayedSegment};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_trait_object(
        &mut self,
        type_trait_object: &TypeTraitObject,
    ) -> anyhow::Result<MirType> {
        // frb-coverage:ignore-end
        if let Some(trait_name_path) = extract_trait_name_path(type_trait_object) {
            let segments = extract_path_data(&trait_name_path)?;
            let splayed_segments = splay_segments(&segments);

            if let Some(last_segment) = splayed_segments.last() {
                if let Some(out) =
                    self.parse_type_trait_object_concrete(&last_segment, &splayed_segments)?
                {
                    return Ok(out);
                }

                if let Some(out) =
                    self.parse_type_trait_object_core(&last_segment, &splayed_segments)?
                {
                    return Ok(out);
                }
            }
        }

        // fallback
        self.parse_type_rust_auto_opaque_implicit(
            None,
            &syn::Type::TraitObject(type_trait_object.clone()),
            None,
            None,
        )
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn parse_type_trait_object_core(
        &mut self,
        last_segment: &SplayedSegment,
        _splayed_segments: &[SplayedSegment],
    ) -> anyhow::Result<Option<MirType>> {
        // frb-coverage:ignore-end
        let SplayedSegment { name, .. } = last_segment;
        if let Some(trait_ty) = parse_type_trait(name, self.inner) {
            let data = match self.context.parse_mode {
                ParseMode::Early => None,
                ParseMode::Normal => {
                    let trait_def_info = (self.inner.trait_def_infos.iter())
                        .find(|info| info.trait_def_name == trait_ty.name)
                        .with_context(|| {
                            format!("Cannot find trait def info for {:?}", trait_ty.name)
                        })?;
                    Some(MirTypeDelegateDynTraitData {
                        delegate_namespace: trait_def_info.delegate_namespace.clone(),
                        variants: trait_def_info.variants.clone(),
                    })
                }
            };

            return Ok(Some(MirType::Delegate(MirTypeDelegate::DynTrait(
                MirTypeDelegateDynTrait {
                    trait_def_name: trait_ty.name.clone(),
                    data,
                },
            ))));
        }
        Ok(None)
    }
}

fn extract_trait_name_path(type_trait_object: &TypeTraitObject) -> Option<syn::Path> {
    let bounds = &type_trait_object.bounds;
    if bounds.len() < 1 {
        return None;
    }

    if_then_some!(
        let syn::TypeParamBound::Trait(trait_bound) = bounds.first().unwrap(),
        trait_bound.path.clone()
    )
}
