use crate::command_run;
use crate::integration::utils::extract_dir_and_modify;
use crate::library::commands::flutter::flutter_pub_add;
use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Result;
use include_dir::{include_dir, Dir};
use itertools::Itertools;
use log::{debug, warn};
use std::env;
use std::ffi::OsStr;
use std::path::Path;

static INTEGRATION_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template");

/// Integrate Rust into existing Flutter project.
// ref: https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/
pub fn integrate(enable_integration_test: bool) -> Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");

    extract_dir_and_modify(
        &INTEGRATION_TEMPLATE_DIR,
        &dart_root,
        &modify_file,
        &filter_file,
    )?;

    pub_add_dependencies()?;

    Ok(())
}

fn modify_file(path: &Path, src_raw: &[u8], existing_content: Option<Vec<u8>>) -> Option<Vec<u8>> {
    if let Some(existing_content) = existing_content {
        if path.file_name() == Some(OsStr::new("main.dart")) {
            let existing_content = String::from_utf8(existing_content);
            let commented_existing_content = existing_content
                .map(|x| {
                    format!(
                        "\n\n{}",
                        x.split('\n').map(|line| format!("// {line}")).join("\n")
                    )
                })
                .unwrap_or_default();
            return Some([src_raw, commented_existing_content.as_bytes()].concat());
        }

        warn!(
            "Skip writing to {path:?} because file already exists. \
            It is suggested to remove that file before running this command to apply the full template."
        );
        return None;
    }

    if path.iter().contains(&OsStr::new("cargokit")) {
        if let Some(comments) = compute_cargokit_comments(path) {
            return Some([comments.as_bytes(), src_raw].concat());
        }
    }

    Some(src_raw.to_owned())
}

fn filter_file(path: &Path) -> bool {
    if path.iter().contains(&OsStr::new("cargokit")) {
        return !vec![".git", ".github", "docs", "test"].contains(&file_name(path));
    }

    true
}

fn compute_cargokit_comments(path: &Path) -> Option<String> {
    if vec![".gitignore"].contains(&file_name(path)) {
        return None;
    }

    let comment_leading = match file_extension(path) {
        "dart" | "md" | "gradle" | "" => "///",
        "yaml" | "toml" | "sh" => "#",
        "lock" | "cmake" | "ps1" | "cmd" => return None,
        ext => unreachable!("unexpected file extension for path={:?} ext={}", path, ext),
    };

    Some(
        (CARGOKIT_PRELUDE.iter())
            .map(|line| format!("{comment_leading} {line}"))
            .join("\n"),
    )
}

fn file_name(p: &Path) -> &str {
    p.file_name().unwrap().to_str().unwrap()
}

fn file_extension(p: &Path) -> &str {
    p.extension().unwrap_or_default().to_str().unwrap()
}

const CARGOKIT_PRELUDE: &[&str] = &[
    "This is copied from cargokit, TODO",
    "TODO explain more",
    "\n\n",
];

fn pub_add_dependencies() -> Result<()> {
    flutter_pub_add(&[
        r#"rust_builder:{"path":"./rust_builder"}"#.into(),
        r#"flutter_rust_bridge:{"path":"../../frb_dart"}"#.into(),
        r#"dev:ffigen:^8.0.0"#.into(),
    ])
}
