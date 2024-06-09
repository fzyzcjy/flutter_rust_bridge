use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::ir::early_generator::proxied_type::IrEarlyGeneratorProxiedType;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateProxyEnum, MirTypeDelegateProxyVariant,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::early_generator::utils::lockable;
use crate::codegen::parser::hir::flat::extra_code_injector::{
    inject_extra_codes, InjectExtraCodeBlock,
};
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::function::real::FUNC_PREFIX_FRB_INTERNAL_NO_IMPL;
use crate::if_then_some;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use std::collections::HashMap;

pub(crate) fn generate(
    pack: &mut IrEarlyGeneratorPack,
    tentative_mir_pack: &MirPack,
    config_mir: &ParserMirInternalConfig,
) -> anyhow::Result<()> {
    let distinct_types = tentative_mir_pack.distinct_types(None);
    let proxy_variants = (distinct_types.iter())
        .filter_map(|ty| if_then_some!(let MirType::Delegate(MirTypeDelegate::ProxyVariant(inner)) = ty, inner.clone()))
        .collect_vec();

    let output_namespace = &(config_mir.rust_input_namespace_pack).rust_output_path_namespace;

    let proxy_variants_of_enum =
        (proxy_variants.iter()).into_group_map_by(|ty| ty.upstream.safe_ident());

    let proxied_types = compute_proxied_types(&proxy_variants_of_enum, &output_namespace);

    let extra_codes = (proxy_variants_of_enum.values())
        .map(|proxy_variants| generate_proxy_enum(proxy_variants))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    inject_extra_codes(&mut pack.hir_flat_pack, output_namespace, &extra_codes)?;
    (pack.proxied_types).extend(proxied_types);

    Ok(())
}

fn compute_proxied_types(
    proxy_variants_of_enum: &HashMap<String, Vec<&MirTypeDelegateProxyVariant>>,
    proxy_enum_namespace: &Namespace,
) -> Vec<IrEarlyGeneratorProxiedType> {
    (proxy_variants_of_enum.values())
        .map(|variants| IrEarlyGeneratorProxiedType {
            proxy_enum_namespace: proxy_enum_namespace.clone(),
            original_ty: (*variants[0].inner).to_owned(),
            variants: variants.iter().map(|&x| x.to_owned()).collect_vec(),
        })
        .collect_vec()
}

fn generate_proxy_enum(
    proxy_variants: &[&MirTypeDelegateProxyVariant],
) -> anyhow::Result<Vec<InjectExtraCodeBlock>> {
    let proxy_enum_ty = *proxy_variants[0].inner.clone();

    let enum_name = MirTypeDelegateProxyEnum::proxy_enum_name_raw(&proxy_enum_ty);

    let variants = (proxy_variants.iter().enumerate())
        .map(|(index, variant)| lockable::VariantInfo {
            enum_variant_name: format!("Variant{index}"),
            ty_name: variant.upstream.rust_api_type(),
        })
        .collect_vec();

    let deref_target = compute_deref_target(&proxy_enum_ty);

    lockable::generate(&enum_name, &deref_target, &variants)
}

fn compute_deref_target(ty: &MirType) -> String {
    match ty {
        MirType::RustAutoOpaqueImplicit(ty) => ty.raw.string.clone(),
        _ => unimplemented!(),
    }
}
