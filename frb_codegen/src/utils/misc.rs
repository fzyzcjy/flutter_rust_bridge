use std::cell::RefCell;

use std::fmt::Display;
use std::fs;

use std::path::Path;

use crate::commands::cargo_expand;
use crate::ir::{IrType, IrTypeTrait};

use crate::utils::consts::DART_KEYWORDS;
use anyhow::anyhow;
use convert_case::{Case, Casing};
use pathdiff::diff_paths;

/// get the raw or shared API module name
pub fn mod_from_rust_path(config: &crate::Opts, get_shared_mod: bool) -> Option<String> {
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

    let output = match config.share_mode {
        ShareMode::Unique => {
            if !get_shared_mod {
                get_module_name(&config.rust_input_path, &config.rust_crate_dir)
            } else {
                // get the shared module in the regular block
                if is_multi_blocks_case(None) {
                    get_module_name(
                        &config.shared_rust_output_path.clone().unwrap(),
                        &config.rust_crate_dir,
                    )
                } else {
                    "".into()
                }
            }
        }
        ShareMode::Shared => {
            // Whatever `get_shared_mod` is, return the shared module name for a shared block
            get_module_name(
                &config.shared_rust_output_path.clone().unwrap(),
                &config.rust_crate_dir,
            )
        }
    };

    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}

thread_local!(pub static IS_MULTI_BLOCKS_CASE: RefCell<Option<bool>> = RefCell::new(None));

pub fn is_multi_blocks_case(all_configs: Option<&[crate::Opts]>) -> bool {
    // if already fetched, get it directly from `IS_MULTI_BLOCKS_CASE`
    if all_configs.is_none() {
        return IS_MULTI_BLOCKS_CASE.with(|data| {
            let v = *data.borrow();
            assert!(v.is_some());
            v.unwrap()
        });
    }

    // fetch from scratch
    let all_configs = all_configs.unwrap();
    let r = match all_configs.len() {
        0 => panic!("there should be at least 1 config"),
        1 => {
            assert_eq!(all_configs[0].share_mode, ShareMode::Unique); // single item must not be shared
            false
        }
        _ => {
            for (i, config) in all_configs.iter().enumerate().take(all_configs.len() - 1) {
                if config.share_mode == ShareMode::Shared {
                    log::error!("Config {i} is shared, but should not be");
                }
            }
            assert_eq!(
                all_configs[all_configs.len() - 1].share_mode,
                ShareMode::Shared
            ); // last item must be shared
            true
        }
    };

    IS_MULTI_BLOCKS_CASE.with(|data| {
        *data.borrow_mut() = Some(r);
    });

    r
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

fn check_for_keywords(v: &[String]) -> anyhow::Result<()> {
    if let Some(s) = v.iter().find(|s| DART_KEYWORDS.contains(&s.as_str())) {
        let err_msg = format!("Api name cannot be a dart keyword: {}", s);
        log::warn!("{}", err_msg);
        return Err(anyhow!(err_msg));
    };
    Ok(())
}

/// check duplication among regular defined API block(s), and return symbols in tuple of
/// format `(all_no_shared_symbols, all_shared_symbols)`
/// for `all_no_shared_symbols`: if there is duplication among EXPLICITLY defined APIs, it would panic;
/// for `all_shared_symbols`: it would be extended if there is duplication among IMPLICITLY defined API,
/// which should not exist in `all_no_shared_symbols`. While it is not empty, it means
/// there are at least one API(symbol) is shared among regualr defined API blocks
pub fn get_symbols_if_no_duplicates(
    regular_configs: &[crate::Opts],
) -> Result<(Vec<String>, Vec<String>), anyhow::Error> {
    let mut explicit_raw_symbols = Vec::new();
    let mut all_symbols = Vec::new();
    for config in regular_configs.iter() {
        let ir_file = config.get_ir_file(&[])?; // all_configs` is empty, no need to care about other configs here

        // for checking explicit API duplication
        let iter = ir_file.funcs(true).into_iter().map(|f| f.name);
        explicit_raw_symbols.extend(iter);
        // for checking implicit API duplication
        let iter = ir_file.get_all_symbols(config);
        all_symbols.extend(iter);
    }
    // check duplication among explicitly defined API
    let duplicates = explicit_raw_symbols.find_duplicates(true);
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

    Ok((regular_symbols, shared_symbols))
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

/// check if all items in `paths` contains the same directory
pub fn is_same_directory(paths: &[String]) -> bool {
    let paths = paths
        .iter()
        .map(|path| Path::new(&path).get_directory_name().to_owned())
        .collect::<Vec<_>>();
    paths.iter().all(|item| item == &paths[0])
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, serde::Serialize)]
pub struct BlockIndex(pub usize);

impl BlockIndex {
    pub const PRIMARY: BlockIndex = BlockIndex(0);
}

impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, serde::Serialize, Hash)]
pub enum ShareMode {
    Unique, // for (the types only for) regular/no-shared block
    Shared, // for (the types only for) shared block
}

pub trait PathExt {
    fn file_name_str(&self) -> Option<&str>;
    fn get_file_name(&self) -> &str;
    fn directory_name_str(&self) -> Option<&str>;
    fn get_directory_name(&self) -> &str;
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
    fn get_file_name(&self) -> &str {
        self.file_name_str().unwrap_or_default()
    }
    #[inline]
    fn directory_name_str(&self) -> Option<&str> {
        self.parent().and_then(|p| p.to_str())
    }
    #[inline]
    fn get_directory_name(&self) -> &str {
        self.directory_name_str().unwrap_or_default()
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

pub(crate) trait ExtraTraitForVec<T: Clone + Eq + std::hash::Hash> {
    /// This function finds unique and duplicate elements in a vector within original order.
    ///
    /// # Arguments
    ///
    /// * `exclude_duplicates_in_uniques` - A boolean that indicates whether to exclude duplicate elements in the returned unique_list.
    /// * `exclude_duplicates_in_duplicates` - A boolean that indicates whether to exclude duplicate elements in the returned duplicate_list.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crate::utils::misc::ExtraTraitForVec;
    ///
    /// let vec = vec![1, 2, 3, 4, 5, 2, 1, 3];
    /// // Case 1: exclude duplicates in both uniques and duplicates
    /// let (uniques, duplicates) = vec.find_uniques_and_duplicates(true, true);
    /// assert_eq!(uniques, vec![4, 5]);
    /// assert_eq!(duplicates, vec![1, 2, 3]);
    /// // Case 2: only exclude duplicates in uniques
    /// let (uniques, duplicates) = vec.find_uniques_and_duplicates(true, false);
    /// assert_eq!(uniques, vec![4, 5]);
    /// assert_eq!(duplicates, vec![1, 2, 3, 2, 1, 3]);
    /// // Case 3: only exclude duplicates in duplicates
    /// let (uniques, duplicates) = vec.find_uniques_and_duplicates(false, true);
    /// assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
    /// assert_eq!(duplicates, vec![1, 2, 3]);
    /// // Case 4: include duplicates in both uniques and duplicates
    /// let (uniques, duplicates) = vec.find_uniques_and_duplicates(false, false);
    /// assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
    /// assert_eq!(duplicates, vec![1, 2, 3, 2, 1, 3]);
    /// ```
    fn find_uniques_and_duplicates(
        &self,
        exclude_duplicates_in_uniques: bool,
        exclude_duplicates_in_duplicates: bool,
    ) -> (Vec<T>, Vec<T>);

    /// This function finds unique elements in a vector within original order.
    /// # Arguments
    /// * `exclude_duplicates` - A boolean that indicates whether to exclude duplicate elements in the returned unique_list.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crate::utils::misc::ExtraTraitForVec;
    ///
    /// let vec = vec![1, 2, 3, 4, 5, 2, 1, 3];
    ///
    /// let uniques = vec.find_uniques(true);
    /// assert_eq!(uniques, vec![4, 5]);
    ///
    /// let uniques = vec.find_uniques(false);
    /// assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
    /// ```
    fn find_uniques(&self, exclude_duplicates: bool) -> Vec<T>;

    /// This function finds duplicate elements in a vector within original order.
    /// # Arguments
    /// * `exclude_duplicates` - A boolean that indicates whether to exclude duplicate elements in the returned duplicate_list.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crate::utils::misc::ExtraTraitForVec;
    /// 
    /// let vec = vec![1, 2, 3, 4, 5, 2, 1, 3];
    ///
    /// let duplicates = vec.find_duplicates(true);
    /// assert_eq!(duplicates, vec![1, 2, 3]);
    ///
    /// let duplicates = vec.find_duplicates(false);
    /// assert_eq!(duplicates, vec![1, 2, 3, 2, 1, 3]);
    /// ```
    fn find_duplicates(&self, exclude_duplicates: bool) -> Vec<T>;
}

impl<T: Clone + Eq + std::hash::Hash> ExtraTraitForVec<T> for Vec<T> {
    fn find_uniques_and_duplicates(
        &self,
        exclude_duplicates_in_uniques: bool,
        exclude_duplicates_in_duplicates: bool,
    ) -> (Vec<T>, Vec<T>) {
        let mut uniques = vec![];
        let mut duplicates = vec![];
        for item in self {
            let count = self.iter().filter(|&n| n == item).count();
            let is_unique = count == 1;
            if is_unique {
                if !uniques.contains(item) || !exclude_duplicates_in_uniques {
                    uniques.push(item.clone());
                }
            } else {
                if !uniques.contains(item) && !exclude_duplicates_in_uniques {
                    uniques.push(item.clone());
                }
                if !duplicates.contains(item) || !exclude_duplicates_in_duplicates {
                    duplicates.push(item.clone());
                }
            }
        }
        (uniques, duplicates)
    }

    fn find_uniques(&self, exclude_duplicates: bool) -> Vec<T> {
        let (uniques, _) = self.find_uniques_and_duplicates(exclude_duplicates, false);
        uniques
    }

    fn find_duplicates(&self, exclude_duplicates: bool) -> Vec<T> {
        let (_, duplicates) = self.find_uniques_and_duplicates(true, exclude_duplicates);
        duplicates
    }
}

/// ouput `IrType`s with no duplicated safe_ident
pub fn get_deduplicate_type(types: &[IrType]) -> Vec<IrType> {
    let mut types_clone = types.to_vec();
    types_clone.sort_by_key(|ty| ty.safe_ident());
    types_clone.dedup_by_key(|ty| ty.safe_ident());
    types_clone
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

    use super::ExtraTraitForVec;
    #[test]
    fn test_find_uniques_and_duplicates() {
        let vec = vec![1, 2, 3, 4, 5, 2, 1, 3];

        // Case 1: exclude duplicates in both uniques and duplicates
        let (uniques, duplicates) = vec.find_uniques_and_duplicates(true, true);
        assert_eq!(uniques, vec![4, 5]);
        assert_eq!(duplicates, vec![1, 2, 3]);

        // Case 2: only exclude duplicates in uniques
        let (uniques, duplicates) = vec.find_uniques_and_duplicates(true, false);
        assert_eq!(uniques, vec![4, 5]);
        assert_eq!(duplicates, vec![1, 2, 3, 2, 1, 3]);

        // Case 3: only exclude duplicates in duplicates
        let (uniques, duplicates) = vec.find_uniques_and_duplicates(false, true);
        assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
        assert_eq!(duplicates, vec![1, 2, 3]);

        // Case 4: include duplicates in both uniques and duplicates
        let (uniques, duplicates) = vec.find_uniques_and_duplicates(false, false);
        assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
        assert_eq!(duplicates, vec![1, 2, 3, 2, 1, 3]);
    }

    #[test]
    fn test_find_uniques() {
        let vec = vec![1, 2, 3, 4, 5, 2, 1, 3];

        let uniques = vec.find_uniques(true);
        assert_eq!(uniques, vec![4, 5]);

        let uniques = vec.find_uniques(false);
        assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_find_duplicates() {
        let vec = vec![1, 2, 3, 4, 5, 2, 1, 3];

        let duplicates = vec.find_duplicates(true);
        assert_eq!(duplicates, vec![1, 2, 3]);

        let duplicates = vec.find_duplicates(false);
        assert_eq!(duplicates, vec![1, 2, 3, 2, 1, 3]);
    }
}
