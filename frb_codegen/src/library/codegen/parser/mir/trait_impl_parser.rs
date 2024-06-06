use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::parser::mir::type_parser::TypeParser;
use crate::utils::syn_utils::ty_to_string;
use itertools::Itertools;

pub(super) fn parse(
    hir_trait_impls: &[HirFlatTraitImpl],
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirTraitImpl>> {
    (hir_trait_impls.iter())
        .map(|x| MirTraitImpl {
            trait_ty: x.trait_name.clone(),
            impl_ty: ty_to_string(&x.impl_ty),
        })
        .collect_vec()
}
