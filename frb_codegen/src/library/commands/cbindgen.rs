use anyhow::bail;
use log::debug;
use std::fs;
use std::path::Path;

pub(crate) struct CbindgenArgs<'a> {
    pub rust_crate_dir: &'a Path,
    pub c_struct_names: Vec<String>,
    pub exclude_symbols: Vec<String>,
}

pub(crate) fn cbindgen(args: CbindgenArgs) -> anyhow::Result<String> {
    let temp_c_output_file = tempfile::Builder::new().suffix(".h").tempfile()?;

    cbindgen_to_file(args, temp_c_output_file.path())?;
    let output_text = fs::read_to_string(temp_c_output_file.as_file())?;

    drop(temp_c_output_file); // do not drop too early

    Ok(output_text)
}

fn cbindgen_to_file(args: CbindgenArgs, c_output_path: &Path) -> anyhow::Result<()> {
    debug!("execute cbindgen rust_crate_dir={rust_crate_dir:?} c_output_path={c_output_path:?}",);

    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        sys_includes: vec![
            "stdbool.h".to_string(),
            "stdint.h".to_string(),
            "stdlib.h".to_string(),
        ],
        no_includes: true,
        // copied from: dart-sdk/dart_api.h
        // used to convert Dart_Handle to Object.
        after_includes: Some("typedef struct _Dart_Handle* Dart_Handle;".to_owned()),
        export: cbindgen::ExportConfig {
            include: args
                .c_struct_names
                .iter()
                .map(|name| format!("\"{name}\""))
                .collect(),
            exclude: args.exclude_symbols,
            ..Default::default()
        },
        ..Default::default()
    };
    debug!("cbindgen config: {:#?}", config);

    let parsed_crate_dir = parse_crate_dir(args.rust_crate_dir)?;
    debug!("cbindgen parsed_crate_dir={}", parsed_crate_dir);

    let bindings = cbindgen::generate_with_config(parsed_crate_dir, config)?;

    if bindings.write_to_file(c_output_path) {
        Ok(())
    } else {
        bail!("cbindgen failed writing file")
    }
}

fn parse_crate_dir(rust_crate_dir: &Path) -> anyhow::Result<String> {
    let canonical_path = Path::new(rust_crate_dir).canonicalize()?;
    let mut path = canonical_path.to_str().unwrap();

    // on windows get rid of the UNC path
    if path.starts_with(r"\\?\") {
        path = &path[r"\\?\".len()..];
    }

    Ok(path.to_owned())
}
