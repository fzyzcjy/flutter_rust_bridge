// pub(crate) struct CachedRustReader {
//     cache: SimpleCache<PathBuf, syn::File>,
// }
//
// impl Default for CachedRustReader {
//     fn default() -> Self {
//         Self {
//             cache: SimpleCache::default(),
//         }
//     }
// }
//
// impl CachedRustReader {
//
// pub(crate) fn read_rust_crate(rust_crate_dir: &Path, dumper: &Dumper) -> Result<syn::File> {
//     debug!("read_rust_crate rust_crate_dir={rust_crate_dir:?}");
//     // let ans = (self.cache).get_or_insert(rust_crate_dir.to_owned(), || {
//     let text = run_cargo_expand(rust_crate_dir, dumper)?;
//     Ok(syn::parse_file(&text)?)
//     // })?;
//     // dumper.dump_str(ConfigDumpContent::Source, "read_rust_crate/data.rs", &ans)?;
//     // Ok(ans)
// }
//
// }
//
// fn get_rust_mod(rust_file_path: &Path, rust_crate_dir: &Path) -> Result<Option<String>> {
//     let relative_path = rust_file_path
//         .strip_prefix(rust_crate_dir.join("src"))
//         .with_context(|| {
//             // This will stop the whole generator and tell the users, so we do not care about testing it
//             // frb-coverage:ignore-start
//             format!("rust_file_path={rust_file_path:?} rust_crate_dir={rust_crate_dir:?}")
//             // frb-coverage:ignore-end
//         })?;
//
//     let mut components = relative_path
//         .iter()
//         .map(|s| s.to_str().unwrap().to_owned())
//         .collect_vec();
//
//     strip_suffix_inplace(components.last_mut().unwrap(), ".rs");
//
//     if components.last().unwrap() == "mod" || components.last().unwrap() == "lib" {
//         components.pop();
//     }
//
//     let ans = components.join("::");
//     Ok(if ans.is_empty() { None } else { Some(ans) })
// }
//
// fn strip_suffix_inplace(s: &mut String, suffix: &str) {
//     if let Some(stripped) = s.strip_suffix(suffix) {
//         *s = stripped.to_owned();
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::path::PathBuf;
//
//     #[test]
//     pub fn test_get_dir_and_mod_simple_mod() {
//         let actual = get_rust_mod(
//             &PathBuf::from("/project/src/api.rs"),
//             &PathBuf::from("/project"),
//         )
//         .unwrap();
//         assert_eq!(Some("api".to_owned()), actual);
//     }
//
//     #[test]
//     pub fn test_get_dir_and_mod_sub_mod() {
//         let actual = get_rust_mod(
//             &PathBuf::from("/project/src/sub/subsub.rs"),
//             &PathBuf::from("/project"),
//         )
//         .unwrap();
//         assert_eq!(Some("sub::subsub".to_owned()), actual);
//     }
//
//     #[test]
//     pub fn test_get_dir_and_mod_lib_rs() {
//         let actual = get_rust_mod(
//             &PathBuf::from("/project/src/lib.rs"),
//             &PathBuf::from("/project"),
//         )
//         .unwrap();
//         assert_eq!(None, actual);
//     }
//
//     #[test]
//     pub fn test_get_dir_and_mod_mod_rs() {
//         let actual = get_rust_mod(
//             &PathBuf::from("/project/src/hello/mod.rs"),
//             &PathBuf::from("/project"),
//         )
//         .unwrap();
//         assert_eq!(Some("hello".to_owned()), actual);
//     }
// }
