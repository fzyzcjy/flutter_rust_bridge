// use crate::codegen::dumper::Dumper;
// use crate::codegen::parser::file_reader::read_files;
// use crate::codegen::parser::function_extractor::extract_generalized_functions_from_file;
// use crate::codegen::parser::reader::CachedRustReader;
// use itertools::Itertools;
// use log::warn;
// use std::path::{Path, PathBuf};
//
// pub(crate) fn check_suppressed_input_path_no_content(
//     rust_suppressed_input_paths: &[PathBuf],
//     rust_crate_dir: &Path,
//     cached_rust_reader: &mut CachedRustReader,
//     dumper: &Dumper,
// ) -> anyhow::Result<()> {
//     let file_data_arr = read_files(
//         rust_suppressed_input_paths,
//         rust_crate_dir,
//         cached_rust_reader,
//         dumper,
//     )?;
//
//     for file in file_data_arr.iter() {
//         let extracted_fns = extract_generalized_functions_from_file(&file.ast, &file.path)?;
//         if !extracted_fns.is_empty() {
//             warn!(
//                 "Functions or methods in {:?} are ignored. Please do not put them in `mod.rs`. (Function names: {:?})",
//                 file.path,
//                 extracted_fns.iter().map(|f| f.generalized_item_fn.sig().ident.to_string()).collect_vec()
//             );
//         }
//     }
//
//     Ok(())
// }
