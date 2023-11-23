use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::ir::func::{IrFunc, IrFuncOwnerInfo};
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use crate::utils::basic_code::DartBasicHeaderCode;
use anyhow::Result;
use itertools::Itertools;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

pub(crate) mod base;
pub(crate) mod class;
pub(crate) mod function;
pub(crate) mod info;
pub(crate) mod misc;

#[derive(Serialize)]
pub(crate) struct ApiDartOutputSpec {
    pub namespaced_items: HashMap<Namespace, ApiDartOutputSpecItem>,
}

#[derive(Serialize)]
pub(crate) struct ApiDartOutputSpecItem {
    pub funcs: Vec<ApiDartGeneratedFunction>,
    pub classes: Vec<ApiDartGeneratedClass>,
    pub imports: DartBasicHeaderCode,
    pub needs_freezed: bool,
}

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorApiDartInternalConfig,
) -> Result<ApiDartOutputSpec> {
    let cache = IrPackComputedCache::compute(ir_pack);
    let context = ApiDartGeneratorContext { ir_pack, config };

    let grouped_funcs = (ir_pack.funcs.iter()).into_group_map_by(|x| x.name.namespace.clone());
    let grouped_classes = (cache.distinct_types.iter())
        .filter(|x| x.self_namespace().is_some())
        .into_group_map_by(|x| x.self_namespace().unwrap());

    let namespaces = grouped_funcs
        .keys()
        .chain(grouped_classes.keys())
        .collect::<HashSet<_>>();

    let namespaced_items = namespaces
        .iter()
        .map(|&namespace| {
            (
                namespace.to_owned(),
                generate_item(
                    &grouped_classes.get(namespace),
                    &grouped_funcs.get(namespace),
                    context,
                ),
            )
        })
        .collect();

    Ok(ApiDartOutputSpec { namespaced_items })
}

fn generate_item(
    classes: &Option<&Vec<&IrType>>,
    funcs: &Option<&Vec<&IrFunc>>,
    context: ApiDartGeneratorContext,
) -> ApiDartOutputSpecItem {
    let imports = generate_imports(classes, funcs);

    let funcs = funcs
        .map(|funcs| {
            funcs
                .iter()
                .filter(|f| f.owner == IrFuncOwnerInfo::Function)
                .map(|f| function::generate(f, context))
                .collect_vec()
        })
        .unwrap_or_default();

    let classes = classes
        .map(|classes| {
            classes
                .iter()
                .filter_map(|&ty| ApiDartGenerator::new(ty.clone(), context).generate_class())
                .collect_vec()
        })
        .unwrap_or_default();

    let needs_freezed = classes.iter().any(|class| class.needs_freezed);

    ApiDartOutputSpecItem {
        funcs,
        classes,
        imports,
        needs_freezed,
    }
}

fn generate_imports(
    classes: &Option<&Vec<&IrType>>,
    funcs: &Option<&Vec<&IrFunc>>,
) -> DartBasicHeaderCode {
    let interest_types = TODO;

    DartBasicHeaderCode {
        import: TODO,
        ..Default::default()
    }
}
