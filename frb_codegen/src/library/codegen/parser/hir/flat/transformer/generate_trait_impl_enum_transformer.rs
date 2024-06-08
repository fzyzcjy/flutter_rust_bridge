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

    let enum_impl = generate_simple_enum(
        &interest_trait_impls,
        &format!("{trait_def_name}Impl"),
        |ty| format!("RustAutoOpaque<{ty}>"),
    );

    Ok(format!(
        "{enum_impl}

        pub fn {FUNC_PREFIX_FRB_INTERNAL_NO_IMPL}_dummy_function_{trait_def_name}(a: {trait_def_name}Impl) {{ }}
        "
    ))
}

fn generate_simple_enum(
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
