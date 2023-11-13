use std::collections::HashSet;

use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::path::Path;

use crate::commands::cargo_expand;
use crate::utils::consts::DART_KEYWORDS;
use anyhow::anyhow;
use convert_case::{Case, Casing};
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

pub fn with_changed_file<F: FnOnce() -> crate::Result<()>>(
    path: &str,
    append_content: &str,
    f: F,
) -> crate::Result<()> {
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

fn check_for_keywords(v: &[String]) -> anyhow::Result<()> {
    if let Some(s) = v.iter().find(|s| DART_KEYWORDS.contains(&s.as_str())) {
        return Err(anyhow!("Api name cannot be a dart keyword: {}", s));
    };
    Ok(())
}

/// check api defined by users, if no duplicates, then generate all symbols (api function name),
/// including those generated implicitly by frb
pub fn get_symbols_if_no_duplicates(configs: &[crate::Opts]) -> Result<Vec<String>, anyhow::Error> {
    let mut explicit_raw_symbols = Vec::new();
    let mut all_symbols = Vec::new();
    for config in configs {
        let raw_ir_file = config.get_ir_file()?;

        // for checking explicit api duplication
        explicit_raw_symbols.extend(raw_ir_file.funcs.iter().map(|f| f.name.clone()));

        // for avoiding redundant generation in dart
        all_symbols.extend(raw_ir_file.get_all_symbols(config));
    }
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

    check_for_keywords(&all_symbols)?;

    Ok(all_symbols)
}

/// If the given string is a Dart keyword, then
/// convert it to PascalCase to avoid issues.
/// If the string is not a keyword, then the original
/// is returned.
pub fn make_string_keyword_safe(input: String) -> String {
    if check_for_keywords(&[input.clone()]).is_err() {
        input.to_case(Case::Pascal)
    } else {
        input
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, serde::Serialize)]
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

/// For structs that only has an `inner` serializable property that
/// would be better (de)serialized as a newtype.
#[macro_export]
macro_rules! derive_serde_inner_as_newtype {
    ($($type:ident),*) => {$(
        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $type {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                s.serialize_newtype_struct(::std::stringify!($type), self.inner.as_ref())
            }
        }
    )*};
}

/// Adds some common derives for IR types.
///
/// Valid forms:
/// - `ir! { pub struct Foo { .. } .. }`
/// - `ir! { #[no_serde] pub struct Bar { .. } .. }`
#[macro_export]
macro_rules! ir {
    () => {};
    (#[no_serde] $decl:item $($rest:tt)*) => {
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        $decl

        $crate::ir!($($rest)*);
    };
    ($decl:item $($rest:tt)*) => {
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
        $decl

        $crate::ir!($($rest)*);
    }
}

pub fn read_rust_file(path: &Path) -> String {
    let path = path.to_str().unwrap();
    let (dir, module) = get_dir_and_mod(path);
    cargo_expand(&dir, module, path)
}

fn get_dir_and_mod(path: &str) -> (String, Option<String>) {
    const SRC: &str = "/src/";
    #[cfg(windows)]
    let path = &path.replace('\\', "/");
    let src_index = path.rfind(SRC).expect("src dir must exist in rust project");
    let dir = &path[..src_index];
    #[cfg(windows)]
    let dir = dir.strip_prefix("//?/").unwrap_or(dir);
    let module = &path[src_index + SRC.len()..];
    let module = module.strip_suffix("mod.rs").unwrap_or(module);
    let module = match module {
        "lib.rs" => None,
        "" => None,
        _ => {
            let module = module.replace('/', "::");
            Some(
                module
                    .strip_suffix(".rs")
                    .map(String::from)
                    .unwrap_or(module),
            )
        }
    };
    (String::from(dir), module)
}

pub fn dart_maybe_implements_exception(is_exception: bool) -> &'static str {
    if is_exception {
        "implements FrbException"
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn simple_mod() {
        let (dir, module) = get_dir_and_mod("/project/src/api.rs");
        assert_eq!("/project", dir);
        assert_eq!(Some(String::from("api")), module);
    }

    #[test]
    pub fn sub_mod() {
        let (dir, module) = get_dir_and_mod("/project/src/sub/subsub.rs");
        assert_eq!("/project", dir);
        assert_eq!(Some(String::from("sub::subsub")), module);
    }
}
