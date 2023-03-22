use std::collections::HashSet;

use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::path::Path;

use anyhow::anyhow;
use itertools::Itertools;
use pathdiff::diff_paths;

use crate::{transformer, Opts};

/// get the raw/shared API module
pub fn mod_from_rust_path(
    config: &Opts,
    all_configs: &[Opts],
    get_shared_mod: bool,
) -> Option<String> {
    fn get_module_name(code_path: &str, crate_path: &str) -> String {
        Path::new(code_path)
            .strip_prefix(Path::new(crate_path).join("src"))
            .unwrap()
            .with_extension("")
            .into_os_string()
            .into_string()
            .unwrap()
            .replace('/', "::")
    }

    let output = match !config.shared {
        true => {
            if !get_shared_mod {
                get_module_name(&config.rust_input_path, &config.rust_crate_dir)
            } else {
                // get the shared module in the regular block
                if all_configs.len() > 1 {
                    get_module_name(&config.shared_rust_output_path, &config.rust_crate_dir)
                } else {
                    "".into()
                }
            }
        }
        false => {
            if !get_shared_mod {
                // TODO: check
                log::debug!(
                    "config.shared_rust_output_path:{}",
                    &config.shared_rust_output_path
                ); //TODO: delete
                get_module_name(&config.shared_rust_output_path, &config.rust_crate_dir)
            } else {
                // There is no extra shared module needed for a shared block
                "".to_owned()
            }
        }
    };

    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}

pub fn with_changed_file<F: FnOnce() -> anyhow::Result<()>>(
    path: &str,
    append_content: &str,
    f: F,
) -> anyhow::Result<()> {
    let content_original = fs::read_to_string(path)?;
    fs::write(path, content_original.clone() + append_content)?;

    f()?;

    Ok(fs::write(path, content_original)?)
}

// https://dart.dev/guides/language/language-tour#keywords
const DART_KEYWORDS: [&str; 63] = [
    "abstract",
    "else",
    "import",
    "show",
    "as",
    "enum",
    "in",
    "static",
    "assert",
    "export",
    "interface",
    "super",
    "async",
    "extends",
    "is",
    "switch",
    "await",
    "extension",
    "late",
    "sync",
    "break",
    "external",
    "library",
    "this",
    "case",
    "factory",
    "mixin",
    "throw",
    "catch",
    "false",
    "new",
    "true",
    "class",
    "final",
    "null",
    "try",
    "const",
    "finally",
    "on",
    "typedef",
    "continue",
    "for",
    "operator",
    "var",
    "covariant",
    "Function",
    "part",
    "void",
    "default",
    "get",
    "required",
    "while",
    "deferred",
    "hide",
    "rethrow",
    "with",
    "do",
    "if",
    "return",
    "yield",
    "dynamic",
    "implements",
    "set",
];

fn check_for_keywords(v: &[String]) -> anyhow::Result<()> {
    if let Some(s) = v.iter().find(|s| DART_KEYWORDS.contains(&s.as_str())) {
        let err_msg = format!("Api name cannot be a dart keyword: {}", s);
        log::error!("{}", err_msg);
        return Err(anyhow!(err_msg));
    };
    Ok(())
}

/// check duplication among regular defined API block(s), and return symbols in tuple of
/// format `(all_no_shared_symbols, all_shared_symbols)`
/// for `all_no_shared_symbols`: if there is duplication among EXPLICITLY defined APIs, it would panic;
/// for `all_shared_symbols`: it would be extended if there is duplication among IMPLICITYLY defined API,
/// which should not exist in `all_no_shared_symbols`; it also means
/// there are at least one API is shared among regualr defined API blocks
pub fn get_symbols_if_no_duplicates(
    regular_configs: &[crate::Opts],
) -> Result<(Vec<String>, Vec<String>), anyhow::Error> {
    let mut explicit_raw_symbols = Vec::new();
    let mut all_symbols = Vec::new();
    for (i, config) in regular_configs.iter().enumerate() {
        log::debug!("for {i}th config:\n"); //TODO: delete
        let ir_file = config.get_ir_file(None)?;
        // for checking explicit API duplication
        //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓TODO: delete test↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
        let iter = ir_file.funcs.iter().map(|f| f.name.clone());
        log::debug!("the ir_file.funcs:{:?}", iter);

        explicit_raw_symbols.extend(iter);

        // for checking implicit API duplication
        let iter = ir_file.get_all_symbols(config, regular_configs);
        log::debug!("raw_ir_file.get_all_symbols:{:?}", iter);
        //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑TODO: delete test↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
        all_symbols.extend(iter);
    }

    // check duplication among explicitly defined API
    let duplicates = explicit_raw_symbols.find_duplicates();
    if !duplicates.is_empty() {
        let duplicated_symbols = duplicates.join(",");

        let (symbol_str, verb_str) = if duplicates.len() == 1 {
            ("symbol", "has")
        } else {
            ("symbols", "have")
        };
        return Err(anyhow!(
            "{} [{}] {} already been defined",
            symbol_str,
            duplicated_symbols,
            verb_str
        ));
    }

    check_for_keywords(&all_symbols)?;

    // check duplication among implicitly defined API
    let (regular_symbols, shared_symbols) = all_symbols.find_uniques_and_duplicates(true, true);
    log::debug!("regular_symbols:{:?}", regular_symbols);
    log::debug!("shared_symbols:{:?}", shared_symbols);

    Ok((regular_symbols, shared_symbols))
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BlockIndex(pub usize);

impl BlockIndex {
    pub const PRIMARY: BlockIndex = BlockIndex(0);
}

impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait PathExt {
    fn file_name_str(&self) -> Option<&str>;
    fn directory_name_str(&self) -> Option<&str>;

    fn get_relative_path_to<P>(&self, path: P, exclude_file: bool) -> String
    where
        P: AsRef<Path>;
}

impl PathExt for std::path::Path {
    #[inline]
    fn file_name_str(&self) -> Option<&str> {
        self.file_name().and_then(std::ffi::OsStr::to_str)
    }
    #[inline]
    fn directory_name_str(&self) -> Option<&str> {
        self.parent().and_then(|p| p.to_str())
    }
    #[inline]
    fn get_relative_path_to<P>(&self, path: P, exclude_file: bool) -> String
    where
        P: AsRef<Path>,
    {
        if exclude_file {
            let src = self.parent().and_then(|p| p.to_str()).unwrap();
            diff_paths(path, src).unwrap().to_str().unwrap().to_owned()
        } else {
            diff_paths(path, self).unwrap().to_str().unwrap().to_owned()
        }
    }
}

pub trait ExtraTraitForVec<T: Clone + Eq + std::hash::Hash> {
    /// Splits the vector into two parts: unique items and duplicate items.
    ///
    /// # Arguments
    ///
    /// * `exclude_duplicates_in_uniques`: If set to `true`, exclude duplicated items from the  returned unique items vector.
    /// * `exclude_duplicates_in_duplicates`: If set to `true`, exclude repeated items from the returned duplicates vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ExtraTraitForVec;
    ///
    /// let vec = vec![1, 2, 3, 2, 4, 5, 4, 6];
    /// let (uniques, duplicates) = vec.split_uniques_and_duplicates(true, true);
    /// assert_eq!(uniques, vec![1, 3, 5, 6]);
    /// assert_eq!(duplicates, vec![2, 4]);
    /// ```
    fn find_uniques_and_duplicates(
        &self,
        exclude_duplicates_in_uniques: bool,
        exclude_duplicates_in_duplicates: bool,
    ) -> (Vec<T>, Vec<T>);

    fn find_duplicates(&self) -> Vec<T>;
}

impl<T: Clone + Eq + std::hash::Hash> ExtraTraitForVec<T> for Vec<T> {
    fn find_uniques_and_duplicates(
        &self,
        exclude_duplicates_in_uniques: bool,
        exclude_duplicates_in_duplicates: bool,
    ) -> (Vec<T>, Vec<T>) {
        let mut uniques = Vec::new();
        let mut duplicates = Vec::new();
        let mut seen = HashSet::new();

        for item in self {
            if !seen.insert(item.clone()) {
                duplicates.push(item.clone());
            } else {
                uniques.push(item.clone());
            }
        }

        if exclude_duplicates_in_uniques {
            uniques.retain(|item| !duplicates.contains(item));
        }

        if exclude_duplicates_in_duplicates {
            let mut seen = HashSet::new();
            duplicates = duplicates
                .iter()
                .filter_map(|x| {
                    if seen.insert(x.clone()) {
                        Some(x.clone())
                    } else {
                        None
                    }
                })
                .collect();
        }
        (uniques, duplicates)
    }

    fn find_duplicates(&self) -> Vec<T> {
        let (_, duplicates) = self.find_uniques_and_duplicates(true, true);
        duplicates
    }
}
