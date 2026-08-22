use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncOutput, MirFuncOwnerInfo, MirStandardOperator,
};
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::enumeration::{MirEnumMode, MirTypeEnumRef};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::function::real::parse_effective_function_name_of_method;
use crate::codegen::parser::mir::parser::function::sort_and_add_func_id;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn transform(mut pack: MirPack) -> anyhow::Result<MirPack> {
    let simple_enum_owners = pack
        .enum_pool
        .iter()
        .filter(|(_, enu)| enu.mode == MirEnumMode::Simple)
        .map(|(ident, _)| {
            MirType::EnumRef(MirTypeEnumRef {
                ident: ident.clone(),
                is_exception: false,
            })
            .safe_ident()
        })
        .collect::<HashSet<_>>();
    let existing_methods = pack
        .funcs_all
        .iter()
        .filter_map(method_key)
        .collect::<HashSet<_>>();

    let funcs = pack
        .funcs_all
        .drain(..)
        .flat_map(|func| transform_func(func, &existing_methods, &simple_enum_owners))
        .collect_vec();
    pack.funcs_all = sort_and_add_func_id(funcs);

    Ok(pack)
}

fn transform_func(
    func: MirFunc,
    existing_methods: &HashSet<(String, String)>,
    simple_enum_owners: &HashSet<String>,
) -> Vec<MirFunc> {
    let MirFuncOwnerInfo::Method(method) = &func.owner else {
        return vec![func];
    };

    if method.standard_operator() == Some(MirStandardOperator::PartialEq)
        && simple_enum_owners.contains(&method.owner_ty.safe_ident())
    {
        return vec![];
    }

    if method.trait_name.as_deref() == Some("PartialOrd")
        && method.trait_def.is_none()
        && method.actual_method_name == "partial_cmp"
    {
        return ["lt", "le", "gt", "ge"]
            .into_iter()
            .filter(|name| {
                !existing_methods.contains(&(method.owner_ty.safe_ident(), (*name).to_owned()))
            })
            .map(|name| create_partial_ord_operator(&func, name))
            .collect();
    }

    if method.is_standard_operator_trait() && method.standard_operator().is_none() {
        vec![]
    } else {
        vec![func]
    }
}

fn create_partial_ord_operator(template: &MirFunc, method_name: &str) -> MirFunc {
    let mut output = template.clone();
    let MirFuncOwnerInfo::Method(method) = &mut output.owner else {
        unreachable!()
    };
    method.actual_method_name = method_name.to_owned();
    output.name = MirIdent::new(parse_effective_function_name_of_method(method), None);
    output.output = MirFuncOutput {
        normal: MirType::Primitive(MirTypePrimitive::Bool),
        error: None,
    };
    output.rust_call_code = None;
    debug_assert!(matches!(
        method.standard_operator(),
        Some(
            MirStandardOperator::PartialOrdLt
                | MirStandardOperator::PartialOrdLe
                | MirStandardOperator::PartialOrdGt
                | MirStandardOperator::PartialOrdGe
        )
    ));
    output
}

fn method_key(func: &MirFunc) -> Option<(String, String)> {
    let MirFuncOwnerInfo::Method(method) = &func.owner else {
        return None;
    };
    Some((
        method.owner_ty.safe_ident(),
        method.actual_method_name.clone(),
    ))
}
