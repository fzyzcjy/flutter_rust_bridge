use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::hir::flat::extra_code_injector::inject_extra_code;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::real::FUNC_PREFIX_FRB_INTERNAL_NO_IMPL;
use crate::codegen::parser::mir::parser::tentative_parse_trait_impls;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

pub(crate) fn transform(
    mut pack: HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    let trait_impls = tentative_parse_trait_impls(&pack)?;

    let extra_code = (pack.traits.iter())
        .filter(|x| FrbAttributes::parse(&x.attrs).unwrap().generate_impl_enum())
        .map(|x| generate_trait_impl_enum(x, &trait_impls))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .join("");

    let namespace = &config.rust_input_namespace_pack.rust_output_path_namespace;

    inject_extra_code(&mut pack, &extra_code, namespace)?;

    Ok(pack)
}

fn generate_trait_impl_enum(
    hir_trait: &HirFlatTrait,
    trait_impls: &[MirTraitImpl],
) -> anyhow::Result<String> {
    let trait_def_name = &hir_trait.name.name;

    let interest_trait_impls = (trait_impls.iter())
        .filter(|x| x.trait_ty.name == hir_trait.name)
        .map(|x| x.impl_ty.clone())
        .collect_vec();

    let code_impl = generate_code_impl(trait_def_name, &interest_trait_impls);
    let code_read_guard = generate_code_read_guard();
    let code_write_guard = generate_code_write_guard();

    Ok(format!(
        "{code_impl}
        {code_read_guard}
        {code_write_guard}
        pub fn {FUNC_PREFIX_FRB_INTERNAL_NO_IMPL}_dummy_function_{trait_def_name}(a: {trait_def_name}Impl) {{ }}
        "
    ))
}

fn generate_code_impl(trait_def_name: &str, trait_impls: &[MirType]) -> String {
    let enum_name = format!("{trait_def_name}Impl");
    let enum_def = generate_enum_raw(&trait_impls, &enum_name, |ty| {
        format!("RustAutoOpaque<{ty}>")
    });

    let blocking_read_body = generate_match_raw(trait_impls, |ty| {
        format!("{trait_def_name}RwLockReadGuard::{ty}(inner.blocking_read())")
    });
    let blocking_write_body = generate_match_raw(trait_impls, |ty| {
        format!("{trait_def_name}RwLockWriteGuard::{ty}(inner.blocking_write())")
    });

    format!(
        "{enum_def}

        impl {enum_name} {{
            #[frb(ignore)]
            pub fn blocking_read(&self) -> {trait_def_name}RwLockReadGuard {{
                {blocking_read_body}
            }}

            #[frb(ignore)]
            pub fn blocking_write(&mut self) -> {trait_def_name}RwLockWriteGuard {{
                {blocking_write_body}
            }}
        }}"
    )
}

fn generate_code_read_guard(trait_def_name: &str, trait_impls: &[MirType]) -> String {
    let enum_name = format!("{trait_def_name}RwLockReadGuard");
    let enum_def = generate_enum_raw(&trait_impls, &enum_name, |ty| {
        format!("flutter_rust_bridge::for_generated::rust_async::RwLockReadGuard<'a, {ty}>")
    });

    let deref_body = generate_match_raw(trait_impls, |_| "inner.deref()".to_owned());

    format!(
        "#[frb(ignore)]
        {enum_def}


        impl std::ops::Deref for {enum_name}<'_> {{
            type Target = dyn SimpleTraitForDynTwinNormal;

            fn deref(&self) -> &Self::Target {{
                {deref_body}
            }}
        }}
        "
    )
}

fn generate_code_write_guard(trait_def_name: &str, trait_impls: &[MirType]) -> String {
    TODO;
}

fn generate_enum_raw(
    trait_impls: &[MirType],
    enum_name: &str,
    wrapper: impl Fn(&str) -> String,
) -> String {
    let variants = (trait_impls.iter())
        .map(|ty| {
            let rust_api_type = ty.rust_api_type();
            format!("{rust_api_type}({}),\n", wrapper(&rust_api_type))
        })
        .join("");

    format!(
        "pub enum {enum_name} {{
            {variants}
        }}"
    )
}

fn generate_match_raw(trait_impls: &[MirType], branch: impl Fn(&str) -> String) -> String {
    let variants = (trait_impls.iter())
        .map(|ty| {
            let rust_api_type = ty.rust_api_type();
            format!("Self::{rust_api_type}(inner) => {}", branch(&rust_api_type))
        })
        .join("");

    format!(
        "match self {{
            {variants}
        }}"
    )
}
