use std::fmt::Display;
use std::fs;

use quote::ToTokens;
use std::path::Path;
use syn::{parse_file, Item, Type};

use crate::commands::cargo_expand;
use crate::ir::{IrType, IrTypeTrait};

use crate::utils::consts::DART_KEYWORDS;
use anyhow::anyhow;
use convert_case::{Case, Casing};
use pathdiff::diff_paths;

use extend::ext;

/// TODO: need?Given a path to a Rust code file and the path to the crate directory, returns the Rust module name of the file.
///
/// # Arguments
///
/// * `code_path` - A string slice that holds the path to the Rust code file.
/// * `crate_path` - A string slice that holds the path to the Rust crate directory.
///
/// # Panics
///
/// If the combined path of `crate_path` and `code_path` is not a valid Rust file, the function will panic.
///
/// # Returns
///
/// A string that holds the Rust module name of the file.
// pub fn get_rust_module_name(code_path: &str, crate_path: &str) -> String {
//     let src_path = Path::new(crate_path).join("src");
//     let full_path = Path::new(code_path);

//     let relative_path = diff_paths(full_path, &src_path)
//         .unwrap_or_else(|| panic!("Failed to get relative path for code file: {}", code_path));

//     relative_path
//         .with_extension("")
//         .to_string_lossy()
//         .replace("/", "::")
// }

/// TODO: delete? Can the code filter out empty path?
/// Get the rust module name from the given code path.
/// If the combined path of `crate_path` and `code_path` is not avalid rust file, panic.
pub fn get_rust_module_name(crate_path: &str, code_path: &str) -> String {
    Path::new(code_path)
        .strip_prefix(Path::new(crate_path).join("src"))
        .unwrap_or_else(|_| panic!("`strip_prefix` went wrong with the path:{:?}", code_path))
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

pub fn check_func_dart_keywords(func_name: &str) -> anyhow::Result<()> {
    if DART_KEYWORDS.contains(&func_name) {
        let err_msg = format!("Api name cannot be a dart keyword: {func_name}");
        log::warn!("{}", err_msg);
        return Err(anyhow!(err_msg));
    };
    Ok(())
}

/// If the given string is a Dart keyword, then
/// convert it to PascalCase to avoid issues.
/// If the string is not a keyword, then the original
/// is returned.
pub fn make_string_keyword_safe(input: String) -> String {
    if check_func_dart_keywords(&input).is_err() {
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

/// This struct is used to store the index for a block---A single IrFIle/Opt instance can be treated as a block
/// Specifically, for a regualr block,it is indexed started from 0;
/// While for a shared block, it is indexed with `None`.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, serde::Serialize)]
pub struct BlockIndex(pub Option<usize>);
impl BlockIndex {
    pub const PRIMARY: BlockIndex = BlockIndex(Some(0));

    pub(crate) fn new_shared() -> BlockIndex {
        BlockIndex(None)
    }

    pub(crate) fn shared() -> BlockIndex {
        BlockIndex(None)
    }
}
impl BlockIndex {
    pub fn unwrap(&self) -> usize {
        self.0.unwrap()
    }

    pub fn is_shared(&self) -> bool {
        self.0.is_none()
    }
}
impl PartialEq<usize> for BlockIndex {
    fn eq(&self, other: &usize) -> bool {
        if self.0.is_none() {
            return false;
        }
        self.0 == Some(*other)
    }
}
impl PartialEq<Option<usize>> for BlockIndex {
    fn eq(&self, other: &Option<usize>) -> bool {
        self.0 == *other
    }
}
impl PartialEq<BlockIndex> for Option<usize> {
    fn eq(&self, other: &BlockIndex) -> bool {
        *self == *other
    }
}
impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, serde::Serialize, Hash)]
pub enum IrTypeUseRange {
    // TODO: delete
    //     UniqueBlockAsInput, // for types used ONLY in one regular Api block, and ONLY in context of function inputs
    //     UniqueBlockAsOutput, // for types used ONLY in one regular Api block, and ONLY in context of function outputs
    //     UniqueBlockAll, // for types used ONLY in one regular Api block, and in context of both function inputs and outputs
    //     SharedBlockAsInput, // for types sharely used among regular Api blocks ONLY in context of function inputs
    //     SharedBlockAsOutput, // for types sharely used among regular Api blocks ONLY in context of function outputs
    //     SharedBlockAll, // for types sharely used among regular Api blocks in context of both function inputs and outputs
    Input,
    Output,
}
impl IrTypeUseRange {
    // TODO: delete
    // pub(crate) fn is_shared(&self) -> bool {
    //     match self {
    //         IrTypeUseRange::SharedBlockAsInput
    //         | IrTypeUseRange::SharedBlockAsOutput
    //         | IrTypeUseRange::SharedBlockAll => true,
    //         _ => false,
    //     }
    // }
    pub(crate) fn is_input(&self) -> bool {
        self == &IrTypeUseRange::Input
        // TODO: delete
        // match self {
        //     IrTypeUseRange::UniqueBlockAsInput
        //     | IrTypeUseRange::UniqueBlockAll
        //     | IrTypeUseRange::SharedBlockAsInput
        //     | IrTypeUseRange::SharedBlockAll => true,
        //     _ => false,
        // }
    }
    pub(crate) fn is_output(&self) -> bool {
        self == &IrTypeUseRange::Output
        // TODO: delete
        // match self {
        //     IrTypeUseRange::UniqueBlockAsOutput
        //     | IrTypeUseRange::UniqueBlockAll
        //     | IrTypeUseRange::SharedBlockAsOutput
        //     | IrTypeUseRange::SharedBlockAll => true,
        //     _ => false,
        // }
    }
}

#[ext(name = PathExt)]
pub(crate) impl std::path::Path {
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

#[ext(name = ExtraTraitForVec)]
pub(crate) impl<T: Clone + Eq + std::hash::Hash> Vec<T> {
    /// This function finds unique and duplicate elements in a vector within original order.
    ///
    /// # Arguments
    ///
    /// * `exclude_duplicates_in_uniques` - A boolean that indicates whether to exclude duplicate elements from the original list for the returned unique list.
    /// * `exclude_duplicates_in_duplicates` - A boolean that indicates whether to exclude duplicate elements in the returned duplicate list.
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
    fn find_uniques_and_duplicates_in_order(
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

    /// This function finds unique elements in a vector within original order.
    /// # Arguments
    /// * `exclude_duplicates` - A boolean that indicates whether to exclude duplicate elements from the original list for the returned unique list.
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
    fn find_uniques_in_order(&self, exclude_duplicates: bool) -> Vec<T> {
        let (uniques, _) = self.find_uniques_and_duplicates_in_order(exclude_duplicates, false);
        uniques
    }

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
    fn find_duplicates_in_order(&self, exclude_duplicates: bool) -> Vec<T> {
        let (_, duplicates) = self.find_uniques_and_duplicates_in_order(true, exclude_duplicates);
        duplicates
    }
}

/// TODO: need? ouput `IrType`s with no duplicated safe_ident
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

pub fn read_rust_file(path: impl AsRef<Path>) -> String {
    let path_str = path.as_ref().to_str().unwrap();
    let (dir, module) = get_dir_and_mod(path_str);
    cargo_expand(&dir, module, path_str)
}

/// return the corresponding rust module/file full path if the input is a valid rust file
/// or rust module folder
pub fn get_rust_module_by_path(path: &str) -> Option<String> {
    let path = Path::new(path);
    if !path.exists() {
        return None;
    }
    if path.is_file() {
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                return Some(path.to_string_lossy().to_string());
            }
        }
    } else if path.is_dir() {
        let mod_rs_path = path.join("mod.rs");
        if mod_rs_path.exists() {
            return Some(mod_rs_path.to_string_lossy().to_string());
        }
    }
    None
}

fn get_dir_and_mod(path: &str) -> (String, Option<String>) {
    const SRC: &str = "/src/";
    #[cfg(windows)]
    let path = path.replace('\\', "/");
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

/// Filter the content of the given rust file by the given types.
/// Specifically, only the given types and their impls will be returned.
pub fn filter_type_content(raw_rust_file: &str, types_names: &[&str]) -> String {
    let syntax = parse_file(raw_rust_file).unwrap();
    let mut final_result = String::new();
    for type_name in types_names {
        let mut result = String::new();
        let target_type: Type = syn::parse_str(type_name).unwrap();

        for item in syntax.clone().items {
            match item {
                Item::Struct(item_struct) if item_struct.ident == type_name => {
                    result.push_str(&item_struct.into_token_stream().to_string());
                    result.push('\n');
                }
                Item::Enum(item_enum) if item_enum.ident == type_name => {
                    result.push_str(&item_enum.into_token_stream().to_string());
                    result.push('\n');
                }
                Item::Impl(item_impl)
                    if item_impl.self_ty.to_token_stream().to_string()
                        == target_type.to_token_stream().to_string() =>
                {
                    result.push_str(&item_impl.into_token_stream().to_string());
                    result.push('\n');
                }
                _ => {}
            }
        }
        final_result.push_str(&result);
    }

    final_result
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

    #[test]
    fn test_find_uniques_and_duplicates() {
        let vec = vec![1, 2, 3, 4, 5, 2, 1, 3];

        // Case 1: exclude duplicates in both uniques and duplicates
        let (uniques, duplicates) = vec.find_uniques_and_duplicates_in_order(true, true);
        assert_eq!(uniques, vec![4, 5]);
        assert_eq!(duplicates, vec![1, 2, 3]);

        // Case 2: only exclude duplicates in uniques
        let (uniques, duplicates) = vec.find_uniques_and_duplicates_in_order(true, false);
        assert_eq!(uniques, vec![4, 5]);
        assert_eq!(duplicates, vec![1, 2, 3, 2, 1, 3]);

        // Case 3: only exclude duplicates in duplicates
        let (uniques, duplicates) = vec.find_uniques_and_duplicates_in_order(false, true);
        assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
        assert_eq!(duplicates, vec![1, 2, 3]);

        // Case 4: include duplicates in both uniques and duplicates
        let (uniques, duplicates) = vec.find_uniques_and_duplicates_in_order(false, false);
        assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
        assert_eq!(duplicates, vec![1, 2, 3, 2, 1, 3]);
    }

    #[test]
    fn test_find_uniques() {
        let vec = vec![1, 2, 3, 4, 5, 2, 1, 3];

        let uniques = vec.find_uniques_in_order(true);
        assert_eq!(uniques, vec![4, 5]);

        let uniques = vec.find_uniques_in_order(false);
        assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_find_duplicates() {
        let vec = vec![1, 2, 3, 4, 5, 2, 1, 3];

        let duplicates = vec.find_duplicates_in_order(true);
        assert_eq!(duplicates, vec![1, 2, 3]);

        let duplicates = vec.find_duplicates_in_order(false);
        assert_eq!(duplicates, vec![1, 2, 3, 2, 1, 3]);
    }
}
