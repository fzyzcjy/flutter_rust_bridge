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
