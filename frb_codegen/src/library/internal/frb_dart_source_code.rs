use crate::codegen::generator::wire::rust::spec_generator::codec::pde::entrypoint::{
    generate_ffi_dispatcher_raw, FfiDispatcherMode,
};
use crate::library::commands::cargo_metadata::execute_cargo_metadata;
use crate::library::commands::cbindgen::{cbindgen_raw, default_cbindgen_config};
use crate::library::commands::ffigen::{
    ffigen_raw, FfigenCommandConfig, FfigenCommandConfigHeaders,
};
use crate::utils::file_utils::temp_change_file;
use crate::utils::path_utils::path_to_string;
use convert_case::{Case, Casing};
use log::{debug, info};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use strum::IntoEnumIterator;

pub(crate) fn generate_frb_dart_source_code(repo_base_dir: &Path) -> anyhow::Result<()> {
    generate_frb_rust_cbindgen(repo_base_dir)?;
    generate_allo_isolate_cbindgen(repo_base_dir)?;

    ffigen(repo_base_dir)?;

    Ok(())
}

fn generate_frb_rust_cbindgen(repo_base_dir: &Path) -> anyhow::Result<()> {
    info!("generate_frb_rust_cbindgen");

    let dir_frb_rust = repo_base_dir.join("frb_rust");

    let extra_code = generate_frb_rust_extra_code();
    debug!("extra_code={extra_code:?}");
    let temp_change_handle = temp_change_file(dir_frb_rust.join("src").join("lib.rs"), |text| {
        text.unwrap() + &extra_code
    })?;

    let default_config = default_cbindgen_config();
    cbindgen(
        cbindgen::Config {
            parse: cbindgen::ParseConfig {
                expand: cbindgen::ParseExpandConfig {
                    crates: vec!["flutter_rust_bridge".to_owned()],
                    ..Default::default()
                },
                ..Default::default()
            },
            export: cbindgen::ExportConfig {
                rename: HashMap::from([("DartCObject".to_owned(), "Dart_CObject".to_owned())]),
                ..Default::default()
            },
            after_includes: Some(
                default_config.after_includes.unwrap()
                    + "\n"
                    + r#"#include "dart_api.h""#
                    + "\n"
                    + r#"#include "dart_native_api.h""#,
            ),
            ..default_config
        },
        repo_base_dir,
        &dir_frb_rust,
        "frb_rust",
    )?;

    drop(temp_change_handle);

    Ok(())
}

fn generate_frb_rust_extra_code() -> String {
    format!(
        "
        crate::frb_generated_io_extern_func!();
        crate::frb_generated_io_content_hash!();
        {}
        ",
        generate_ffi_dispatcher_raw(
            &FfiDispatcherMode::iter()
                .map(|m| (m, "".to_owned()))
                .collect(),
            "crate"
        ),
    )
}

fn generate_allo_isolate_cbindgen(repo_base_dir: &Path) -> anyhow::Result<()> {
    info!("generate_allo_isolate_cbindgen");

    let metadata = execute_cargo_metadata(&repo_base_dir.join("frb_codegen/Cargo.toml"))?;

    let package_name = "allo-isolate";
    let package = (metadata.packages.iter())
        .find(|package| package.name == package_name)
        .unwrap();
    let rust_crate_dir = package.manifest_path.as_std_path().parent().unwrap();

    let default_config = default_cbindgen_config();
    cbindgen(
        cbindgen::Config {
            export: cbindgen::ExportConfig {
                exclude: vec!["DartCObject".to_owned()],
                ..Default::default()
            },
            after_includes: Some(
                default_config.after_includes.unwrap()
                    + "\n"
                    + "struct DartCObject;\ntypedef struct DartCObject DartCObject;"
                    + "\n"
                    + r#"#include "dart_api.h""#,
            ),
            ..default_config
        },
        repo_base_dir,
        rust_crate_dir,
        "allo_isolate",
    )
}

fn cbindgen(
    config: cbindgen::Config,
    repo_base_dir: &Path,
    rust_crate_dir: &Path,
    name: &str,
) -> anyhow::Result<()> {
    let c_path = repo_base_dir.join(format!(
        "frb_dart/lib/src/ffigen_generated/intermediate/{}.h",
        name.to_case(Case::Snake)
    ));
    cbindgen_raw(config, rust_crate_dir, &c_path)
}

fn ffigen(repo_base_dir: &Path) -> anyhow::Result<()> {
    let dir_dart_api = repo_base_dir.join("frb_rust/src/dart_api");
    let dir_intermediate = repo_base_dir.join("frb_dart/lib/src/ffigen_generated/intermediate");

    let raw_headers = vec![
        dir_dart_api.join("dart_native_api.h"),
        dir_intermediate.join("allo_isolate.h"),
        dir_intermediate.join("frb_rust.h"),
    ];

    let output_path = repo_base_dir.join("frb_dart/lib/src/ffigen_generated/multi_package.dart");
    ffigen_raw(
        &FfigenCommandConfig {
            output: output_path.clone(),
            name: "MultiPackageCBinding".to_string(),
            headers: FfigenCommandConfigHeaders {
                entry_points: raw_headers.clone(),
                include_directives: raw_headers,
            },
            preamble: FFIGEN_PREAMBLE.to_owned(),
            description: FFIGEN_DESCRIPTION.to_owned(),
            compiler_opts: vec![
                // directory of `#include`
                format!("-I{}", &path_to_string(&dir_dart_api)?),
                format!("-I{}", &path_to_string(&dir_intermediate)?),
            ],
            // same reason as is discussed in ffigen.rs
            ignore_source_errors: true,
            ..Default::default()
        },
        &repo_base_dir.join("frb_dart"),
    )?;

    fs::write(
        &output_path,
        fs::read_to_string(&output_path)? + FFIGEN_POSTLUDE,
    )?;

    Ok(())
}

const FFIGEN_PREAMBLE: &str = "// AUTO-GENERATED BY frb_codegen::internal command\n// coverage:ignore-start\n// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names, unused_field, library_private_types_in_public_api, unused_element\n";
const FFIGEN_POSTLUDE: &str = "\n// coverage:ignore-end\n";
const FFIGEN_DESCRIPTION: &str = "generated by frb_codegen::internal command";
