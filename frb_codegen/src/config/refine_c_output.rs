use crate::config::opts_parser::{canon_path, format_fail_to_guess_error, raw_opts_bail};
use anyhow::{Context, Result};
use std::path::Path;

pub(crate) fn get_refined_c_output(
    c_output: &Option<Vec<String>>,
    extra_c_output_path: &Option<Vec<String>>,
    rust_input_paths: &Vec<String>,
) -> Vec<Vec<String>> {
    use clap::error::ErrorKind;

    assert!(!rust_input_paths.is_empty());

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
    if c_output_paths.len() != rust_input_paths.len() {
        raw_opts_bail(
            ErrorKind::WrongNumberOfValues,
            "--c-output's inputs should match --rust-input's length".into(),
        );
    }
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
    fn test_coutput_with_no_input_block() {
        let rust_input = vec![];
        let c_output = None::<Vec<String>>;
        let extra_c_output_path = None::<Vec<String>>;
        let _ = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);
    }

    // #[test]
    // TODO: how to catch panic from clap error?
    // #[should_panic]
    // fn test_coutput_with_inconsistent_number_of_input_block_api() {
    //     let c_output = Some(vec!["pathA/api_1.rs".into(),"pathA/api_2.rs".into()]);
    //     let rust_input = vec!["api_1.rs".into()];
    //     let extra_c_output_path = None::<Vec<String>>;
    //     let _ = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);
    //     unreachable !()
    // }

    #[test]
    fn test_single_block_with_no_c_output_with_no_extra_paths() {
        let rust_input = vec!["api_1.rs".into()];
        let c_output = None;
        let extra_c_output_path = None;
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
    }

    #[test]
    fn test_single_block_with_c_output_with_extra_paths() {
        let rust_input = vec!["api_1.rs".into()];
        let c_output = Some(vec!["./c_output.h".into()]);
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);

        // check 1st path
        let (dir, file) = get_dir_and_file_str(&result[0][0]);
        assert_eq!(dir, env::current_dir().unwrap().display().to_string());
        assert_eq!(&file, "c_output.h");
        // check 2ed path
        assert_eq!(&result[0][1], "./extra_path_1/c_output.h");
        // check 3ed path
        assert_eq!(&result[0][2], "./extra_path_2/c_output.h");
    }

    #[test]
    fn test_single_block_with_c_output_with_no_extra_paths() {
        let rust_input = vec!["api_1.rs".into()];
        let c_output = Some(vec!["./c_output.h".into()]);
        let extra_c_output_path = None;
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);

        // check path
        let (dir, file) = get_dir_and_file_str(&result[0][0]);
        assert_eq!(dir, env::current_dir().unwrap().display().to_string());
        assert_eq!(&file, "c_output.h");
    }

    #[test]
    fn test_single_block_with_no_c_output_with_extra_paths() {
        let rust_input = vec!["api_1.rs".into()];
        let c_output = None;
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);

        // get essential info from 1st path which has an arbitrary name
        let (_, file) = get_dir_and_file_str(&result[0][0]);
        // check 2ed path
        assert_eq!(&result[0][1], &format!("./extra_path_1/{}", file));
        // check 3ed path
        assert_eq!(&result[0][2], &format!("./extra_path_2/{}", file));
    }

    #[test]
    fn test_multi_blocks_with_no_c_output_with_no_extra_paths() {
        let rust_input = vec!["api_1.rs".into(), "api_2.rs".into()];
        let c_output = None;
        let extra_c_output_path = None;
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
    }
    #[test]
    fn test_multi_blocks_with_c_output_with_extra_paths() {
        let rust_input = vec!["api_1.rs".into(), "api_2.rs".into()];
        let c_output = Some(vec!["./c_output_1.h".into(), "./c_output_2.h".into()]);
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

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
            // check 3ed path
            assert_eq!(
                &each_block[2],
                &format!("./extra_path_2/c_output_{}.h", i + 1)
            );
        });
    }

    #[test]
    fn test_multi_blocks_with_c_output_with_no_extra_paths() {
        let rust_input = vec!["api_1.rs".into(), "api_2.rs".into()];
        let c_output = Some(vec!["./c_output_1.h".into(), "./c_output_2.h".into()]);
        let extra_c_output_path = None;
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

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
    fn test_multi_blocks_with_no_c_output_with_extra_paths() {
        let rust_input = vec!["api_1.rs".into(), "api_2.rs".into()];
        let c_output = None;
        let extra_c_output_path = Some(vec!["./extra_path_1/".into(), "./extra_path_2/".into()]);
        let result = get_refined_c_output(&c_output, &extra_c_output_path, &rust_input);

        assert_eq!(result.len(), 2);
        result.iter().for_each(|each_block| {
            assert_eq!(each_block.len(), 3);
        });

        result.iter().for_each(|each_block| {
            // get essential info from 1st path which has an arbitrary name
            let (_, file) = get_dir_and_file_str(&each_block[0]);
            // check 2ed path
            assert_eq!(&each_block[1], &format!("./extra_path_1/{}", file));
            // check 3ed path
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
