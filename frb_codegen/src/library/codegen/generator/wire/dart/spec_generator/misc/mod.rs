use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{TargetOrCommon, TargetOrCommonMap};
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use itertools::Itertools;
use pathdiff::diff_paths;
use serde::Serialize;
use std::path::{Path, PathBuf};

mod api_impl_body;
mod c_binding;
pub(crate) mod ty;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpecMisc {
    pub(crate) c_binding: WireDartOutputCode,
    pub(crate) boilerplate: Acc<Vec<WireDartOutputCode>>,
    pub(crate) api_impl_normal_functions: Vec<WireDartOutputCode>,
    pub(crate) api_impl_opaque_getters: Vec<WireDartOutputCode>,
    pub(crate) needs_freezed: bool,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
    c_file_content: &str,
    api_dart_actual_output_paths: &[PathBuf],
) -> anyhow::Result<WireDartOutputSpecMisc> {
    Ok(WireDartOutputSpecMisc {
        c_binding: c_binding::generate(&context.config, c_file_content)?,
        boilerplate: generate_boilerplate(
            &context.config.dart_output_class_name_pack,
            &context.config.default_external_library_stem,
            &context.config.default_external_library_relative_directory,
            &context.config.dart_impl_output_path,
            api_dart_actual_output_paths,
        )?,
        api_impl_normal_functions: (context.ir_pack.funcs.iter())
            .map(|f| api_impl_body::generate_api_impl_normal_function(f, context))
            .collect(),
        api_impl_opaque_getters: (cache.distinct_types.iter())
            .filter_map(|ty| api_impl_body::generate_api_impl_opaque_getter(ty, context))
            .collect(),
        needs_freezed: compute_needs_freezed(cache, context.ir_pack),
    })
}

fn generate_boilerplate(
    dart_output_class_name_pack: &DartOutputClassNamePack,
    default_external_library_stem: &str,
    default_external_library_relative_directory: &str,
    dart_impl_output_path: &TargetOrCommonMap<PathBuf>,
    api_dart_actual_output_paths: &[PathBuf],
) -> anyhow::Result<Acc<Vec<WireDartOutputCode>>> {
    let DartOutputClassNamePack {
        entrypoint_class_name,
        api_class_name,
        api_impl_class_name,
        wire_class_name,
        ..
    } = &dart_output_class_name_pack;

    let file_top = "// ignore_for_file: unused_import\n".to_owned();

    let universal_imports =
        generate_import_dart_api_layer(dart_impl_output_path, api_dart_actual_output_paths)?
            + "\nimport 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';";

    Ok(Acc {
        common: vec![WireDartOutputCode {
            file_top: file_top.clone(),
            import: format!(
                "
                {universal_imports}
                import 'frb_generated.io.dart' if (dart.library.html) 'frb_generated.web.dart.dart';
            "
            ),
            body_top: format!(
                r#"
                /// Main entrypoint of the Rust API
                class {entrypoint_class_name} extends BaseEntrypoint<{api_class_name}, {api_impl_class_name}, {wire_class_name}> {{
                  @internal
                  static final instance = {entrypoint_class_name}._();

                  {entrypoint_class_name}._();

                  /// Initialize flutter_rust_bridge
                  static Future<void> init({{
                    {api_class_name}? api,
                    BaseHandler? handler,
                  }}) async {{
                    await instance.initImpl(api: api, handler: handler);
                  }}
                  
                  /// Dispose flutter_rust_bridge
                  ///
                  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
                  /// is automatically disposed when the app stops.
                  static void dispose() => instance.disposeImpl();

                  @override
                  ApiImplConstructor<{api_impl_class_name}, {wire_class_name}> get apiImplConstructor => {api_impl_class_name}.new;

                  @override
                  WireConstructor<{wire_class_name}> get wireConstructor => {wire_class_name}.new;

                  @override
                  String get defaultExternalLibraryStem => '{default_external_library_stem}';
                  
                  @override
                  String get defaultExternalLibraryRelativeDirectory => '{default_external_library_relative_directory}';
                }}
                "#
            ),
            ..Default::default()
        }],
        io: vec![WireDartOutputCode {
            file_top: file_top.clone(),
            import: universal_imports.to_owned(),
            ..Default::default()
        }],
        wasm: vec![WireDartOutputCode {
            file_top: file_top.clone(),
            import: universal_imports.to_owned(),
            body: format!(
                r#"
                class {wire_class_name} extends BaseWire {{
                  // TODO
                }}
                "#
            ),
            ..Default::default()
        }],
    })
}

fn generate_import_dart_api_layer(
    dart_impl_output_path: &TargetOrCommonMap<PathBuf>,
    api_dart_actual_output_paths: &[PathBuf],
) -> anyhow::Result<String> {
    Ok(api_dart_actual_output_paths
        .iter()
        .map(|path| {
            let dir_base = (dart_impl_output_path[TargetOrCommon::Common].parent())
                .context("cannot find parent dir")?;
            let relative_path = diff_paths(path, dir_base).context("cannot find relative path")?;
            let relative_path = path_to_string(&relative_path)?;
            Ok(format!("import '{relative_path}';\n"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .join(""))
}

fn compute_needs_freezed(cache: &IrPackComputedCache, ir_pack: &IrPack) -> bool {
    cache
        .distinct_types
        .iter()
        .any(|ty| compute_needs_freezed_for_type(ty, ir_pack))
}

fn compute_needs_freezed_for_type(ty: &IrType, ir_pack: &IrPack) -> bool {
    match ty {
        EnumRef(_) => true,
        StructRef(st) => st.get(ir_pack).using_freezed(),
        _ => false,
    }
}
