use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::hir::flat::extra_code_injector::{
    inject_extra_codes, InjectExtraCodeBlock,
};
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::real::FUNC_PREFIX_FRB_INTERNAL_NO_IMPL;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;
use strum_macros::Display;

pub(crate) fn generate(
    pack: &mut IrEarlyGeneratorPack,
    tentative_mir_pack: &MirPack,
    config_mir: &ParserMirInternalConfig,
) -> anyhow::Result<()> {
    let extra_codes = (pack.hir_flat_pack.traits.iter())
        .filter(|x| {
            FrbAttributes::parse(&x.attrs)
                .unwrap()
                .generate_implementor_enum()
        })
        .sorted_by_key(|x| x.name.clone())
        .flat_map(|x| generate_trait_impl_enum(x, &tentative_mir_pack.trait_impls))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let output_namespace = &(config_mir.rust_input_namespace_pack).rust_output_path_namespace;

    inject_extra_codes(&mut pack.hir_flat_pack, output_namespace, &extra_codes)?;

    Ok(())
}

fn generate_trait_impl_enum(
    hir_trait: &HirFlatTrait,
    all_trait_impls: &[MirTraitImpl],
) -> anyhow::Result<Vec<InjectExtraCodeBlock>> {
    let trait_def_name = &hir_trait.name.name;

    let interest_trait_impls = (all_trait_impls.iter())
        .filter(|x| x.trait_ty.name == hir_trait.name)
        .map(|x| x.impl_ty.clone())
        .sorted_by_key(|x| x.safe_ident())
        .collect_vec();

    let code_impl = generate_code_impl(trait_def_name, &interest_trait_impls);
    let code_read_guard =
        generate_code_read_write_guard(ReadWrite::Read, trait_def_name, &interest_trait_impls);
    let code_write_guard =
        generate_code_read_write_guard(ReadWrite::Write, trait_def_name, &interest_trait_impls);

    let code = format!(
        "{code_impl}

        {code_read_guard}

        {code_write_guard}

        pub fn {FUNC_PREFIX_FRB_INTERNAL_NO_IMPL}_dummy_function_{trait_def_name}(a: {trait_def_name}Implementor) {{ }}
        "
    );

    Ok(vec![InjectExtraCodeBlock {
        code,
        should_parse: true,
    }])
}

fn generate_code_impl(trait_def_name: &str, trait_impls: &[MirType]) -> String {
    let enum_name = format!("{trait_def_name}Implementor");
    let enum_def = generate_enum_raw(trait_impls, &enum_name, |ty| {
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
            #[flutter_rust_bridge::frb(ignore)]
            pub fn blocking_read(&self) -> {trait_def_name}RwLockReadGuard {{
                {blocking_read_body}
            }}

            #[flutter_rust_bridge::frb(ignore)]
            pub fn blocking_write(&mut self) -> {trait_def_name}RwLockWriteGuard {{
                {blocking_write_body}
            }}
        }}"
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
enum ReadWrite {
    Read,
    Write,
}

fn generate_code_read_write_guard(
    rw: ReadWrite,
    trait_def_name: &str,
    trait_impls: &[MirType],
) -> String {
    let rw_pascal = rw.to_string().to_case(Case::Pascal);

    let enum_name = format!("{trait_def_name}RwLock{rw_pascal}Guard");
    let enum_def = generate_enum_raw(trait_impls, &format!("{enum_name}<'a>"), |ty| {
        format!("flutter_rust_bridge::for_generated::rust_async::RwLock{rw_pascal}Guard<'a, {ty}>")
    });

    let deref_body = generate_match_raw(trait_impls, |_| "inner.deref()".to_owned());
    let deref_code = format!(
        "
        impl std::ops::Deref for {enum_name}<'_> {{
            type Target = dyn {trait_def_name};

            fn deref(&self) -> &Self::Target {{
                {deref_body}
            }}
        }}
        "
    );

    let maybe_deref_mut_code = if rw == ReadWrite::Write {
        let body = generate_match_raw(trait_impls, |_| "inner.deref_mut()".to_owned());
        format!(
            "
            impl std::ops::DerefMut for {enum_name}<'_> {{
                fn deref_mut(&mut self) -> &mut Self::Target {{
                    {body}
                }}
            }}
            "
        )
    } else {
        "".to_owned()
    };

    format!(
        "#[flutter_rust_bridge::frb(ignore)]
        {enum_def}

        {deref_code}

        {maybe_deref_mut_code}
        "
    )
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
            format!(
                "Self::{rust_api_type}(inner) => {},\n",
                branch(&rust_api_type)
            )
        })
        .join("");

    format!(
        "match self {{
            {variants}
        }}"
    )
}
