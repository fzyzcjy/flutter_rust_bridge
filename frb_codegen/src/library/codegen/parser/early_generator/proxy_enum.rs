use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::ir::early_generator::proxied_type::IrEarlyGeneratorProxiedType;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateProxyEnum, MirTypeDelegateProxyVariant,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::early_generator::inject_extra_code_to_rust_output;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::function::real::FUNC_PREFIX_FRB_INTERNAL_NO_IMPL;
use crate::if_then_some;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
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

    let proxy_variants_of_enum =
        (proxy_variants.iter()).into_group_map_by(|ty| ty.upstream.safe_ident());

    let proxied_types = compute_proxied_types(&proxy_variants_of_enum, config_mir);

    let extra_code = proxy_variants_of_enum
        .into_values()
        .map(|proxy_variants| generate_proxy_enum(&proxy_variants))
        .join("");

    inject_extra_code_to_rust_output(&mut pack.hir_flat_pack, &extra_code, config_mir)?;
    (pack.proxied_types).extend(proxied_types);

    Ok(())
}

fn compute_proxied_types(
    proxy_variants_of_enum: &HashMap<String, Vec<&MirTypeDelegateProxyVariant>>,
    config_mir: &ParserMirInternalConfig,
) -> Vec<IrEarlyGeneratorProxiedType> {
    (proxy_variants_of_enum.values())
        .map(|variants| {
            let proxy_enum_namespace = config_mir
                .rust_input_namespace_pack
                .rust_output_path_namespace
                .clone();

            IrEarlyGeneratorProxiedType {
                proxy_enum_namespace,
                original_ty: (*variants[0].inner).to_owned(),
                variants: variants.iter().map(|&x| x.to_owned()).collect_vec(),
            }
        })
        .collect_vec()
}

fn generate_proxy_enum(proxy_variants: &[&MirTypeDelegateProxyVariant]) -> String {
    let proxy_enum_ty = *proxy_variants[0].inner.clone();

    let enum_name = MirTypeDelegateProxyEnum::proxy_enum_name_raw(&proxy_enum_ty);

    let variants = (proxy_variants.iter().enumerate())
        .map(|(index, variant)| {
            format!(
                "Variant{index}(RustAutoOpaque<{}>),\n",
                &variant.upstream.rust_api_type()
            )
        })
        .join("");

    let impl_lockable = generate_impl_lockable(enum_name);

    format!(
        "
        enum {enum_name} {{
            {variants}
        }}

        {impl_lockable}

        pub fn {FUNC_PREFIX_FRB_INTERNAL_NO_IMPL}_dummy_function_{enum_name}(a: {enum_name}) {{ }}
        "
    )
}

fn generate_impl_lockable(enum_name: &str) -> String {
    format!(
        "
        impl Lockable for {enum_name} {{
            type RwLockReadGuard<'a> = TODO;
            type RwLockWriteGuard<'a> = TODO;

            fn lockable_order(&self) -> LockableOrder {{
                TODO
            }}

            fn lockable_decode_sync_ref(&self) -> Self::RwLockReadGuard<'_> {{
                TODO
            }}

            fn lockable_decode_sync_ref_mut(&self) -> Self::RwLockWriteGuard<'_> {{
                TODO
            }}

            fn lockable_decode_async_ref<'a>(
                &'a self,
            ) -> Pin<Box<dyn Future<Output = Self::RwLockReadGuard<'_>> + Send + 'a>>
            where
                Self: Sync + 'a,
            {{
                Box::pin(async move {{ TODO }})
            }}

            fn lockable_decode_async_ref_mut<'a>(
                &'a self,
            ) -> Pin<Box<dyn Future<Output = Self::RwLockWriteGuard<'_>> + Send + 'a>>
            where
                Self: Sync + 'a,
            {{
                Box::pin(async move {{ TODO }})
            }}
        }
        "
    )
}
