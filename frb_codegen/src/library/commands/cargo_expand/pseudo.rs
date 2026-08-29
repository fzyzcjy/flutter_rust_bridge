use crate::utils::crate_name::CrateName;
use anyhow::ensure;
use itertools::Itertools;
use log::warn;
use std::fs;
use std::path::{Path, PathBuf};

// This is executed because `dart_build_rs`'s `build.rs` will go through this branch
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
pub(super) fn run(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
) -> anyhow::Result<syn::File> {
    warn!(
        "Skip cargo-expand on {rust_crate_dir:?}, \
         because cargo is already running and would block cargo-expand. \
         This might cause errors if your api contains macros or complex mods."
    );

    ensure!(
        interest_crate_name.is_none(),
        "When parsing third party crates, need to use cargo-expand"
    );

    parse_file(&rust_crate_dir.join("src/lib.rs"))
}

fn parse_file(path: &Path) -> anyhow::Result<syn::File> {
    let code = fs::read_to_string(path)?;
    let mut file = syn::parse_file(&code)?;
    modify_file(&mut file, path)?;
    Ok(file)
}

fn modify_file(file: &mut syn::File, path: &Path) -> anyhow::Result<()> {
    for item in file.items.iter_mut() {
        if let syn::Item::Mod(item_mod) = item {
            if item_mod.content.is_none() {
                modify_mod(item_mod, path)?;
            }
        }
    }

    expand_known_macros(&mut file.items)?;
    Ok(())
}

fn expand_known_macros(items: &mut Vec<syn::Item>) -> anyhow::Result<()> {
    let mut output = Vec::with_capacity(items.len());

    for item in std::mem::take(items) {
        match item {
            syn::Item::Macro(item_macro) if is_frb_logging_macro(&item_macro) => {
                output.extend(frb_logging_api_items()?);
            }
            syn::Item::Mod(mut item_mod) => {
                if let Some((_, content)) = &mut item_mod.content {
                    expand_known_macros(content)?;
                }
                output.push(syn::Item::Mod(item_mod));
            }
            item => output.push(item),
        }
    }

    *items = output;
    Ok(())
}

fn is_frb_logging_macro(item: &syn::ItemMacro) -> bool {
    (item.mac.path.segments.last())
        .is_some_and(|segment| segment.ident == "enable_frb_rust_to_dart_logging")
}

fn frb_logging_api_items() -> anyhow::Result<Vec<syn::Item>> {
    Ok(syn::parse_file(
        r##"
pub struct FrbLogRecord {
    pub level: String,
    pub message: String,
    pub target: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[flutter_rust_bridge::frb(init_dart_code = r#"
    kFrbDartLogging.init(
      rustLogStream: frbInternalInitLogger(maxLevel: frbInternalLoggingMaxLevel()),
      mapRecord: (record) => FrbLogRecordData(
        level: record.level,
        message: record.message,
        target: record.target,
        modulePath: record.modulePath,
        file: record.file,
        line: record.line,
      ),
      setupDefaultOutput: frbInternalLoggingSetupDartLoggingOutput(),
      disposeRustLogger: frbInternalDisposeLogger,
    );
"#)]
pub fn frb_internal_init_logger(
    sink: crate::frb_generated::StreamSink<FrbLogRecord>,
    max_level: String,
) {}

#[flutter_rust_bridge::frb(sync)]
pub fn frb_internal_dispose_logger() {}

#[flutter_rust_bridge::frb(sync)]
pub fn frb_internal_logging_max_level() -> String {
    unreachable!()
}

#[flutter_rust_bridge::frb(sync)]
pub fn frb_internal_logging_setup_dart_logging_output() -> bool {
    unreachable!()
}
"##,
    )?
    .items)
}

fn modify_mod(item_mod: &mut syn::ItemMod, path: &Path) -> anyhow::Result<()> {
    ensure!(item_mod.content.is_none() && item_mod.semi.is_some());

    let mod_name = item_mod.ident.to_string();
    if let Some(mod_path) = get_module_file_path(&mod_name, path) {
        let mod_syn_file = parse_file(&mod_path)?;
        item_mod.semi = None;
        item_mod.content = Some((syn::token::Brace::default(), mod_syn_file.items));
    } else {
        log::debug!("Skip parsing {mod_name} since do not know its corresponding file path");
    }

    Ok(())
}

fn get_module_file_path(module_name: &str, parent_module_file_path: &Path) -> Option<PathBuf> {
    let path_candidates = get_module_file_path_candidates(module_name, parent_module_file_path);
    path_candidates.iter().find(|path| path.exists()).cloned()
}
// frb-coverage:ignore-end

fn get_module_file_path_candidates(
    module_name: &str,
    parent_module_file_path: &Path,
) -> Vec<PathBuf> {
    [
        parent_module_file_path.parent().unwrap().to_owned(),
        parent_module_file_path.with_extension(""),
    ]
    .iter()
    .flat_map(|folder_path| {
        [
            folder_path.join(module_name).with_extension("rs"),
            folder_path.join(module_name).join("mod.rs"),
        ]
    })
    .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_module_file_path_candidates_simple() {
        let actual = get_module_file_path_candidates("api", &PathBuf::from("/hello/src/main.rs"));
        let expect = vec![
            PathBuf::from("/hello/src/api.rs"),
            PathBuf::from("/hello/src/api/mod.rs"),
            PathBuf::from("/hello/src/main/api.rs"),
            PathBuf::from("/hello/src/main/api/mod.rs"),
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    /// Verify build-script parsing discovers APIs declared by the logging macro.
    fn test_expand_known_frb_logging_macro_into_api_items() {
        let mut items = syn::parse_file("flutter_rust_bridge::enable_frb_rust_to_dart_logging!();")
            .unwrap()
            .items;

        expand_known_macros(&mut items).unwrap();

        assert_eq!(items.len(), 5);
        assert!(matches!(&items[0], syn::Item::Struct(item) if item.ident == "FrbLogRecord"));
        assert!(
            matches!(&items[1], syn::Item::Fn(item) if item.sig.ident == "frb_internal_init_logger")
        );
        assert!(
            matches!(&items[2], syn::Item::Fn(item) if item.sig.ident == "frb_internal_dispose_logger")
        );
        assert!(
            matches!(&items[3], syn::Item::Fn(item) if item.sig.ident == "frb_internal_logging_max_level")
        );
        assert!(
            matches!(&items[4], syn::Item::Fn(item) if item.sig.ident == "frb_internal_logging_setup_dart_logging_output")
        );
    }
}
