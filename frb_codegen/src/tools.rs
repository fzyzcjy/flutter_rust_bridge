//! tools and environment checking
//!
//! please note that in the future this can probably be greatly simplified,
//! and beware that Cargo and Dart interpret semantic versioning differently :
//! see this [discussion](https://github.com/fzyzcjy/flutter_rust_bridge/pull/605#discussion_r935180160) for more informations.

use std::{collections::HashMap, convert::TryFrom, path::PathBuf, str::FromStr};

use cargo_metadata::{Version, VersionReq};
use lazy_static::lazy_static;
use log::debug;
use serde::Deserialize;

use crate::{args, commands::call_shell, error::Error, run};

lazy_static! {
    pub(crate) static ref FFI_REQUIREMENT: VersionReq =
        VersionReq::parse(">= 2.0.1, < 3.0.0").unwrap();
    pub(crate) static ref FFIGEN_REQUIREMENT: VersionReq =
        VersionReq::parse(">= 6.0.1, < 8.0.0").unwrap();
}

/// represents dart or flutter toolchain
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DartToolchain {
    Dart,
    Flutter,
}

/// represents a dart / flutter repository
#[derive(Debug)]
pub(crate) struct DartRepository {
    pub(crate) at: PathBuf,
    pub(crate) toolchain: DartToolchain,
}

/// used to deserialize packages from pubspec.lock
#[derive(Debug, Deserialize)]
struct PubspecLock {
    pub packages: HashMap<String, PubspecLockDependency>,
}

/// represents a dependency from pubspec.lock
#[derive(Debug, Deserialize)]
struct PubspecLockDependency {
    pub dependency: String,
    pub version: DartDependencyVersion,
}

impl PubspecLockDependency {
    pub(crate) fn installed_in(&self) -> Option<PackageManager> {
        match self.dependency.as_str() {
            "direct dev" => Some(PackageManager::DevDependencies),
            "direct main" => Some(PackageManager::Dependencies),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct DartDependencyVersion(String);

#[derive(Debug, Clone)]
pub struct CargoDependencyVersion(String);

impl From<&DartDependencyVersion> for CargoDependencyVersion {
    /// convert from a dependency version in Dart syntax to Cargo syntax (to be used with VersionReq later on)
    ///
    /// be careful because this is where you can shoot yourself in the foot :)
    ///
    /// see module level comments for more informations.
    fn from(v: &DartDependencyVersion) -> Self {
        if v.0.starts_with('^') {
            let version = Version::parse(v.0.split_at(1).1).unwrap();

            if version.major > 0 {
                return CargoDependencyVersion(version.to_string());
            }
            return CargoDependencyVersion(
                version.to_string().rsplit_once('.').unwrap().0.to_string(),
            );
        }
        CargoDependencyVersion(v.0.clone())
    }
}

/// extract dependency version in pubspec.yaml, no matter its format
///
/// e.g.
/// ```yaml
/// freezed: ^2.0.1
/// ```
/// or
/// ```yaml
/// freezed:
///   version: ^2.0.1
/// ```
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PackageVersion {
    Inline(DartDependencyVersion),
    Multiline {
        version: Option<DartDependencyVersion>,
    },
}

/// represents a package version kind
#[derive(Debug, PartialEq, Eq)]
pub enum PackageVersionKind {
    /// exact dependency requirement
    /// e.g. `1.2.3`
    Exact(Version),
    /// a range of dependencies requirement
    /// e.g. `^1.2.3`
    Range(VersionReq),
}

/// represents dependencies package manager
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageManager {
    Dependencies,
    DevDependencies,
}

/// used to deserialize `dependencies` and `dev_dependencies` from pubspec.yaml
#[derive(Debug, Deserialize)]
struct Pubspec {
    pub dependencies: Option<HashMap<String, Option<PackageVersion>>>,
    pub dev_dependencies: Option<HashMap<String, Option<PackageVersion>>>,
}

#[inline]
fn read_file(at: &str, filename: &str) -> anyhow::Result<String> {
    let file = PathBuf::from(at).join(filename);
    if !file.exists() {
        return Err(anyhow::Error::msg(format!(
            "missing {} in {}",
            filename, at
        )));
    }
    let content = std::fs::read_to_string(file).map_err(|e| {
        anyhow::Error::msg(format!("unable to read {} in {}: {:#}", filename, at, e))
    })?;
    Ok(content)
}

impl ToString for DartToolchain {
    fn to_string(&self) -> String {
        match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter",
        }
        .to_string()
    }
}

impl DartToolchain {
    #[inline]
    pub fn manifest_filename() -> &'static str {
        "pubspec.yaml"
    }
    #[inline]
    pub fn lock_filename() -> &'static str {
        "pubspec.lock"
    }
}

impl DartToolchain {
    pub(crate) fn as_run_command(&self) -> Vec<PathBuf> {
        match self {
            DartToolchain::Dart => args!("dart"),
            DartToolchain::Flutter => args!("flutter", "pub"),
        }
    }
    pub(crate) fn available(&self) -> bool {
        let toolchain = match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter",
        };
        run!(call_shell[None], toolchain, "--version")
            .unwrap()
            .status
            .success()
    }
}

impl FromStr for DartRepository {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("Guessing toolchain the runner is run into");
        let filename = DartToolchain::lock_filename();
        let lock_file = read_file(s, filename)?;
        let lock_file: PubspecLock = serde_yaml::from_str(&lock_file).map_err(|e| {
            anyhow::Error::msg(format!("unable to parse {} in {}: {:#}", filename, s, e))
        })?;
        if lock_file
            .packages
            .contains_key(&DartToolchain::Flutter.to_string())
        {
            return Ok(DartRepository {
                at: PathBuf::from(s),
                toolchain: DartToolchain::Flutter,
            });
        }
        Ok(DartRepository {
            at: PathBuf::from(s),
            toolchain: DartToolchain::Dart,
        })
    }
}

impl DartRepository {
    /// check whether the toolchain is available from the CLI
    pub(crate) fn toolchain_available(&self) -> bool {
        self.toolchain.available()
    }
    /// check whether a package has been correctly specified in pubspec.yaml
    pub(crate) fn has_specified(
        &self,
        package: &str,
        manager: PackageManager,
        requirement: &VersionReq,
    ) -> anyhow::Result<()> {
        let at = self.at.to_str().unwrap();
        debug!("Checking presence of {} in {} at {}", package, manager, at);
        let manifest_file = read_file(at, DartToolchain::manifest_filename())?;
        let manifest_file: Pubspec = serde_yaml::from_str(&manifest_file).map_err(|e| {
            anyhow::Error::msg(format!(
                "unable to parse {} in {}: {:#}",
                DartToolchain::manifest_filename(),
                at,
                e
            ))
        })?;
        let deps = match manager {
            PackageManager::Dependencies => manifest_file.dependencies.unwrap_or_default(),
            PackageManager::DevDependencies => manifest_file.dev_dependencies.unwrap_or_default(),
        };
        deps.get(package).map(|_| ()).ok_or_else(|| {
            anyhow::Error::new(Error::MissingDep {
                name: package.to_string(),
                manager,
                requirement: requirement.to_string(),
            })
        })
    }
    /// check whether a package has been correctly pinned in pubspec.lock
    pub(crate) fn has_installed(
        &self,
        package: &str,
        manager: PackageManager,
        requirement: &VersionReq,
    ) -> anyhow::Result<()> {
        let at = self.at.to_str().unwrap();
        debug!("Checking presence of {} in {} at {}", package, manager, at);
        let lock_file = read_file(at, DartToolchain::lock_filename())?;
        let lock_file: PubspecLock = serde_yaml::from_str(&lock_file).map_err(|e| {
            anyhow::Error::msg(format!(
                "unable to parse {} in {}: {:#}",
                DartToolchain::lock_filename(),
                at,
                e
            ))
        })?;
        let dependency = lock_file.packages.get(package);
        let version = match dependency {
            Some(dependency) => {
                let pm = dependency.installed_in();
                if pm.as_ref() != Some(&manager) {
                    return Err(anyhow::Error::new(Error::InvalidDep {
                        name: package.to_string(),
                        manager,
                        requirement: requirement.to_string(),
                    }));
                }
                PackageVersionKind::try_from(dependency).map_err(|e| {
                    anyhow::Error::msg(format!(
                        "unable to parse {} version in {}: {:#}",
                        package,
                        DartToolchain::lock_filename(),
                        e
                    ))
                })?
            }
            None => {
                return Err(anyhow::Error::new(Error::MissingDep {
                    name: package.to_string(),
                    manager,
                    requirement: requirement.to_string(),
                }))
            }
        };

        match version {
            PackageVersionKind::Exact(ref v) if requirement.matches(v) => Ok(()),
            PackageVersionKind::Range(_) => Err(anyhow::Error::new(Error::StringError(format!(
                "unexpected version range for {} in {}",
                package,
                DartToolchain::lock_filename()
            )))),
            _ => Err(anyhow::Error::new(Error::InvalidDep {
                name: package.to_string(),
                manager,
                requirement: requirement.to_string(),
            })),
        }
    }
}

impl PackageVersion {
    pub(crate) fn version(&self) -> Option<DartDependencyVersion> {
        match self {
            PackageVersion::Inline(v) => Some(v.clone()),
            PackageVersion::Multiline { version } => version.clone(),
        }
    }
}

impl TryFrom<&PackageVersion> for PackageVersionKind {
    type Error = anyhow::Error;
    fn try_from(version: &PackageVersion) -> Result<Self, Self::Error> {
        if let Some(ref version) = version.version() {
            return Self::try_from(version);
        }
        Err(anyhow::anyhow!("no version found"))
    }
}

impl TryFrom<&PubspecLockDependency> for PackageVersionKind {
    type Error = anyhow::Error;
    fn try_from(dependency: &PubspecLockDependency) -> Result<Self, Self::Error> {
        Self::try_from(&dependency.version)
    }
}

impl TryFrom<&DartDependencyVersion> for PackageVersionKind {
    type Error = anyhow::Error;

    fn try_from(s: &DartDependencyVersion) -> Result<Self, Self::Error> {
        Self::try_from(&CargoDependencyVersion::from(s))
    }
}

impl TryFrom<&CargoDependencyVersion> for PackageVersionKind {
    type Error = anyhow::Error;

    fn try_from(s: &CargoDependencyVersion) -> Result<Self, Self::Error> {
        let range: [char; 4] = ['>', '<', '=', '~'];
        if s.0.contains(range) {
            let version_req = VersionReq::parse(&s.0)?;
            Ok(PackageVersionKind::Range(version_req))
        } else {
            let version = Version::parse(&s.0)?;
            Ok(PackageVersionKind::Exact(version))
        }
    }
}

impl ToString for PackageVersionKind {
    fn to_string(&self) -> String {
        match self {
            PackageVersionKind::Exact(v) => v.to_string(),
            PackageVersionKind::Range(v) => v.to_string(),
        }
    }
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Dependencies => write!(f, "dependencies"),
            PackageManager::DevDependencies => write!(f, "dev_dependencies"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
        str::FromStr,
    };

    use super::{DartDependencyVersion, DartRepository, DartToolchain, PackageVersion, Pubspec};
    use cargo_metadata::VersionReq;
    use lazy_static::lazy_static;
    use semver::Op;

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
            FRB_EXAMPLES_FOLDER.join("pure_dart").join("dart").as_path(),
            DartToolchain::Dart,
        );
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER
                .join("pure_dart_multi")
                .join("dart")
                .as_path(),
            DartToolchain::Dart,
        );
    }

    #[test]
    fn guess_flutter_toolchain() {
        guess_toolchain_base(
            FRB_EXAMPLES_FOLDER.join("with_flutter").as_path(),
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
        let pubspec: Pubspec = serde_yaml::from_str(yaml).expect("Failed to parse pubspec.yaml");
        let mut expected = HashMap::new();
        expected.insert(
            "this_package".to_string(),
            Some(PackageVersion::Inline(DartDependencyVersion(
                "^1.0.1".to_string(),
            ))),
        );
        expected.insert(
            "that_package".to_string(),
            Some(PackageVersion::Inline(DartDependencyVersion(
                "1.0.1".to_string(),
            ))),
        );
        expected.insert("other_package".to_string(), None);

        assert_eq!(pubspec.dependencies, Some(expected));
    }
}
