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
    Ok(())
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
    use quote::ToTokens;
    use std::fs;
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

    /// Recursively expands nested external modules.
    #[test]
    fn test_run_recursively_expands_nested_external_modules() {
        let temp_dir = TempDir::new().unwrap();
        write_crate_file(&temp_dir, "src/lib.rs", "mod foo;");
        write_crate_file(&temp_dir, "src/foo.rs", "mod nested; pub struct Foo;");
        write_crate_file(&temp_dir, "src/nested.rs", "pub struct Decoy;");
        write_crate_file(&temp_dir, "src/foo/nested.rs", "pub struct Nested;");

        assert_eq!(
            parsed_tokens(&temp_dir),
            "mod foo { mod nested { pub struct Nested ; } pub struct Foo ; }"
        );
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
