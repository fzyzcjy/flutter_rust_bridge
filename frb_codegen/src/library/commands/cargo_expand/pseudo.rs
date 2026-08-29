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
                ensure!(
                    item_macro.attrs.is_empty(),
                    "Attributes on enable_frb_rust_to_dart_logging are unsupported during build.rs code generation"
                );
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
    let segments = &item.mac.path.segments;
    segments.len() == 2
        && segments[0].ident == "flutter_rust_bridge"
        && segments[1].ident == "enable_frb_rust_to_dart_logging"
}

fn frb_logging_api_items() -> anyhow::Result<Vec<syn::Item>> {
    Ok(syn::parse_file(
        r##"
#[doc(hidden)]
#[flutter_rust_bridge::frb(internal_logging, init_dart_code = r#"
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

#[doc(hidden)]
#[flutter_rust_bridge::frb(internal_logging, sync)]
pub fn frb_internal_dispose_logger() {}

#[doc(hidden)]
#[flutter_rust_bridge::frb(internal_logging, sync)]
pub fn frb_internal_logging_max_level() -> String {
    unreachable!()
}

#[doc(hidden)]
#[flutter_rust_bridge::frb(internal_logging, sync)]
pub fn frb_internal_logging_setup_dart_logging_output() -> bool {
    unreachable!()
}

#[derive(Clone, Debug)]
pub struct FrbLogRecord {
    pub level: String,
    pub message: String,
    pub target: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

impl Clone for FrbLogRecord {
    fn clone(&self) -> Self {
        unreachable!()
    }
}

impl std::fmt::Debug for FrbLogRecord {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

type FrbLogSink = crate::frb_generated::StreamSink<FrbLogRecord>;

struct FrbDartLogger {
    sink: std::sync::RwLock<Option<std::sync::Arc<FrbLogSink>>>,
}

impl log::Log for FrbDartLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        unreachable!()
    }

    fn log(&self, record: &log::Record) {}

    fn flush(&self) {}
}

impl FrbDartLogger {
    fn load_sink(&self) -> Option<std::sync::Arc<FrbLogSink>> {
        unreachable!()
    }

    fn swap_sink(&self, sink: Option<FrbLogSink>) {}
}

fn frb_log_record_to_console(record: &log::Record) {}

static FRB_DART_LOGGER: std::sync::OnceLock<FrbDartLogger> = std::sync::OnceLock::new();

fn frb_parse_logging_max_level(max_level: &str) -> log::LevelFilter {
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
    use proc_macro2::{Delimiter, TokenTree};
    use quote::quote;
    use quote::ToTokens;
    use std::fs;
    use syn::parse_quote;
    use tempfile::TempDir;

    fn write_crate_file(temp_dir: &TempDir, relative_path: &str, content: &str) {
        let path = temp_dir.path().join(relative_path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn parsed_tokens(temp_dir: &TempDir) -> String {
        run(temp_dir.path(), None)
            .unwrap()
            .into_token_stream()
            .to_string()
    }

    /// Lists file candidates in Rust's module resolution order.
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

        assert_eq!(items.len(), 14);
        assert!(
            matches!(&items[0], syn::Item::Fn(item) if item.sig.ident == "frb_internal_init_logger")
        );
        assert!(
            matches!(&items[1], syn::Item::Fn(item) if item.sig.ident == "frb_internal_dispose_logger")
        );
        assert!(
            matches!(&items[2], syn::Item::Fn(item) if item.sig.ident == "frb_internal_logging_max_level")
        );
        assert!(
            matches!(&items[3], syn::Item::Fn(item) if item.sig.ident == "frb_internal_logging_setup_dart_logging_output")
        );
        assert!(matches!(&items[4], syn::Item::Struct(item) if item.ident == "FrbLogRecord"));
    }

    #[test]
    /// Verify pseudo logging declarations stay aligned with the canonical macro.
    fn test_frb_logging_api_items_match_canonical_macro() {
        let canonical = canonical_frb_logging_api_items();
        let pseudo = frb_logging_api_items().unwrap();

        assert_eq!(project_items(pseudo), project_items(canonical));
    }

    #[test]
    /// Verify pseudo expansion leaves an unrelated same-named macro unchanged.
    fn test_expand_known_frb_logging_macro_ignores_other_crates() {
        let mut items = syn::parse_file("other::enable_frb_rust_to_dart_logging!();")
            .unwrap()
            .items;

        expand_known_macros(&mut items).unwrap();

        assert!(
            matches!(&items[0], syn::Item::Macro(item) if item.mac.path.segments[0].ident == "other")
        );
    }

    #[test]
    /// Verify conditional logging macros fail instead of producing mismatched bindings.
    fn test_expand_known_frb_logging_macro_rejects_attributes() {
        let mut items = syn::parse_file(
            "#[cfg(feature = \"logging\")] flutter_rust_bridge::enable_frb_rust_to_dart_logging!();",
        )
        .unwrap()
        .items;

        assert!(expand_known_macros(&mut items).is_err());
    }

    fn canonical_frb_logging_api_items() -> Vec<syn::Item> {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../frb_rust/src/misc/frb_logging.rs");
        let file = syn::parse_file(&fs::read_to_string(path).unwrap()).unwrap();
        let item_macro = file
            .items
            .into_iter()
            .find_map(|item| match item {
                syn::Item::Macro(item) if item.mac.path.is_ident("macro_rules") => Some(item),
                _ => None,
            })
            .unwrap();
        let final_arm = item_macro
            .mac
            .tokens
            .into_iter()
            .filter_map(|token| match token {
                TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => Some(group),
                _ => None,
            })
            .collect_vec()
            .pop()
            .unwrap();
        let source = final_arm
            .stream()
            .to_string()
            .replace("$ crate", "flutter_rust_bridge")
            .replace("$ max_level", "log::LevelFilter::Info")
            .replace("$ setup_dart_logging_output", "true");

        syn::parse_file(&source).unwrap().items
    }

    fn project_items(items: Vec<syn::Item>) -> Vec<String> {
        let mut output = items.into_iter().filter_map(project_item).collect_vec();
        output.sort();
        output
    }

    fn project_item(mut item: syn::Item) -> Option<String> {
        match &mut item {
            syn::Item::Fn(item) => item.block = Box::new(parse_quote!({})),
            syn::Item::Impl(item) if is_derived_impl(item) => return None,
            syn::Item::Impl(item) => {
                for impl_item in &mut item.items {
                    if let syn::ImplItem::Fn(function) = impl_item {
                        function.block = parse_quote!({});
                    }
                }
            }
            syn::Item::Static(item) => item.expr = Box::new(parse_quote!(())),
            _ => {}
        }

        Some(quote!(#item).to_string().split_whitespace().join(" "))
    }

    fn is_derived_impl(item: &syn::ItemImpl) -> bool {
        item.trait_
            .as_ref()
            .and_then(|(_, path, _)| path.segments.last())
            .is_some_and(|segment| segment.ident == "Clone" || segment.ident == "Debug")
    }

    /// Expands an external module stored in the sibling Rust source file.
    #[test]
    fn test_run_expands_external_module_from_file() {
        let temp_dir = TempDir::new().unwrap();
        write_crate_file(&temp_dir, "src/lib.rs", "mod foo;");
        write_crate_file(&temp_dir, "src/foo.rs", "pub struct Foo;");

        assert_eq!(parsed_tokens(&temp_dir), "mod foo { pub struct Foo ; }");
    }

    /// Expands an external module stored in the conventional mod.rs file.
    #[test]
    fn test_run_expands_external_module_from_mod_rs() {
        let temp_dir = TempDir::new().unwrap();
        write_crate_file(&temp_dir, "src/lib.rs", "mod foo;");
        write_crate_file(&temp_dir, "src/foo/mod.rs", "pub struct Foo;");

        assert_eq!(parsed_tokens(&temp_dir), "mod foo { pub struct Foo ; }");
    }

    /// Leaves an external module untouched when its source file is absent.
    #[test]
    fn test_run_leaves_missing_external_module_unchanged() {
        let temp_dir = TempDir::new().unwrap();
        write_crate_file(&temp_dir, "src/lib.rs", "mod missing;");

        assert_eq!(parsed_tokens(&temp_dir), "mod missing ;");
    }

    /// Returns the parser error when an external module contains malformed Rust.
    #[test]
    fn test_run_returns_error_for_malformed_external_module() {
        let temp_dir = TempDir::new().unwrap();
        write_crate_file(&temp_dir, "src/lib.rs", "mod foo;");
        write_crate_file(&temp_dir, "src/foo.rs", "pub struct Foo {");

        assert!(run(temp_dir.path(), None).is_err());
    }
}
