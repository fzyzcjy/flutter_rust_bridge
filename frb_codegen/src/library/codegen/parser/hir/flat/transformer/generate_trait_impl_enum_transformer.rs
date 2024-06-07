use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::tentative_parse_trait_impls;
use itertools::Itertools;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    let trait_impls = tentative_parse_trait_impls(&pack);

    let extra_code = (pack.traits.iter())
        .filter(|x| FrbAttributes::parse(&x.attrs).unwrap().generate_impl_enum())
        .map(|x| generate_trait_impl_enum(x, &trait_impls))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .join("");

    inject_extra_code(&mut pack, &extra_code);

    Ok(pack)
}

fn generate_trait_impl_enum(
    hir_trait: &HirFlatTrait,
    _trait_impls: &[MirTraitImpl],
) -> anyhow::Result<String> {
    let trait_def_name = &hir_trait.name.name;

    // TODO
    Ok(format!(
        "enum {trait_def_name}Impl {{
            Hello(i32),
        }}
        "
    ))
}

fn inject_extra_code(pack: &mut HirFlatPack, extra_code: &str) {
    TODO;
}
