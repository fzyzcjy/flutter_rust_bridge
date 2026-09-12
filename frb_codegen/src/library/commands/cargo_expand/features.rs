use crate::utils::crate_name::CrateName;
use anyhow::{bail, Context, Result};
use cargo_metadata::MetadataCommand;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(super) fn feature_args(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
    configured: Option<&[String]>,
) -> Result<Vec<PathBuf>> {
    let environment = std::env::vars_os()
        .filter_map(|(key, value)| Some((key.into_string().ok()?, value.into_string().ok()?)))
        .collect();
    feature_args_with_env(
        rust_crate_dir,
        interest_crate_name.map(CrateName::raw),
        configured,
        &environment,
    )
}

fn feature_args_with_env(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&str>,
    configured: Option<&[String]>,
    environment: &HashMap<String, String>,
) -> Result<Vec<PathBuf>> {
    let inherited = if configured.is_none() && environment.contains_key("OUT_DIR") {
        inherited_features(rust_crate_dir, interest_crate_name, environment)?
    } else {
        None
    };
    let mut args = Vec::new();
    if inherited.is_some() {
        args.push(PathBuf::from("--no-default-features"));
    }
    for feature in configured.or(inherited.as_deref()).unwrap_or_default() {
        args.extend([PathBuf::from("--features"), PathBuf::from(feature)]);
    }
    Ok(args)
}

fn inherited_features(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&str>,
    environment: &HashMap<String, String>,
) -> Result<Option<Vec<String>>> {
    let Some(manifest_dir) = environment.get("CARGO_MANIFEST_DIR") else {
        return Ok(None);
    };
    if Path::new(manifest_dir).canonicalize()? != rust_crate_dir.canonicalize()? {
        return Ok(None);
    }
    if let Some(name) = interest_crate_name {
        if environment.get("CARGO_PKG_NAME").map(String::as_str) != Some(name) {
            return Ok(None);
        }
    }
    if let Some(features) = environment.get("CARGO_CFG_FEATURE") {
        return Ok(Some(
            features
                .split(',')
                .filter(|x| !x.is_empty())
                .map(str::to_owned)
                .collect(),
        ));
    }

    let manifest_path = rust_crate_dir.join("Cargo.toml").canonicalize()?;
    let metadata = MetadataCommand::new()
        .manifest_path(&manifest_path)
        .no_deps()
        .exec()?;
    let package = metadata
        .packages
        .iter()
        .find(|package| {
            package
                .manifest_path
                .as_std_path()
                .canonicalize()
                .ok()
                .as_ref()
                == Some(&manifest_path)
        })
        .context("Cargo metadata did not contain the build-script package")?;
    let mut enabled = HashMap::new();
    for feature in package.features.keys() {
        let key = format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"));
        if environment.contains_key(&key) {
            if let Some(previous) = enabled.insert(key, feature.clone()) {
                bail!("Cannot distinguish enabled Cargo features {previous:?} and {feature:?}; use Cargo 1.85 or newer, or configure rust_features explicitly");
            }
        }
    }
    let mut features: Vec<_> = enabled.into_values().collect();
    features.sort();
    Ok(Some(features))
}

#[cfg(test)]
mod tests {
    use super::feature_args_with_env;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use tempfile::{tempdir, TempDir};

    /// Expansion uses the outer feature set, including disabled default features.
    #[test]
    fn test_expansion_matches_outer_features() {
        let fixture = fixture();
        for (features, expected_default, expected_optional) in [
            ("default,default-api", true, false),
            ("optional-api,snake_api,MixedCase", false, true),
            ("", false, false),
        ] {
            let mut environment = environment(&fixture);
            environment.insert("CARGO_CFG_FEATURE".into(), features.into());
            let args = feature_args_with_env(fixture.path(), None, None, &environment).unwrap();
            let output = Command::new("cargo")
                .args(["rustc", "--lib"])
                .args(args)
                .args(["--", "-Zunpretty=expanded"])
                .env("RUSTC_BOOTSTRAP", "1")
                .current_dir(fixture.path())
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let expanded = String::from_utf8(output.stdout).unwrap();
            assert_eq!(expanded.contains("fn default_api"), expected_default);
            assert_eq!(expanded.contains("fn optional_api"), expected_optional);
        }
    }

    /// Older Cargo feature variables retain the names declared by the package.
    #[test]
    fn test_legacy_features_preserve_names() {
        let fixture = fixture();
        let mut environment = environment(&fixture);
        for name in ["OPTIONAL_API", "SNAKE_API", "MIXEDCASE"] {
            environment.insert(format!("CARGO_FEATURE_{name}"), "1".into());
        }
        assert_eq!(
            feature_args_with_env(fixture.path(), None, None, &environment).unwrap(),
            args(&[
                "--no-default-features",
                "--features",
                "MixedCase",
                "--features",
                "optional-api",
                "--features",
                "snake_api"
            ]),
        );
    }

    /// Ambiguous legacy feature names fail instead of enabling unintended APIs.
    #[test]
    fn test_legacy_feature_name_collision_is_rejected() {
        let fixture = fixture();
        let mut environment = environment(&fixture);
        environment.insert("CARGO_FEATURE_AMBIGUOUS_NAME".into(), "1".into());
        let error = feature_args_with_env(fixture.path(), None, None, &environment).unwrap_err();
        assert!(error
            .to_string()
            .contains("Cannot distinguish enabled Cargo features"));
    }

    /// Explicit feature configuration overrides inheritance without changing defaults.
    #[test]
    fn test_explicit_configuration_overrides_inheritance() {
        let fixture = fixture();
        let mut environment = environment(&fixture);
        environment.insert("CARGO_CFG_FEATURE".into(), "optional-api".into());
        for configured in [vec![], vec!["snake_api".to_owned()]] {
            let result =
                feature_args_with_env(fixture.path(), None, Some(&configured), &environment)
                    .unwrap();
            let expected = if configured.is_empty() {
                vec![]
            } else {
                args(&["--features", "snake_api"])
            };
            assert_eq!(result, expected);
        }
    }

    /// CLI and other packages do not inherit the current build script's features.
    #[test]
    fn test_features_are_only_inherited_for_the_build_script_package() {
        let fixture = fixture();
        let mut environment = environment(&fixture);
        environment.insert("CARGO_CFG_FEATURE".into(), "optional-api".into());
        assert!(
            feature_args_with_env(fixture.path(), Some("another_package"), None, &environment)
                .unwrap()
                .is_empty()
        );
        let other_package = tempdir().unwrap();
        assert!(
            feature_args_with_env(other_package.path(), None, None, &environment)
                .unwrap()
                .is_empty()
        );
        environment.remove("OUT_DIR");
        assert!(
            feature_args_with_env(fixture.path(), None, None, &environment)
                .unwrap()
                .is_empty()
        );
    }

    fn fixture() -> TempDir {
        let fixture = tempdir().unwrap();
        fs::create_dir(fixture.path().join("src")).unwrap();
        fs::write(
            fixture.path().join("Cargo.toml"),
            r#"
[package]
name = "feature_probe"
version = "0.1.0"
edition = "2021"
[features]
default = ["default-api"]
default-api = []
optional-api = []
snake_api = []
MixedCase = []
ambiguous-name = []
ambiguous_name = []
"#,
        )
        .unwrap();
        fs::write(
            fixture.path().join("src/lib.rs"),
            r#"
#[cfg(feature = "default-api")]
pub fn default_api() {}
#[cfg(all(feature = "optional-api", feature = "snake_api", feature = "MixedCase"))]
pub fn optional_api() {}
"#,
        )
        .unwrap();
        fixture
    }

    fn environment(fixture: &TempDir) -> HashMap<String, String> {
        [
            (
                "OUT_DIR".into(),
                fixture.path().join("out").to_str().unwrap().into(),
            ),
            (
                "CARGO_MANIFEST_DIR".into(),
                fixture.path().to_str().unwrap().into(),
            ),
            ("CARGO_PKG_NAME".into(), "feature_probe".into()),
        ]
        .into()
    }

    fn args(values: &[&str]) -> Vec<PathBuf> {
        values.iter().map(PathBuf::from).collect()
    }
}
