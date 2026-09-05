use serde::Deserialize;
use std::collections::HashMap;

/// The `pubspec.yaml` in a Dart/Flutter repository
#[derive(Debug, Deserialize)]
pub(crate) struct PubspecYaml {
    pub dependencies: Option<HashMap<String, Option<PubspecYamlDependencyVersion>>>,
    pub dev_dependencies: Option<HashMap<String, Option<PubspecYamlDependencyVersion>>>,
    pub resolution: Option<String>,
    pub workspace: Option<Vec<String>>,
}

/// e.g.
/// ```yaml
/// freezed: ^3.0.6
/// ```
/// or
/// ```yaml
/// freezed:
///   version: ^3.0.6
/// ```
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub(crate) enum PubspecYamlDependencyVersion {
    Inline(DartDependencyVersion),
    Multiline {
        version: Option<DartDependencyVersion>,
    },
}

/// The `pubspec.lock` in a Dart/Flutter repository
#[derive(Debug, Deserialize)]
pub(crate) struct PubspecLock {
    pub packages: HashMap<String, PubspecLockPackage>,
}

/// represents a dependency from pubspec.lock
#[derive(Debug, Deserialize)]
pub(crate) struct PubspecLockPackage {
    pub dependency: String,
    pub version: DartDependencyVersion,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub(crate) struct DartDependencyVersion(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Deserializes inline, multiline, and unspecified dependency versions.
    fn deserializes_pubspec_dependency_version_forms() {
        let pubspec: PubspecYaml = serde_yaml::from_str(
            "dependencies:\n  inline: ^1.2.3\n  multiline:\n    version: 2.0.0\n  no_version:\n    git: https://example.com/package.git\n  absent:\n",
        )
        .unwrap();

        let dependencies = pubspec.dependencies.unwrap();
        assert_eq!(
            dependencies.get("inline"),
            Some(&Some(PubspecYamlDependencyVersion::Inline(
                DartDependencyVersion("^1.2.3".to_owned())
            )))
        );
        assert_eq!(
            dependencies.get("multiline"),
            Some(&Some(PubspecYamlDependencyVersion::Multiline {
                version: Some(DartDependencyVersion("2.0.0".to_owned()))
            }))
        );
        assert_eq!(
            dependencies.get("no_version"),
            Some(&Some(PubspecYamlDependencyVersion::Multiline {
                version: None
            }))
        );
        assert_eq!(dependencies.get("absent"), Some(&None));
    }

    #[test]
    /// Deserializes optional pubspec sections when they are present.
    fn deserializes_optional_pubspec_sections() {
        let pubspec: PubspecYaml = serde_yaml::from_str(
            "dev_dependencies:\n  test: any\nresolution: workspace\nworkspace:\n  - packages/one\n  - packages/two\n",
        )
        .unwrap();

        assert_eq!(pubspec.resolution.as_deref(), Some("workspace"));
        assert_eq!(
            pubspec.workspace,
            Some(vec!["packages/one".to_owned(), "packages/two".to_owned()])
        );
        assert_eq!(
            pubspec.dev_dependencies.unwrap().get("test"),
            Some(&Some(PubspecYamlDependencyVersion::Inline(
                DartDependencyVersion("any".to_owned())
            )))
        );
    }

    #[test]
    /// Deserializes lock packages with their dependency class and version.
    fn deserializes_lock_packages() {
        let lock: PubspecLock = serde_yaml::from_str(
            "packages:\n  collection:\n    dependency: transitive\n    version: 1.19.0\n",
        )
        .unwrap();

        let package = lock.packages.get("collection").unwrap();
        assert_eq!(package.dependency, "transitive");
        assert_eq!(package.version, DartDependencyVersion("1.19.0".to_owned()));
    }
}
