use crate::utils::path_utils::normalize_windows_unc_path;
use cbindgen::Error;
use log::{debug, info};
use std::fs;
use std::path::Path;

pub(crate) struct CbindgenArgs<'a> {
    pub rust_crate_dir: &'a Path,
    pub c_struct_names: Vec<String>,
    pub exclude_symbols: Vec<String>,
    pub after_includes: String,
}

pub(crate) fn cbindgen(args: CbindgenArgs) -> anyhow::Result<String> {
    let temp_c_output_file = tempfile::Builder::new().suffix(".h").tempfile()?;

    cbindgen_to_file(args, temp_c_output_file.path())?;
    let output_text = fs::read_to_string(temp_c_output_file.path())?;

    drop(temp_c_output_file); // do not drop too early

    Ok(output_text)
}

fn cbindgen_to_file(args: CbindgenArgs, c_output_path: &Path) -> anyhow::Result<()> {
    debug!(
        // weirdly this line is not covered, while the `debug!` call is
        // frb-coverage:ignore-start
        "execute cbindgen rust_crate_dir={rust_crate_dir:?} c_output_path={c_output_path:?}",
        // frb-coverage:ignore-end
        rust_crate_dir = args.rust_crate_dir
    );

    let default_cbindgen_config = default_cbindgen_config();
    let config = cbindgen::Config {
        export: cbindgen::ExportConfig {
            include: args.c_struct_names,
            exclude: args.exclude_symbols,
            ..Default::default()
        },
        after_includes: Some(
            args.after_includes + &default_cbindgen_config.after_includes.unwrap(),
        ),
        ..default_cbindgen_config
    };
    debug!("cbindgen config: {:#?}", config);

    cbindgen_raw(config, args.rust_crate_dir, c_output_path)
}

pub(crate) fn default_cbindgen_config() -> cbindgen::Config {
    cbindgen::Config {
        // copied from: dart-sdk/dart_api.h
        // used to convert Dart_Handle to Object.
        after_includes: Some("typedef struct _Dart_Handle* Dart_Handle;".to_owned()),
        language: cbindgen::Language::C,
        sys_includes: vec![
            "stdbool.h".to_string(),
            "stdint.h".to_string(),
            "stdlib.h".to_string(),
        ],
        no_includes: true,
        ..Default::default()
    }
}

pub(crate) fn cbindgen_raw(
    config: cbindgen::Config,
    rust_crate_dir: &Path,
    c_output_path: &Path,
) -> anyhow::Result<()> {
    let parsed_crate_dir = parse_crate_dir(rust_crate_dir)?;
    debug!("cbindgen parsed_crate_dir={}", parsed_crate_dir);

    #[allow(clippy::manual_inspect)]
    let bindings = cbindgen::generate_with_config(parsed_crate_dir, config).map_err(|e| {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        if let Error::ParseSyntaxError { src_path, .. } = &e {
            let content =
                fs::read_to_string(src_path).unwrap_or_else(|_| "CANNOT READ FILE".into());
            info!("More information: src_path={src_path:?} content={content}");
        }
        e
        // frb-coverage:ignore-end
    })?;

    // no need to worry about return value. false just means content not change
    bindings.write_to_file(c_output_path);

    Ok(())
}

fn parse_crate_dir(rust_crate_dir: &Path) -> anyhow::Result<String> {
    let canonical_path = Path::new(rust_crate_dir).canonicalize()?;
    Ok(normalize_windows_unc_path(canonical_path.to_str().unwrap()).to_owned())
}

#[cfg(test)]
mod tests {
    use super::{
        cbindgen, default_cbindgen_config, normalize_windows_unc_path, parse_crate_dir,
        CbindgenArgs,
    };
    use anyhow::Result;
    use std::fs;
    use tempfile::tempdir;

    /// Uses the C header defaults required by Dart's native API.
    #[test]
    fn test_default_cbindgen_config() {
        let config = default_cbindgen_config();

        assert_eq!(config.language, cbindgen::Language::C);
        assert_eq!(config.sys_includes, ["stdbool.h", "stdint.h", "stdlib.h"]);
        assert!(config.no_includes);
        assert_eq!(
            config.after_includes.as_deref(),
            Some("typedef struct _Dart_Handle* Dart_Handle;")
        );
    }

    /// Canonicalizes crate directories before passing them to cbindgen.
    #[test]
    fn test_parse_crate_dir_normalizes_existing_path() -> Result<()> {
        let temp_dir = tempdir()?;
        let crate_dir = temp_dir.path().join("crate");
        fs::create_dir(&crate_dir)?;

        assert_eq!(
            parse_crate_dir(&crate_dir)?,
            normalize_windows_unc_path(&crate_dir.canonicalize()?.to_string_lossy())
        );
        Ok(())
    }

    /// Generates a header from a minimal local crate with Cargo available for metadata lookup.
    #[test]
    fn test_cbindgen_generates_header_for_minimal_crate() -> Result<()> {
        let temp_dir = tempdir()?;
        let crate_dir = temp_dir.path();
        fs::create_dir(crate_dir.join("src"))?;
        fs::write(
            crate_dir.join("Cargo.toml"),
            "[package]\nname = \"cbindgen_test\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )?;
        fs::write(
            crate_dir.join("src/lib.rs"),
            "#[no_mangle]\npub extern \"C\" fn answer() -> i32 { 42 }\n",
        )?;

        let output = cbindgen(CbindgenArgs {
            rust_crate_dir: crate_dir,
            c_struct_names: Vec::new(),
            exclude_symbols: Vec::new(),
            after_includes: "#include <dart_api.h>\n".to_owned(),
        })?;

        assert!(output.contains("#include <dart_api.h>"));
        assert!(output.contains("typedef struct _Dart_Handle* Dart_Handle;"));
        assert!(output.contains("int32_t answer(void);"));
        Ok(())
    }
}
