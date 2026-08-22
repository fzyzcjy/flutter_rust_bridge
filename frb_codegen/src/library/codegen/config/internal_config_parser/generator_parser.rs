use crate::codegen::config::internal_config::{
    GeneratorInternalConfig, GeneratorWireInternalConfig,
};
use crate::codegen::config::internal_config_parser::dart_path_parser::DartOutputPathPack;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::{
    DartOutputClassNamePack, GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig,
    GeneratorWireDartInternalConfig,
};
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::Config;
use crate::library::commands::cargo_metadata::execute_cargo_metadata;
use crate::utils::dart_repository::get_dart_package_name;
use crate::utils::path_utils::path_to_string;
use crate::utils::syn_utils::canonicalize_rust_type;
use anyhow::Context;
use itertools::Itertools;
use pathdiff::diff_paths;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(super) struct Args<'a> {
    pub config: &'a Config,
    pub dart_root: &'a Path,
    pub rust_crate_dir: &'a Path,
    pub dart_output_path_pack: &'a DartOutputPathPack,
    pub dart_output_class_name_pack: &'a DartOutputClassNamePack,
    pub rust_output_path: &'a Path,
    pub default_stream_sink_codec: CodecMode,
    pub default_rust_opaque_codec: RustOpaqueCodecMode,
    pub c_output_path: &'a Option<PathBuf>,
    pub web_enabled: bool,
    pub full_dep: bool,
}

pub(super) fn parse(args: Args) -> anyhow::Result<GeneratorInternalConfig> {
    let Args {
        config,
        dart_root,
        rust_crate_dir,
        dart_output_path_pack,
        dart_output_class_name_pack,
        rust_output_path,
        default_stream_sink_codec,
        default_rust_opaque_codec,
        c_output_path,
        web_enabled,
        full_dep,
    } = args;

    let dart_enums_style = config.dart_enums_style.unwrap_or(true);
    let dart3 = config.dart3.unwrap_or(true);
    let default_external_library_loader =
        compute_default_external_library_loader(rust_crate_dir, dart_root, config);
    let c_symbol_prefix = compute_c_symbol_prefix(dart_root)?;

    Ok(GeneratorInternalConfig {
        api_dart: GeneratorApiDartInternalConfig {
            dart_collection_deep_equality: config.dart_collection_deep_equality.unwrap_or(false),
            dart_enums_style,
            dart3,
            dart_decl_base_output_path: dart_output_path_pack.dart_decl_base_output_path.clone(),
            dart_impl_output_path: dart_output_path_pack.dart_impl_output_path.clone(),
            dart_entrypoint_class_name: dart_output_class_name_pack.entrypoint_class_name.clone(),
            dart_preamble: config.dart_preamble.clone().unwrap_or_default(),
            dart_type_rename: compute_dart_type_rename(config)?,
        },
        wire: GeneratorWireInternalConfig {
            dart: GeneratorWireDartInternalConfig {
                dart_root: dart_root.to_owned(),
                web_enabled,
                llvm_path: config
                    .llvm_path
                    .clone()
                    .unwrap_or_else(fallback_llvm_path)
                    .into_iter()
                    .map(PathBuf::from)
                    .collect_vec(),
                llvm_compiler_opts: config.llvm_compiler_opts.clone().unwrap_or_default(),
                extra_headers: config.extra_headers.clone().unwrap_or_default(),
                dart_impl_output_path: dart_output_path_pack.dart_impl_output_path.clone(),
                dart_output_class_name_pack: dart_output_class_name_pack.to_owned(),
                default_external_library_loader,
                c_symbol_prefix: c_symbol_prefix.clone(),
                has_ffigen: full_dep,
            },
            rust: GeneratorWireRustInternalConfig {
                rust_crate_dir: rust_crate_dir.to_owned(),
                web_enabled,
                rust_output_path: rust_output_path.to_owned(),
                c_symbol_prefix: c_symbol_prefix.clone(),
                has_ffigen: full_dep,
                default_stream_sink_codec,
                default_rust_opaque_codec,
                rust_preamble: config.rust_preamble.clone().unwrap_or_default(),
            },
            c: GeneratorWireCInternalConfig {
                enable: full_dep,
                rust_crate_dir: rust_crate_dir.to_owned(),
                rust_output_path: rust_output_path.to_owned(),
                c_output_path: c_output_path.clone(),
                c_symbol_prefix,
            },
        },
    })
}

fn fallback_llvm_path() -> Vec<String> {
    vec![
        "/opt/homebrew/opt/llvm".to_owned(), // Homebrew root
        "/usr/local/opt/llvm".to_owned(),    // Homebrew x86-64 root
        // Possible Linux LLVM roots
        "/usr/lib/llvm-9".to_owned(),
        "/usr/lib/llvm-10".to_owned(),
        "/usr/lib/llvm-11".to_owned(),
        "/usr/lib/llvm-12".to_owned(),
        "/usr/lib/llvm-13".to_owned(),
        "/usr/lib/llvm-14".to_owned(),
        "/usr/lib/".to_owned(),
        "/usr/lib64/".to_owned(),
        "C:/Program Files/llvm".to_owned(), // Default on Windows
        "C:/msys64/mingw64".to_owned(), // https://packages.msys2.org/package/mingw-w64-x86_64-clang
    ]
}

fn compute_c_symbol_prefix(dart_root: &Path) -> anyhow::Result<String> {
    let package_name = get_dart_package_name(dart_root)?;
    Ok(format!("frbgen_{package_name}_"))
}

fn compute_default_external_library_loader(
    rust_crate_dir: &Path,
    dart_root: &Path,
    config: &Config,
) -> GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig {
    const FALLBACK_DEFAULT_EXTERNAL_LIBRARY_STEM: &str = "UNKNOWN";
    const FALLBACK_DEFAULT_EXTERNAL_LIBRARY_RELATIVE_DIRECTORY: &str = "UNKNOWN";

    GeneratorWireDartDefaultExternalLibraryLoaderInternalConfig {
        stem: compute_default_external_library_stem(rust_crate_dir)
            .unwrap_or(FALLBACK_DEFAULT_EXTERNAL_LIBRARY_STEM.to_owned()),
        io_directory: compute_default_external_library_relative_directory(
            rust_crate_dir,
            dart_root,
        )
        .unwrap_or(FALLBACK_DEFAULT_EXTERNAL_LIBRARY_RELATIVE_DIRECTORY.to_owned()),
        web_prefix: config
            .default_external_library_loader_web_prefix
            .as_deref()
            .unwrap_or("pkg/")
            .into(),
        wasm_bindgen_name: config
            .wasm_bindgen_name
            .as_deref()
            .unwrap_or("wasm_bindgen")
            .into(),
    }
}

fn compute_default_external_library_stem(rust_crate_dir: &Path) -> anyhow::Result<String> {
    let metadata = execute_cargo_metadata(&rust_crate_dir.join("Cargo.toml"))?;
    let package = metadata
        .root_package()
        .context("cannot find root package")?;
    let target = (package.targets.iter())
        .find(|target| target.kind.iter().any(|kind| kind.contains("lib")))
        .context("cannot find target")?;
    Ok(target.name.replace('-', "_"))
}

fn compute_default_external_library_relative_directory(
    rust_crate_dir: &Path,
    dart_root: &Path,
) -> anyhow::Result<String> {
    let diff = diff_paths(rust_crate_dir, dart_root).context("cannot diff path")?;
    Ok(path_to_string(&diff.join("target").join("release/"))?.replace('\\', "/"))
}

fn compute_dart_type_rename(config: &Config) -> anyhow::Result<HashMap<String, String>> {
    fn convert_rust_type(raw: &str) -> anyhow::Result<Vec<String>> {
        Ok(vec![
            canonicalize_rust_type(raw)?,
            canonicalize_rust_type(&format!(
                "flutter_rust_bridge::for_generated::RustAutoOpaqueInner<{raw}>"
            ))?,
        ])
    }

    Ok(config
        .dart_type_rename
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|(k, v)| {
            Ok(convert_rust_type(k)?
                .into_iter()
                .map(|parsed_k| (parsed_k, v.to_owned()))
                .collect_vec())
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{
        compute_c_symbol_prefix, compute_dart_type_rename, compute_default_external_library_loader,
        compute_default_external_library_relative_directory, fallback_llvm_path,
    };
    use crate::codegen::Config;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    /// Keeps the LLVM fallback search order stable across supported platforms.
    #[test]
    fn fallback_llvm_path_has_stable_supported_platform_order() {
        assert_eq!(
            fallback_llvm_path(),
            vec![
                "/opt/homebrew/opt/llvm",
                "/usr/local/opt/llvm",
                "/usr/lib/llvm-9",
                "/usr/lib/llvm-10",
                "/usr/lib/llvm-11",
                "/usr/lib/llvm-12",
                "/usr/lib/llvm-13",
                "/usr/lib/llvm-14",
                "/usr/lib/",
                "/usr/lib64/",
                "C:/Program Files/llvm",
                "C:/msys64/mingw64",
            ]
        );
    }

    /// Derives the C symbol prefix from the Dart pubspec package name.
    #[test]
    fn computes_c_symbol_prefix_from_dart_package_name() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        fs::write(
            temp_dir.path().join("pubspec.yaml"),
            "name: bridge_package\n",
        )?;

        assert_eq!(
            compute_c_symbol_prefix(temp_dir.path())?,
            "frbgen_bridge_package_"
        );
        Ok(())
    }

    /// Produces a slash-normalized relative release loader directory.
    #[test]
    fn computes_relative_loader_directory_with_normalized_slashes() -> anyhow::Result<()> {
        let directory = compute_default_external_library_relative_directory(
            Path::new("project/native"),
            Path::new("project/dart"),
        )?;

        assert_eq!(directory, "../native/target/release/");
        assert!(!directory.contains('\\'));
        Ok(())
    }

    /// Falls back when Cargo metadata cannot identify a Rust library package.
    #[test]
    fn loader_uses_unknown_fallbacks_when_cargo_metadata_fails() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let dart_root = temp_dir.path().join("dart");
        let rust_crate_dir = temp_dir.path().join("native");
        fs::create_dir_all(&dart_root)?;
        fs::create_dir_all(&rust_crate_dir)?;

        let loader = compute_default_external_library_loader(
            &rust_crate_dir,
            &dart_root,
            &Config::default(),
        );

        assert_eq!(loader.stem, "UNKNOWN");
        assert_eq!(loader.io_directory, "../native/target/release/");
        assert_eq!(loader.web_prefix, "pkg/");
        assert_eq!(loader.wasm_bindgen_name, "wasm_bindgen");
        Ok(())
    }

    /// Applies loader overrides while reading a real minimal Cargo package.
    #[test]
    fn loader_uses_cargo_metadata_and_configured_defaults() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let dart_root = temp_dir.path().join("dart");
        let rust_crate_dir = temp_dir.path().join("native");
        fs::create_dir_all(rust_crate_dir.join("src"))?;
        fs::create_dir_all(&dart_root)?;
        fs::write(
            rust_crate_dir.join("Cargo.toml"),
            "[package]\nname = \"native-bridge\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )?;
        fs::write(rust_crate_dir.join("src/lib.rs"), "")?;

        let loader = compute_default_external_library_loader(
            &rust_crate_dir,
            &dart_root,
            &Config {
                default_external_library_loader_web_prefix: Some("assets/".to_owned()),
                wasm_bindgen_name: Some("custom_wasm".to_owned()),
                ..Default::default()
            },
        );

        assert_eq!(loader.stem, "native_bridge");
        assert_eq!(loader.io_directory, "../native/target/release/");
        assert_eq!(loader.web_prefix, "assets/");
        assert_eq!(loader.wasm_bindgen_name, "custom_wasm");
        Ok(())
    }

    /// Maps each rename to canonical raw and RustAutoOpaqueInner type keys.
    #[test]
    fn dart_type_rename_canonicalizes_raw_and_auto_opaque_types() -> anyhow::Result<()> {
        let rename = compute_dart_type_rename(&Config {
            dart_type_rename: Some(HashMap::from([(
                "crate::Type < u8 >".to_owned(),
                "DartType".to_owned(),
            )])),
            ..Default::default()
        })?;

        assert_eq!(rename.len(), 2);
        assert_eq!(
            rename.get("crate :: Type < u8 >"),
            Some(&"DartType".to_owned())
        );
        assert_eq!(
            rename.get(
                "flutter_rust_bridge :: for_generated :: RustAutoOpaqueInner < crate :: Type < u8 > >"
            ),
            Some(&"DartType".to_owned())
        );
        Ok(())
    }

    /// Propagates invalid Rust syntax from configured Dart type rename keys.
    #[test]
    fn dart_type_rename_rejects_invalid_rust_type() {
        let result = compute_dart_type_rename(&Config {
            dart_type_rename: Some(HashMap::from([("Vec<".to_owned(), "DartType".to_owned())])),
            ..Default::default()
        });

        assert!(result.is_err());
    }
}
