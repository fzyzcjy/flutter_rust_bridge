use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::tentative_parse_trait_impls;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    let trait_impls = tentative_parse_trait_impls(&pack);

    for hir_trait in &pack.traits {
        if FrbAttributes::parse(&hir_trait.attrs)?.generate_impl_enum() {
            generate_trait_impl_enum(hir_trait, &trait_impls);
        }
    }

    Ok(pack)
}

fn generate_trait_impl_enum(hir_trait: &HirFlatTrait, trait_impls: &[MirTraitImpl]) {
    TODO;
}
