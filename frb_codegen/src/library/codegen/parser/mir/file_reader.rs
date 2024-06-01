// use crate::codegen::dumper::Dumper;
// use crate::codegen::parser::reader::CachedRustReader;
// use std::path::{Path, PathBuf};
// use syn::File;
//
// pub(crate) struct FileData {
//     pub(crate) path: PathBuf,
//     pub(crate) content: String,
//     pub(crate) ast: File,
// }
//
// pub(crate) fn read_files(
//     rust_input_paths: &[PathBuf],
//     rust_crate_dir: &Path,
//     cached_rust_reader: &mut CachedRustReader,
//     dumper: &Dumper,
// ) -> anyhow::Result<Vec<FileData>> {
//     let contents = rust_input_paths
//         .iter()
//         .map(|rust_input_path| {
//             let content =
//                 cached_rust_reader.read_rust_file(rust_input_path, rust_crate_dir, dumper)?;
//             Ok((rust_input_path.to_owned(), content))
//         })
//         .collect::<anyhow::Result<Vec<(PathBuf, String)>>>()?;
//
//     contents
//         .into_iter()
//         .map(|(rust_input_path, content)| {
//             let ast = syn::parse_file(&content)?;
//             Ok(FileData {
//                 path: rust_input_path,
//                 content,
//                 ast,
//             })
//         })
//         .collect()
// }
