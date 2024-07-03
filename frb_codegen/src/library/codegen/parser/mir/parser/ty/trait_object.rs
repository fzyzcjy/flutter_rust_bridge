use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateDynTrait, MirTypeDelegateDynTraitData,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::trait_def::parse_type_trait;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::codegen::parser::mir::ParseMode;
use crate::if_then_some;
use crate::utils::syn_utils::ty_to_string;
use anyhow::Context;
use syn::TypeTraitObject;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_trait_object(
        &mut self,
        type_trait_object: &TypeTraitObject,
    ) -> anyhow::Result<MirType> {
        if let Some(out) = self.parse_type_trait_object_inner(type_trait_object)? {
            return Ok(out);
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
    fn parse_type_trait_object_inner(
        &mut self,
        type_trait_object: &TypeTraitObject,
    ) -> anyhow::Result<Option<MirType>> {
        // frb-coverage:ignore-end
        if let Some(trait_name_path) = extract_trait_name_path(type_trait_object) {
            let trait_name = ty_to_string(&trait_name_path.segments.last().unwrap());
            if let Some(trait_ty) = parse_type_trait(&trait_name, self.inner) {
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
        }
        Ok(None)
    }
}

fn extract_trait_name_path(type_trait_object: &TypeTraitObject) -> Option<syn::Path> {
    let bounds = &type_trait_object.bounds;
    if bounds.len() != 1 {
        return None;
    }

    if_then_some!(
        let syn::TypeParamBound::Trait(trait_bound) = bounds.first().unwrap(),
        trait_bound.path.clone()
    )
}
