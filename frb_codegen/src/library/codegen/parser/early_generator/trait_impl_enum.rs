use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::ir::early_generator::trait_def_info::IrEarlyGeneratorTraitDefInfo;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateDynTraitVariant};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::early_generator::utils::lockable;
use crate::codegen::parser::hir::flat::extra_code_injector::{
    inject_extra_codes, InjectExtraCodeBlock,
};
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::if_then_some;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

pub(crate) fn generate(
    pack: &mut IrEarlyGeneratorPack,
    tentative_mir_pack: &MirPack,
    config_mir: &ParserMirInternalConfig,
) -> anyhow::Result<()> {
    let distinct_types = tentative_mir_pack.distinct_types(None);
    let output_namespace = &(config_mir.rust_input_namespace_pack).rust_output_path_namespace;

    let dyn_trait_types = (distinct_types.iter())
        .filter_map(|ty| if_then_some!(let MirType::Delegate(MirTypeDelegate::DynTrait(inner)) = ty, inner.clone()));
    let interest_trait_names = dyn_trait_types
        .map(|ty| ty.trait_def_name.clone())
        .unique()
        .collect_vec();

    let generated_items = (pack.hir_flat_pack.traits.iter())
        .filter(|x| interest_trait_names.contains(&x.name))
        .sorted_by_key(|x| x.name.clone())
        .map(|x| generate_trait_impl_enum(x, &tentative_mir_pack.trait_impls, output_namespace))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let extra_codes = (generated_items.iter())
        .flat_map(|x| x.0.clone())
        .collect_vec();

    let trait_def_infos = (generated_items.iter()).map(|x| x.1.clone()).collect_vec();

    inject_extra_codes(&mut pack.hir_flat_pack, output_namespace, &extra_codes)?;
    (pack.trait_def_infos).extend(trait_def_infos);

    Ok(())
}

fn generate_trait_impl_enum(
    hir_trait: &HirFlatTrait,
    all_trait_impls: &[MirTraitImpl],
    output_namespace: &Namespace,
) -> anyhow::Result<(Vec<InjectExtraCodeBlock>, IrEarlyGeneratorTraitDefInfo)> {
    let trait_def_namespaced_name = &hir_trait.name;
    let trait_def_name = &trait_def_namespaced_name.name;
    let enum_name = format!("{trait_def_name}Implementor");

    let interest_trait_impls = (all_trait_impls.iter())
        .filter(|x| &x.trait_ty.name == trait_def_namespaced_name)
        .map(|x| x.impl_ty.clone())
        .sorted_by_key(|x| x.safe_ident())
        .collect_vec();

    let variants = (interest_trait_impls.iter().enumerate())
        .map(|(index, ty)| lockable::VariantInfo {
            // enum_variant_name: ty.rust_api_type(),
            enum_variant_name: format!("Variant{index}"),
            ty_name: ty.rust_api_type(),
            deref_extra_code: "".to_owned(),
        })
        .collect_vec();

    let extra_codes = lockable::generate(
        &enum_name,
        &format!("dyn {trait_def_name}"),
        true,
        &variants,
    )?;

    let info = IrEarlyGeneratorTraitDefInfo {
        trait_def_name: trait_def_namespaced_name.clone(),
        delegate_namespace: output_namespace.clone(),
        variants: (interest_trait_impls.into_iter())
            .map(|ty| MirTypeDelegateDynTraitVariant { ty })
            .collect_vec(),
    };

    Ok((extra_codes, info))
}
