use crate::codegen::dumper::Dumper;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::misc::generate_imports_which_types_and_funcs_use;
use crate::codegen::generator::api_dart::spec_generator::sanity_checker::sanity_check_class_name_duplicates;
use crate::codegen::ir::mir::func::{MirFunc, MirFuncOwnerInfo};
use crate::codegen::ir::mir::pack::{MirPack, MirPackComputedCache};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::misc::skip::IrSkip;
use crate::codegen::ConfigDumpContent;
use crate::library::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use crate::utils::namespace::Namespace;
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
pub(crate) mod sanity_checker;

#[derive(Serialize)]
pub(crate) struct ApiDartOutputSpec {
    pub namespaced_items: HashMap<Namespace, ApiDartOutputSpecItem>,
}

#[derive(Serialize)]
pub(crate) struct ApiDartOutputSpecItem {
    pub funcs: Vec<ApiDartGeneratedFunction>,
    pub classes: Vec<ApiDartGeneratedClass>,
    pub extra_impl_code: Vec<String>,
    pub imports: DartHeaderCode,
    pub preamble: String,
    pub skips: Vec<IrSkip>,
    pub needs_freezed: bool,
}

pub(crate) fn generate(
    mir_pack: &MirPack,
    config: &GeneratorApiDartInternalConfig,
    dumper: &Dumper,
) -> Result<ApiDartOutputSpec> {
    let cache = MirPackComputedCache::compute(mir_pack);
    let context = ApiDartGeneratorContext { mir_pack, config };

    (dumper.with_content(GeneratorInfo))
        .dump("api_dart.json", &generate_dump_info(&cache, context))?;

    let funcs_with_impl = mir_pack.funcs_with_impl();
    let grouped_funcs = (funcs_with_impl.iter()).into_group_map_by(|x| x.name.namespace.clone());
    let grouped_namespaced_types = (cache.distinct_types.iter())
        .filter(|x| x.self_namespace().is_some())
        .into_group_map_by(|x| x.self_namespace().unwrap());

    let namespaces = (grouped_funcs.keys())
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
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|(_, x)| {
            !x.funcs.is_empty() || !x.classes.is_empty() || !x.extra_impl_code.is_empty()
        })
        .collect::<HashMap<_, _>>();

    Ok(ApiDartOutputSpec { namespaced_items })
}

fn generate_item(
    namespace: &Namespace,
    namespaced_types: &Option<&Vec<&MirType>>,
    funcs: &Option<&Vec<&MirFunc>>,
    context: ApiDartGeneratorContext,
) -> Result<ApiDartOutputSpecItem> {
    let imports = DartHeaderCode {
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
                .filter(|f| (f.owner == MirFuncOwnerInfo::Function) && !f.initializer)
                .map(|f| function::generate(f, context))
                .collect::<Result<Vec<_>>>()
        })
        .unwrap_or(Ok(vec![]))?;

    let classes = namespaced_types
        .map(|types| {
            (types.iter())
                .filter_map(|&ty| ApiDartGenerator::new(ty.clone(), context).generate_class())
                .collect_vec()
        })
        .unwrap_or_default();

    let extra_impl_code = namespaced_types
        .map(|classes| {
            (classes.iter())
                .filter_map(|&ty| {
                    ApiDartGenerator::new(ty.clone(), context).generate_extra_impl_code()
                })
                .collect_vec()
        })
        .unwrap_or_default();

    sanity_check_class_name_duplicates(&classes)?;

    let needs_freezed = classes.iter().any(|class| class.needs_freezed);

    Ok(ApiDartOutputSpecItem {
        funcs,
        classes,
        extra_impl_code,
        imports,
        preamble: context.config.dart_preamble.clone(),
        skips: compute_skips(context.mir_pack, namespace),
        needs_freezed,
    })
}

fn compute_skips(mir_pack: &MirPack, namespace: &Namespace) -> Vec<IrSkip> {
    (mir_pack.skips.iter())
        .filter(|t| &t.name.namespace == namespace)
        .sorted_by_cached_key(|x| x.name.clone())
        .cloned()
        .collect_vec()
}
