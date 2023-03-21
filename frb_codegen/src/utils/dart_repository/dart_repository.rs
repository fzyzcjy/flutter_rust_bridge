use std::convert::TryFrom;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use log::debug;
use cargo_metadata::{Version, VersionReq};
use crate::error::Error;
use crate::utils::dart_repository::dart_toolchain::DartToolchain;
use crate::utils::dart_repository::pubspec::*;

/// represents a dart / flutter repository
#[derive(Debug)]
pub(crate) struct DartRepository {
    pub(crate) at: PathBuf,
    pub(crate) toolchain: DartToolchain,
}

impl FromStr for DartRepository {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("Guessing toolchain the runner is run into");
        let filename = DartToolchain::lock_filename();
        let lock_file = read_file(s, filename)?;
        let lock_file: PubspecLock = serde_yaml::from_str(&lock_file)
            .map_err(|e| anyhow::Error::msg(format!("unable to parse {filename} in {s}: {e:#}")))?;
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
        manager: DartDependencyMode,
        requirement: &VersionReq,
    ) -> anyhow::Result<()> {
        let at = self.at.to_str().unwrap();
        debug!("Checking presence of {} in {} at {}", package, manager, at);
        let manifest_file = read_file(at, DartToolchain::manifest_filename())?;
        let manifest_file: PubspecYaml = serde_yaml::from_str(&manifest_file).map_err(|e| {
            anyhow::Error::msg(format!(
                "unable to parse {} in {}: {:#}",
                DartToolchain::manifest_filename(),
                at,
                e
            ))
        })?;
        let deps = match manager {
            DartDependencyMode::Main => manifest_file.dependencies.unwrap_or_default(),
            DartDependencyMode::Dev => manifest_file.dev_dependencies.unwrap_or_default(),
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
        manager: DartDependencyMode,
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
                DartPackageVersion::try_from(dependency).map_err(|e| {
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
            DartPackageVersion::Exact(ref v) if requirement.matches(v) => Ok(()),
            DartPackageVersion::Range(_) => Err(anyhow::Error::new(Error::StringError(format!(
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DartDependencyMode {
    /// Appear in `dependencies` of `pubspec.yaml`
    Main,
    /// Appear in `dev_dependencies` of `pubspec.yaml`
    Dev,
}

impl Display for DartDependencyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DartDependencyMode::Main => write!(f, "dependencies"),
            DartDependencyMode::Dev => write!(f, "dev_dependencies"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DartPackageVersion {
    /// exact dependency requirement
    /// e.g. `1.2.3`
    Exact(Version),
    /// a range of dependencies requirement
    /// e.g. `^1.2.3`
    Range(VersionReq),
}

impl ToString for DartPackageVersion {
    fn to_string(&self) -> String {
        match self {
            DartPackageVersion::Exact(v) => v.to_string(),
            DartPackageVersion::Range(v) => v.to_string(),
        }
    }
}

#[inline]
fn read_file(at: &str, filename: &str) -> anyhow::Result<String> {
    let file = PathBuf::from(at).join(filename);
    if !file.exists() {
        return Err(anyhow::Error::msg(format!("missing {filename} in {at}")));
    }
    let content = std::fs::read_to_string(file)
        .map_err(|e| anyhow::Error::msg(format!("unable to read {filename} in {at}: {e:#}")))?;
    Ok(content)
}

impl PubspecLockPackage {
    pub(crate) fn installed_in(&self) -> Option<DartDependencyMode> {
        match self.dependency.as_str() {
            "direct dev" => Some(DartDependencyMode::Dev),
            "direct main" => Some(DartDependencyMode::Main),
            _ => None,
        }
    }
}

