//! Tools for environment checking
//!
//! please note that in the future this can probably be greatly simplified,
//! and beware that Cargo and Dart interpret semantic versioning differently:
//! see this [discussion](https://github.com/fzyzcjy/flutter_rust_bridge/pull/605#discussion_r935180160) for more information.

use anyhow::Context;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub(crate) mod dart_repo;
pub(crate) mod dart_toolchain;
pub(crate) mod pubspec;
pub(crate) mod version_converter;

pub(crate) fn get_dart_package_name(dart_root: &Path) -> anyhow::Result<String> {
    let pubspec_yaml: Value = serde_yaml::from_slice(&fs::read(dart_root.join("pubspec.yaml"))?)?;
    Ok(pubspec_yaml
        .get("name")
        .context("no name field")?
        .as_str()
        .context("cannot be str")?
        .to_owned())
}

#[cfg(test)]
mod tests {
    use super::dart_repo::DartRepository;
    use super::dart_toolchain::DartToolchain;
    use crate::utils::dart_repository::pubspec::{
        DartDependencyVersion, PubspecYaml, PubspecYamlDependencyVersion,
    };
    use cargo_metadata::VersionReq;
    use lazy_static::lazy_static;
    use semver::Op;
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
        str::FromStr,
    };

    lazy_static! {
        static ref FRB_EXAMPLES_FOLDER: PathBuf = {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("frb_example")
        };
    }

    fn guess_toolchain_base(path: &Path, expect_toolchain: DartToolchain) {
        let repo = DartRepository::from_str(&path.to_string_lossy())
            .unwrap_or_else(|_| panic!("can get toolchain from {}", path.to_string_lossy()));
        assert_eq!(repo.toolchain, expect_toolchain);
    }

    #[test]
    fn guess_dart_toolchain() {
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER.join("pure_dart").as_path(),
            DartToolchain::Dart,
        );
    }

    #[test]
    fn guess_flutter_toolchain() {
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER.join("flutter_via_integrate").as_path(),
            DartToolchain::Flutter,
        );
    }

    #[test]
    fn cannot_parse_dart_range_syntax() {
        assert!(VersionReq::parse(">=0.1.2 <0.2.0").is_err());
    }

    #[test]
    fn can_parse_dart_caret_syntax() {
        let caret = VersionReq::parse("^0.1.2");
        assert!(caret.is_ok());
        assert_eq!(caret.unwrap().comparators.first().unwrap().op, Op::Caret);
    }

    #[test]
    fn cannot_compare_version_req_with_different_op() {
        assert_ne!(
            VersionReq::parse("0.2.1").unwrap(),
            VersionReq::parse(">=0.2.1, <0.3.0").unwrap()
        );
    }

    #[test]
    fn can_parse_pubspec_deps() {
        let yaml = "
            dependencies:
                this_package: ^1.0.1
                that_package: 1.0.1
                other_package:
        ";
        let pubspec: PubspecYaml =
            serde_yaml::from_str(yaml).expect("Failed to parse pubspec.yaml");
        let mut expected = HashMap::new();
        expected.insert(
            "this_package".to_string(),
            Some(PubspecYamlDependencyVersion::Inline(DartDependencyVersion(
                "^1.0.1".to_string(),
            ))),
        );
        expected.insert(
            "that_package".to_string(),
            Some(PubspecYamlDependencyVersion::Inline(DartDependencyVersion(
                "1.0.1".to_string(),
            ))),
        );
        expected.insert("other_package".to_string(), None);

        assert_eq!(pubspec.dependencies, Some(expected));
    }
}
