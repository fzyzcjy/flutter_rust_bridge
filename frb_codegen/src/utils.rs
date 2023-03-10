use std::collections::HashSet;

use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::path::Path;

use anyhow::anyhow;
use itertools::Itertools;
use pathdiff::diff_paths;

pub fn mod_from_rust_path(code_path: &str, crate_path: &str) -> String {
    Path::new(code_path)
        .strip_prefix(Path::new(crate_path).join("src"))
        .unwrap()
        .with_extension("")
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('/', "::")
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

pub fn find_all_duplicates<T>(iter: &[T]) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut uniq = HashSet::new();
    iter.iter()
        .filter(|x| !uniq.insert(*x))
        .cloned()
        .collect::<Vec<_>>()
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
        return Err(anyhow!("Api name cannot be a dart keyword: {}", s));
    };
    Ok(())
}

/// check api defined by users, if no duplicates, then generate all symbols (api function name),
/// including those generated implicitly by frb
pub fn get_symbols_if_no_duplicates(
    configs: &[crate::Opts],
) -> Result<(Vec<String>, Vec<String>), anyhow::Error> {
    let mut explicit_raw_symbols = Vec::new();
    let mut all_symbols = Vec::new();
    for config in configs {
        let raw_ir_file = config.get_ir_file()?;

        // for checking explicit API duplication
        let iter = raw_ir_file.funcs.iter().map(|f| f.name.clone());
        log::debug!(" raw_ir_file.funcs:{:?}", iter);
        explicit_raw_symbols.extend(iter);

        // for checking implicit API duplication
        let iter = raw_ir_file.get_all_symbols(config);
        log::debug!("raw_ir_file.get_all_symbols:{:?}", iter);
        all_symbols.extend(iter);
    }

    // check duplication among explicitly defined API
    let duplicates = find_all_duplicates(&explicit_raw_symbols);
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

    // check duplication among implicitly defined API
    check_for_keywords(&all_symbols)?; //TODO: what is this used for?
    // TODO: `free_WireSyncReturn` should not only existed in the 1st block.
    let (regular_symbols, shared_symbols) = all_symbols.split_uniques_and_duplicates(true, true);
    log::debug!("shared_symbols:{:?}", shared_symbols);
    log::debug!("regular_symbols:{:?}", regular_symbols);

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
    /// * `exclude_duplicates_in_uniques`: If set to `true`, exclude duplicated items from the unique items vector.
    /// * `exclude_duplicates_in_duplicates`: If set to `true`, exclude repeated items from the duplicates vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_flutter_rust_bridge_codegen::ExtraTraitForVec;
    ///
    /// let vec = vec![1, 2, 3, 2, 4, 5, 4, 6];
    /// let (uniques, duplicates) = vec.split_uniques_and_duplicates(true, true);
    /// assert_eq!(uniques, vec![1, 3, 5, 6]);
    /// assert_eq!(duplicates, vec![2, 4]);
    /// ```
    fn split_uniques_and_duplicates(
        &self,
        exclude_duplicates_in_uniques: bool,
        exclude_duplicates_in_duplicates: bool,
    ) -> (Vec<T>, Vec<T>);
}

impl<T: Clone + Eq + std::hash::Hash> ExtraTraitForVec<T> for Vec<T> {
    fn split_uniques_and_duplicates(
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
}
