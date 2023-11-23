use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::ir::func::{IrFunc, IrFuncOwnerInfo};
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::{DistinctTypeGatherer, IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use crate::utils::basic_code::DartBasicHeaderCode;
use crate::utils::path_utils::path_to_string;
use anyhow::{Context, Result};
use itertools::Itertools;
use pathdiff::diff_paths;
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
    let imports = generate_imports(namespace, namespaced_types, funcs, context.ir_pack)?;

    let funcs = funcs
        .map(|funcs| {
            funcs
                .iter()
                .filter(|f| f.owner == IrFuncOwnerInfo::Function)
                .map(|f| function::generate(f, context))
                .collect_vec()
        })
        .unwrap_or_default();

    let classes = namespaced_types
        .map(|classes| {
            classes
                .iter()
                .filter_map(|&ty| ApiDartGenerator::new(ty.clone(), context).generate_class())
                .collect_vec()
        })
        .unwrap_or_default();

    let needs_freezed = classes.iter().any(|class| class.needs_freezed);

    Ok(ApiDartOutputSpecItem {
        funcs,
        classes,
        imports,
        needs_freezed,
    })
}

fn generate_imports(
    current_file_namespace: &Namespace,
    namespaced_types: &Option<&Vec<&IrType>>,
    funcs: &Option<&Vec<&IrFunc>>,
    ir_pack: &IrPack,
) -> Result<DartBasicHeaderCode> {
    let mut gatherer = DistinctTypeGatherer::new();
    if let Some(namespaced_types) = namespaced_types {
        (namespaced_types.iter()).for_each(|x| x.visit_types(&mut |ty| gatherer.add(ty), ir_pack));
    }
    if let Some(funcs) = funcs {
        (funcs.iter()).for_each(|x| x.visit_types(&mut |ty| gatherer.add(ty), true, true, ir_pack));
    }
    let interest_types = gatherer.gather();

    let import = interest_types
        .iter()
        .filter_map(|ty| ty.self_namespace())
        .filter(|import_ty_namespace| import_ty_namespace != current_file_namespace)
        .map(|import_ty_namespace| {
            let path_diff = diff_paths(
                import_ty_namespace.to_pseudo_io_path("dart"),
                (current_file_namespace.to_pseudo_io_path("dart").parent()).unwrap(),
            )
            .context("cannot diff path")?;
            Ok(format!(
                "import '{}';\n",
                path_to_string(&path_diff).unwrap()
            ))
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .join("");

    Ok(DartBasicHeaderCode {
        import,
        ..Default::default()
    })
}
