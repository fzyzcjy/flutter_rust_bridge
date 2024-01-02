use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::misc::generate_imports_which_types_and_funcs_use;
use crate::codegen::generator::misc::generate_code_header;
use crate::codegen::generator::misc::target::{TargetOrCommon, TargetOrCommonMap};
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use crate::codegen::generator::wire::dart::spec_generator::base::{
    WireDartGenerator, WireDartGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::library::codegen::generator::wire::dart::spec_generator::misc::ty::WireDartGeneratorMiscTrait;
use crate::utils::basic_code::DartBasicHeaderCode;
use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use convert_case::{Case, Casing};
use itertools::Itertools;
use pathdiff::diff_paths;
use serde::Serialize;
use std::path::{Path, PathBuf};

mod api_impl_body;
pub(crate) mod ty;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpecMisc {
    pub(crate) wire_class: Acc<Vec<WireDartOutputCode>>,
    pub(crate) boilerplate: Acc<Vec<WireDartOutputCode>>,
    pub(crate) api_impl_normal_functions: Vec<WireDartOutputCode>,
    pub(crate) extra_functions: Acc<Vec<WireDartOutputCode>>,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
    c_file_content: &str,
    api_dart_actual_output_paths: &[PathBuf],
    rust_extern_funcs: &[ExternFunc],
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<WireDartOutputSpecMisc> {
    Ok(WireDartOutputSpecMisc {
        wire_class: super::wire_class::generate(
            context.config,
            c_file_content,
            rust_extern_funcs,
            progress_bar_pack,
        )?,
        boilerplate: generate_boilerplate(api_dart_actual_output_paths, cache, context)?,
        api_impl_normal_functions: (context.ir_pack.funcs.iter())
            .map(|f| api_impl_body::generate_api_impl_normal_function(f, context))
            .collect::<anyhow::Result<Vec<_>>>()?,
        extra_functions: (cache.distinct_types.iter())
            .flat_map(|ty| WireDartGenerator::new(ty.clone(), context).generate_extra_functions())
            .collect(),
    })
}

fn generate_boilerplate(
    api_dart_actual_output_paths: &[PathBuf],
    cache: &IrPackComputedCache,
    context: WireDartGeneratorContext,
) -> anyhow::Result<Acc<Vec<WireDartOutputCode>>> {
    let DartOutputClassNamePack {
        entrypoint_class_name,
        api_class_name,
        api_impl_class_name,
        wire_class_name,
        ..
    } = &context.config.dart_output_class_name_pack;

    let file_top = generate_code_header()
        + "\n\n// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables\n";

    let mut universal_imports = generate_import_dart_api_layer(
        &context.config.dart_impl_output_path,
        api_dart_actual_output_paths,
    )?;
    universal_imports += &generate_imports_which_types_and_funcs_use(
        &Namespace::new_self_crate(file_stem(&context.config.dart_impl_output_path.io)),
        &Some(&cache.distinct_types.iter().collect_vec()),
        &None,
        context.as_api_dart_context(),
    )?;
    universal_imports += "
    import 'dart:convert';
    import 'dart:async';
    ";

    let initializers = (context.ir_pack.funcs.iter())
        .filter(|f| f.initializer)
        .collect_vec();
    let initializer_imports = initializers
        .iter()
        .map(|f| format!(r#"import '{TODO}';"#))
        .join("");
    let execute_rust_initializers = initializers
        .iter()
        .map(|f| format!("await api.{}();\n", f.name.name.to_case(Case::Camel)))
        .join("");

    Ok(Acc {
        common: vec![WireDartOutputCode {
            header: DartBasicHeaderCode {
                file_top: file_top.clone(),
                import: format!(
                    "
                    {universal_imports}{initializer_imports}
                    import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
                    import 'frb_generated.io.dart' if (dart.library.html) 'frb_generated.web.dart';
                    "
                ),
                ..Default::default()
            },
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
                    ExternalLibrary? externalLibrary,
                  }}) async {{
                    await instance.initImpl(
                      api: api,
                      handler: handler,
                      externalLibrary: externalLibrary,
                    );
                  }}
                  
                  /// Dispose flutter_rust_bridge
                  ///
                  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
                  /// is automatically disposed when the app stops.
                  static void dispose() => instance.disposeImpl();

                  @override
                  ApiImplConstructor<{api_impl_class_name}, {wire_class_name}> get apiImplConstructor => {api_impl_class_name}.new;

                  @override
                  WireConstructor<{wire_class_name}> get wireConstructor => {wire_class_name}.fromExternalLibrary;
                  
                  @override
                  Future<void> executeRustInitializers() async {{
                    {execute_rust_initializers}
                  }}

                  @override
                  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig => kDefaultExternalLibraryLoaderConfig;
                  
                  static const kDefaultExternalLibraryLoaderConfig = ExternalLibraryLoaderConfig(
                    stem: '{stem}',
                    ioDirectory: '{io_directory}',
                    webPrefix: '{web_prefix}',
                  );
                }}
                "#,
                stem = context.config.default_external_library_loader.stem,
                io_directory = context.config.default_external_library_loader.io_directory,
                web_prefix = context.config.default_external_library_loader.web_prefix,
            ),
            ..Default::default()
        }],
        io: vec![WireDartOutputCode {
            header: DartBasicHeaderCode {
                file_top: file_top.clone(),
                import: format!(
                    "
                    {universal_imports}
                    import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
                    import 'frb_generated.dart';
                    "
                ),
                ..Default::default()
            },
            ..Default::default()
        }],
        web: vec![WireDartOutputCode {
            header: DartBasicHeaderCode {
                file_top,
                import: format!(
                    "
                    {universal_imports}
                    import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';
                    import 'frb_generated.dart';
                    "
                ),
                ..Default::default()
            },
            ..Default::default()
        }],
    })
}

fn file_stem(p: &Path) -> String {
    p.file_stem().unwrap().to_str().unwrap().into()
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
            let relative_path = path_to_string(&relative_path)?.replace('\\', "/");
            Ok(format!("import '{relative_path}';\n"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .join(""))
}
