use crate::codegen::dumper::Dumper;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::misc::generate_imports_which_types_and_funcs_use;
use crate::codegen::ir::func::{IrFunc, IrFuncOwnerInfo};
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ConfigDumpContent;
use crate::library::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use crate::utils::basic_code::DartBasicHeaderCode;
use anyhow::Result;
use itertools::Itertools;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use ConfigDumpContent::GeneratorInfo;

pub(crate) mod base;
pub(crate) mod class;
mod dump;
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
    dumper: &Dumper,
) -> Result<ApiDartOutputSpec> {
    let cache = IrPackComputedCache::compute(ir_pack);
    let context = ApiDartGeneratorContext { ir_pack, config };

    dumper.dump(
        GeneratorInfo,
        "api_dart.json",
        &generate_dump_info(&cache, context),
    )?;

    let grouped_funcs = (ir_pack.funcs.iter()).into_group_map_by(|x| x.name.namespace.clone());
    let grouped_namespaced_types = (cache.distinct_types.iter())
        .filter(|x| x.self_namespace().is_some())
        .into_group_map_by(|x| x.self_namespace().unwrap());

    let namespaces = grouped_funcs
        .keys()
        .chain(grouped_namespaced_types.keys())
        .collect::<HashSet<_>>();

    let namespaced_items = namespaces
        .iter()
        .map(|&namespace| {
            Ok((
                namespace.to_owned(),
                generate_item(
                    namespace,
                    &grouped_namespaced_types.get(namespace),
                    &grouped_funcs.get(namespace),
                    context,
                )?,
            ))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    Ok(ApiDartOutputSpec { namespaced_items })
}

fn generate_item(
    namespace: &Namespace,
    namespaced_types: &Option<&Vec<&IrType>>,
    funcs: &Option<&Vec<&IrFunc>>,
    context: ApiDartGeneratorContext,
) -> Result<ApiDartOutputSpecItem> {
    let imports = DartBasicHeaderCode {
        import: generate_imports_which_types_and_funcs_use(
            namespace,
            namespaced_types,
            funcs,
            context,
        )?,
        ..Default::default()
    };

    let funcs = funcs
        .map(|funcs| {
            funcs
                .iter()
                .filter(|f| f.owner == IrFuncOwnerInfo::Function)
                .map(|f| function::generate(f, context))
                .collect::<Result<Vec<_>>>()
        })
        .unwrap_or(Ok(vec![]))?;

    let classes = namespaced_types
        .map(|classes| {
            Ok::<_, anyhow::Error>(
                classes
                    .iter()
                    .filter_map(|&ty| ApiDartGenerator::new(ty.clone(), context).generate_class())
                    .map(|c| {
                        c.sanity_check()?;
                        Ok(c)
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .unique_by(|c| (c.namespace.clone(), c.class_name.clone()))
                    .collect_vec(),
            )
        })
        .unwrap_or(Ok(vec![]))?;

    let needs_freezed = classes.iter().any(|class| class.needs_freezed);

    Ok(ApiDartOutputSpecItem {
        funcs,
        classes,
        imports,
        needs_freezed,
    })
}
