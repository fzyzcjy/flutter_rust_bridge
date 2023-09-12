use crate::config::opts_parser::{canon_path, format_fail_to_guess_error};
use crate::utils::misc::{is_same_directory, PathExt};
use anyhow::{Context, Result};
use std::path::Path;
pub(crate) fn get_refined_c_output(
    c_output: &Option<Vec<String>>,
    shared_rust_output_path: &Option<String>,
    extra_c_output_path: &Option<Vec<String>>,
    rust_input_paths: &Vec<String>,
) -> Vec<Vec<String>> {
    assert!(!rust_input_paths.is_empty());

    let c_output = if let Some(c_output) = c_output {
        assert_eq!(rust_input_paths.len(), c_output.len(), "when flag `c-output` is specified, then length of it should match that of `rust-input`");
        if c_output.len() <= 1 {
            Some(c_output.clone())
        } else if let Some(shared_rust_output_path) = shared_rust_output_path {
            if !is_same_directory(c_output) {
                panic!("for multi-blocks case, paths in flag `c-output`(if defined) should be in the same directory ");
            }
            let shared_rust_file_name = Path::new(&shared_rust_output_path).get_file_name();
            let directory = Path::new(&c_output[0]).get_directory_name();
            let shared_c_file_path = Path::join(Path::new(&directory), shared_rust_file_name)
                .into_os_string()
                .into_string()
                .unwrap()
                .replace(".rs", ".h");
            let mut ret_vec = c_output.clone();
            ret_vec.push(shared_c_file_path);
            Some(ret_vec)
        } else {
            Some(c_output.clone())
        }
    } else {
        None
    };

    // 1.c path with file name from flag rawOpt.c_output
    let c_output_paths = c_output
        .as_ref()
        .map(|outputs| {
            outputs
                .iter()
                .map(|output| canon_path(output))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            (0..rust_input_paths.len())
                .map(|_| {
                    fallback_c_output_path()
                        .unwrap_or_else(|_| panic!("{}", format_fail_to_guess_error("c_output")))
                })
                .collect()
        });
    // 2.extra c path from flag rawOpt.extra_c_output_path
    let extra_c_output_paths = extra_c_output_path
        .as_deref()
        .unwrap_or_default()
        .iter()
        .flat_map(|extra_path| {
            c_output_paths.iter().map(move |c| {
                let path = Path::new(c);
                let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
                Path::join(extra_path.as_ref(), file_name)
                    .into_os_string()
                    .into_string()
                    .unwrap()
            })
        })
        .collect::<Vec<_>>();
    // 3.integrate c output path(s) for each rust input API block
    let refined_c_outputs = c_output_paths
        .iter()
        .enumerate()
        .map(|(i, each_a)| {
            let mut first = vec![each_a.clone()];
            if !extra_c_output_paths.is_empty() {
                let iter = extra_c_output_paths[i..]
                    .iter()
                    .step_by(c_output_paths.len())
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>();
                first.extend(iter);
            }
            first
        })
        .collect::<Vec<_>>();
    refined_c_outputs
}
#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::path::PathBuf;
    fn get_dir_and_file_str(path: &str) -> (String, String) {
        let path = PathBuf::from(path);
        let directory = path.parent().unwrap().display().to_string();
        let file_name = path.file_name().unwrap().to_owned().into_string().unwrap();
        (directory, file_name)
    }
    #[test]
    #[should_panic]
    fn test_coutput_with_no_input_block_with_no_shared_output() {
        let rust_input = vec![];
        let c_output = None::<Vec<String>>;
        let extra_c_output_path = None::<Vec<String>>;
        let shared_rust_output_path = None;
        let _ = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
    }
    #[test]
    #[should_panic]
    fn test_coutput_with_inconsistent_number_of_input_block_api_with_no_shared_output() {
        let c_output = Some(vec![
            "pathA/api_block_1.rs".into(),
            "pathA/api_block_2.rs".into(),
        ]);
        let rust_input = vec!["api_block_1.rs".into()];
        let extra_c_output_path = None::<Vec<String>>;
        let shared_rust_output_path = None;
        let _ = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
    }
    #[test]
    fn test_single_block_with_no_c_output_with_no_extra_paths_with_no_shared_output() {
        let rust_input = vec!["api_block_1.rs".into()];
        let c_output = None;
        let extra_c_output_path = None;
        let shared_rust_output_path = None;
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
    }
    #[test]
    fn test_single_block_with_c_output_with_extra_paths_with_no_shared_output() {
        let rust_input = vec!["api_block_1.rs".into()];
        let c_output = Some(vec!["./c_output.h".into()]);
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let shared_rust_output_path = None;
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        // check 1st path
        let (dir, file) = get_dir_and_file_str(&result[0][0]);
        assert_eq!(dir, env::current_dir().unwrap().display().to_string());
        assert_eq!(&file, "c_output.h");
        // check 2ed path
        assert_eq!(&result[0][1], "./extra_path_1/c_output.h");
        // check 3rd path
        assert_eq!(&result[0][2], "./extra_path_2/c_output.h");
    }
    #[test]
    fn test_single_block_with_c_output_with_no_extra_paths_with_no_shared_output() {
        let rust_input = vec!["api_block_1.rs".into()];
        let c_output = Some(vec!["./c_output.h".into()]);
        let extra_c_output_path = None;
        let shared_rust_output_path = None;
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        // check path
        let (dir, file) = get_dir_and_file_str(&result[0][0]);
        assert_eq!(dir, env::current_dir().unwrap().display().to_string());
        assert_eq!(&file, "c_output.h");
    }
    #[test]
    fn test_single_block_with_no_c_output_with_extra_paths_with_no_shared_output() {
        let rust_input = vec!["api_block_1.rs".into()];
        let c_output = None;
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let shared_rust_output_path = None;
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        // get essential info from 1st path which has an arbitrary name
        let (_, file) = get_dir_and_file_str(&result[0][0]);
        // check 2ed path
        assert_eq!(&result[0][1], &format!("./extra_path_1/{}", file));
        // check 3rd path
        assert_eq!(&result[0][2], &format!("./extra_path_2/{}", file));
    }
    #[test]
    fn test_multi_blocks_with_no_c_output_with_no_extra_paths_with_no_shared_output() {
        let rust_input = vec!["api_block_1.rs".into(), "api_block_2.rs".into()];
        let c_output = None;
        let extra_c_output_path = None;
        let shared_rust_output_path = None;
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
    }
    #[test]
    fn test_multi_blocks_with_c_output_with_extra_paths_with_no_shared_output() {
        let rust_input = vec!["api_block_1.rs".into(), "api_block_2.rs".into()];
        let c_output = Some(vec!["./c_output_1.h".into(), "./c_output_2.h".into()]);
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let shared_rust_output_path = None;
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 2);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 3);
        });
        result.iter().enumerate().for_each(|(i, each_block)| {
            // check 1st path
            let (dir, file) = get_dir_and_file_str(&each_block[0]);
            assert_eq!(dir, env::current_dir().unwrap().display().to_string());
            assert_eq!(&file, &format!("c_output_{}.h", i + 1));
            // check 2ed path
            assert_eq!(
                &each_block[1],
                &format!("./extra_path_1/c_output_{}.h", i + 1)
            );
            // check 3rd path
            assert_eq!(
                &each_block[2],
                &format!("./extra_path_2/c_output_{}.h", i + 1)
            );
        });
    }
    #[test]
    fn test_multi_blocks_with_c_output_with_no_extra_paths_with_no_shared_output() {
        let rust_input = vec!["api_block_1.rs".into(), "api_block_2.rs".into()];
        let c_output = Some(vec!["./c_output_1.h".into(), "./c_output_2.h".into()]);
        let extra_c_output_path = None;
        let shared_rust_output_path = None;
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 2);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 1);
        });
        // check path
        result.iter().enumerate().for_each(|(i, each_block)| {
            let (dir, file) = get_dir_and_file_str(&each_block[0]);
            assert_eq!(dir, env::current_dir().unwrap().display().to_string());
            assert_eq!(&file, &format!("c_output_{}.h", i + 1));
        });
    }
    #[test]
    fn test_multi_blocks_with_no_c_output_with_extra_paths_with_no_shared_output() {
        let rust_input = vec!["api_block_1.rs".into(), "api_block_2.rs".into()];
        let c_output = None;
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let shared_rust_output_path = None;
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 2);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 3);
        });
        result.iter().for_each(|each_block| {
            // get essential info from 1st path which has an arbitrary name
            let (_, file) = get_dir_and_file_str(&each_block[0]);
            // check 2ed path
            assert_eq!(&each_block[1], &format!("./extra_path_1/{}", file));
            // check 3rd path
            assert_eq!(&each_block[2], &format!("./extra_path_2/{}", file));
        });
    }
    // ---- with shared path
    #[test]
    #[should_panic]
    fn test_coutput_with_no_input_block_with_shared_output() {
        let rust_input = vec![];
        let c_output = None::<Vec<String>>;
        let extra_c_output_path = None::<Vec<String>>;
        let shared_rust_output_path = Some("fake_directory/custom_shared_file_name.rs".to_owned());
        let _ = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
    }
    #[test]
    #[should_panic]
    fn test_coutput_with_inconsistent_number_of_input_block_api_with_shared_output() {
        let c_output = Some(vec![
            "pathA/api_block_1.rs".into(),
            "pathA/api_block_2.rs".into(),
        ]);
        let rust_input = vec!["api_block_1.rs".into()];
        let extra_c_output_path = None::<Vec<String>>;
        let shared_rust_output_path = Some("fake_directory/custom_shared_file_name.rs".to_owned());
        let _ = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
    }
    #[test]
    fn test_single_block_with_no_c_output_with_no_extra_paths_with_shared_output() {
        let rust_input = vec!["api_block_1.rs".into()];
        let c_output = None;
        let extra_c_output_path = None;
        let shared_rust_output_path = Some("fake_directory/custom_shared_file_name.rs".to_owned());
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
    }
    #[test]
    fn test_single_block_with_c_output_with_extra_paths_with_shared_output() {
        let rust_input = vec!["api_block_1.rs".into()];
        let c_output = Some(vec!["./c_output.h".into()]);
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let shared_rust_file_name = "custom_shared_file_name";
        let shared_rust_output_path = Some(format!("fake_directory/{shared_rust_file_name}.rs"));
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        // check 1st path
        let (dir, file) = get_dir_and_file_str(&result[0][0]);
        assert_eq!(dir, env::current_dir().unwrap().display().to_string());
        assert_eq!(&file, "c_output.h");
        // check 2ed path
        assert_eq!(&result[0][1], "./extra_path_1/c_output.h");
        // check 3rd path
        assert_eq!(&result[0][2], "./extra_path_2/c_output.h"); // check 1st path
    }
    #[test]
    fn test_single_block_with_c_output_with_no_extra_paths_with_shared_output() {
        let rust_input = vec!["api_block_1.rs".into()];
        let c_output = Some(vec!["./c_output.h".into()]);
        let extra_c_output_path = None;
        let shared_rust_output_path = Some("fake_directory/custom_shared_file_name.rs".to_owned());
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        // check path
        let (dir, file) = get_dir_and_file_str(&result[0][0]);
        assert_eq!(dir, env::current_dir().unwrap().display().to_string());
        assert_eq!(&file, "c_output.h");
    }
    #[test]
    fn test_single_block_with_no_c_output_with_extra_paths_with_shared_output() {
        let rust_input = vec!["api_block_1.rs".into()];
        let c_output = None;
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let shared_rust_output_path = Some("fake_directory/custom_shared_file_name.rs".to_owned());
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        // get essential info from 1st path which has an arbitrary name
        let (_, file) = get_dir_and_file_str(&result[0][0]);
        // check 2ed path
        assert_eq!(&result[0][1], &format!("./extra_path_1/{}", file));
        // check 3rd path
        assert_eq!(&result[0][2], &format!("./extra_path_2/{}", file));
    }
    #[test]
    fn test_multi_blocks_with_no_c_output_with_no_extra_paths_with_shared_output() {
        let rust_input = vec!["api_block_1.rs".into(), "api_block_2.rs".into()];
        let c_output = None;
        let extra_c_output_path = None;
        let shared_rust_output_path: Option<String> =
            Some("fake_directory/custom_shared_file_name.rs".to_owned());
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
    }
    #[test]
    fn test_multi_blocks_with_c_output_with_extra_paths_with_shared_output() {
        let rust_input = vec!["api_block_1.rs".into(), "api_block_2.rs".into()];
        let c_output = Some(vec!["./c_output_1.h".into(), "./c_output_2.h".into()]);
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let shared_rust_file_name = "custom_shared_file_name";
        let shared_rust_output_path = Some(format!("fake_directory/{shared_rust_file_name}.rs"));
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 3);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 3);
        });
        result.iter().enumerate().for_each(|(i, each_block)| {
            if i != result.len() - 1 {
                // check 1st path
                let (dir, file) = get_dir_and_file_str(&each_block[0]);
                assert_eq!(dir, env::current_dir().unwrap().display().to_string());
                assert_eq!(&file, &format!("c_output_{}.h", i + 1));
                // check 2ed path
                assert_eq!(
                    &each_block[1],
                    &format!("./extra_path_1/c_output_{}.h", i + 1)
                );
                // check 3rd path
                assert_eq!(
                    &each_block[2],
                    &format!("./extra_path_2/c_output_{}.h", i + 1)
                );
            } else {
                // the last one should be the shared paths
                // check 1st path
                let (dir, file) = get_dir_and_file_str(&each_block[0]);
                assert_eq!(dir, env::current_dir().unwrap().display().to_string());
                assert_eq!(&file, &format!("{shared_rust_file_name}.h"));
                // check 2ed path
                assert_eq!(
                    &each_block[1],
                    &format!("./extra_path_1/{shared_rust_file_name}.h")
                );
                // check 3rd path
                assert_eq!(
                    &each_block[2],
                    &format!("./extra_path_2/{shared_rust_file_name}.h")
                );
            }
        });
    }
    #[test]
    fn test_multi_blocks_with_c_output_with_no_extra_paths_with_shared_output() {
        let rust_input = vec!["api_block_1.rs".into(), "api_block_2.rs".into()];
        let c_output = Some(vec!["./c_output_1.h".into(), "./c_output_2.h".into()]);
        let extra_c_output_path = None;
        let shared_rust_file_name = "custom_shared_file_name";
        let shared_rust_output_path = Some(format!("fake_directory/{shared_rust_file_name}.rs"));
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 3);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 1);
        });
        // check path
        result.iter().enumerate().for_each(|(i, each_block)| {
            let (dir, file) = get_dir_and_file_str(&each_block[0]);
            assert_eq!(dir, env::current_dir().unwrap().display().to_string());
            if i != result.len() - 1 {
                assert_eq!(&file, &format!("c_output_{}.h", i + 1));
            } else {
                assert_eq!(&file, &format!("{shared_rust_file_name}.h"));
            }
        });
    }
    #[test]
    fn test_multi_blocks_with_no_c_output_with_extra_paths_with_shared_output() {
        let rust_input = vec!["api_block_1.rs".into(), "api_block_2.rs".into()];
        let c_output = None;
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let shared_rust_output_path = None;
        let result = get_refined_c_output(
            &c_output,
            &shared_rust_output_path,
            &extra_c_output_path,
            &rust_input,
        );
        assert_eq!(result.len(), 2);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 3);
        });
        result.iter().for_each(|each_block| {
            // get essential info from 1st path which has an arbitrary name
            let (_, file) = get_dir_and_file_str(&each_block[0]);
            // check 2ed path
            assert_eq!(&each_block[1], &format!("./extra_path_1/{}", file));
            // check 3rd path
            assert_eq!(&each_block[2], &format!("./extra_path_2/{}", file));
        });
    }
}
fn fallback_c_output_path() -> Result<String> {
    let named_temp_file = Box::leak(Box::new(tempfile::Builder::new().suffix(".h").tempfile()?));
    Ok(named_temp_file
        .path()
        .to_str()
        .context("Not a UTF-8 path")?
        .to_string())
}
