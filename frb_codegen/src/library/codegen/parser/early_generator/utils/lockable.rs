use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::hir::flat::extra_code_injector::InjectExtraCodeBlock;
use crate::codegen::parser::mir::parser::function::real::FUNC_PREFIX_FRB_INTERNAL_NO_IMPL;
use convert_case::{Case, Casing};
use itertools::Itertools;
use std::env;
use std::env::var;
use strum_macros::Display;

pub(crate) struct VariantInfo {
    pub enum_variant_name: String,
    pub ty_name: String,
}

pub(crate) fn generate(
    enum_name: &str,
    variants: &[VariantInfo],
) -> anyhow::Result<Vec<InjectExtraCodeBlock>> {
    let code_impl = generate_code_impl(enum_name, variants);
    let code_read_guard = generate_code_read_write_guard(enum_name, ReadWrite::Read, variants);
    let code_write_guard = generate_code_read_write_guard(enum_name, ReadWrite::Write, variants);

    let code = format!(
        "{code_impl}

        {code_read_guard}

        {code_write_guard}

        pub fn {FUNC_PREFIX_FRB_INTERNAL_NO_IMPL}_dummy_function_{enum_name}(a: {enum_name}) {{ }}
        "
    );

    Ok(vec![InjectExtraCodeBlock {
        code,
        should_parse: true,
    }])
}

fn generate_code_impl(enum_name: &str, variants: &[VariantInfo]) -> String {
    let enum_def = generate_enum_raw(variants, &enum_name, |variant| {
        format!("RustAutoOpaque<{}>", variant.ty_name)
    });

    let blocking_read_body = generate_match_raw(variants, |variant| {
        format!(
            "{enum_name}RwLockReadGuard::{}(inner.blocking_read())",
            variant.enum_variant_name
        )
    });
    let blocking_write_body = generate_match_raw(variants, |variant| {
        format!(
            "{enum_name}RwLockWriteGuard::{}(inner.blocking_write())",
            variant.enum_variant_name
        )
    });

    format!(
        "{enum_def}

        impl {enum_name} {{
            #[flutter_rust_bridge::frb(ignore)]
            pub fn blocking_read(&self) -> {enum_name}RwLockReadGuard {{
                {blocking_read_body}
            }}

            #[flutter_rust_bridge::frb(ignore)]
            pub fn blocking_write(&mut self) -> {enum_name}RwLockWriteGuard {{
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
    enum_name: &str,
    rw: ReadWrite,
    variants: &[VariantInfo],
) -> String {
    let rw_pascal = rw.to_string().to_case(Case::Pascal);

    let enum_name = format!("{enum_name}RwLock{rw_pascal}Guard");
    let enum_def = generate_enum_raw(variants, &format!("{enum_name}<'a>"), |variant| {
        format!(
            "flutter_rust_bridge::for_generated::rust_async::RwLock{rw_pascal}Guard<'a, {}>",
            variant.ty_name
        )
    });

    let deref_body = generate_match_raw(variants, |_| "inner.deref()".to_owned());
    let deref_code = format!(
        "
        impl std::ops::Deref for {enum_name}<'_> {{
            type Target = dyn TODO;

            fn deref(&self) -> &Self::Target {{
                {deref_body}
            }}
        }}
        "
    );

    let maybe_deref_mut_code = if rw == ReadWrite::Write {
        let body = generate_match_raw(variants, |_| "inner.deref_mut()".to_owned());
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
    variants: &[VariantInfo],
    enum_name: &str,
    wrapper: impl Fn(&VariantInfo) -> String,
) -> String {
    let variants = (variants.iter())
        .map(|variant| format!("{}({}),\n", variant.enum_variant_name, wrapper(variant)))
        .join("");

    format!(
        "pub enum {enum_name} {{
            {variants}
        }}"
    )
}

fn generate_match_raw(variants: &[VariantInfo], branch: impl Fn(&VariantInfo) -> String) -> String {
    let variants = (variants.iter())
        .map(|variant| {
            format!(
                "Self::{}(inner) => {},\n",
                &variant.enum_variant_name,
                branch(&variant)
            )
        })
        .join("");

    format!(
        "match self {{
            {variants}
        }}"
    )
}
