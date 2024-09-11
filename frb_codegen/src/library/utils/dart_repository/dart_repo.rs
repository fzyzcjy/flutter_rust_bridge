use crate::utils::dart_repository::dart_toolchain::DartToolchain;
use crate::utils::dart_repository::pubspec::*;
use anyhow::{anyhow, bail, Context};
use cargo_metadata::{Version, VersionReq};
use log::debug;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;
use std::fmt::Display;
use std::path::{Path, PathBuf};

/// represents a dart / flutter repository
pub(crate) struct DartRepository {
    pub(crate) at: PathBuf,
    pub(crate) toolchain: DartToolchain,
}

impl DartRepository {
    pub(crate) fn from_path(path: &Path) -> anyhow::Result<Self> {
        debug!("Guessing toolchain the runner is run into");

        let lock_file: PubspecYaml =
            read_file_and_parse_yaml(path, DartToolchain::lock_filename())?;

        let toolchain = if let Some(dependencies) = lock_file.dependencies.as_ref() {
            if dependencies.contains_key(&DartToolchain::Flutter.to_string()) {
                DartToolchain::Flutter
            } else {
                DartToolchain::Dart
            }
        } else {
            DartToolchain::Dart
        };

        Ok(DartRepository {
            at: path.to_owned(),
            toolchain,
        })
    }

    /// check whether the toolchain is available from the CLI
    pub(crate) fn toolchain_available(&self) -> bool {
        self.toolchain.available()
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    pub(crate) fn has_specified_and_installed(
        &self,
        package: &str,
        manager: DartDependencyMode,
        requirement: &VersionReq,
    ) -> anyhow::Result<()> {
        // frb-coverage:ignore-end
        self.has_specified(package, manager, requirement)?;
        self.has_installed(package, manager, requirement)?;
        Ok(())
    }

    /// check whether a package has been correctly specified in pubspec.yaml
    pub(crate) fn has_specified(
        &self,
        package: &str,
        manager: DartDependencyMode,
        requirement: &VersionReq,
    ) -> anyhow::Result<()> {
        let at = &self.at;
        debug!(
            "Checking presence of {} in {} at {:?}",
            package, manager, at
        );
        let manifest_file: PubspecYaml =
            read_file_and_parse_yaml(at, DartToolchain::manifest_filename())?;
        let deps = match manager {
            DartDependencyMode::Main => manifest_file.dependencies.unwrap_or_default(),
            DartDependencyMode::Dev => manifest_file.dev_dependencies.unwrap_or_default(),
        };
        if !deps.contains_key(package) {
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            Err(error_missing_dep(package, manager, requirement))?;
            // frb-coverage:ignore-end
        }
        Ok(())
    }

    /// check whether a package has been correctly pinned in pubspec.lock
    pub(crate) fn has_installed(
        &self,
        package: &str,
        manager: DartDependencyMode,
        requirement: &VersionReq,
    ) -> anyhow::Result<()> {
        let at = &self.at;
        debug!(
            "Checking presence of {} in {} at {:?}",
            package, manager, at
        );
        let lock_file: PubspecLock = read_file_and_parse_yaml(at, DartToolchain::lock_filename())?;
        let dependency = lock_file.packages.get(package);
        let version = match dependency {
            Some(dependency) => {
                let pm = dependency.installed_in();
                if pm.as_ref() != Some(&manager) {
                    // This will stop the whole generator and tell the users, so we do not care about testing it
                    // frb-coverage:ignore-start
                    return Err(error_invalid_dep(package, manager, requirement));
                    // frb-coverage:ignore-end
                }
                DartPackageVersion::try_from(dependency).map_err(|e| {
                    // This will stop the whole generator and tell the users, so we do not care about testing it
                    // frb-coverage:ignore-start
                    anyhow::Error::msg(format!(
                        "unable to parse {} version in {}: {:#}",
                        package,
                        DartToolchain::lock_filename(),
                        e
                    ))
                    // frb-coverage:ignore-end
                })?
            }
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            None => return Err(error_missing_dep(package, manager, requirement)),
            // frb-coverage:ignore-end
        };

        match version {
            DartPackageVersion::Exact(ref v) if requirement.matches(v) => Ok(()),
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            DartPackageVersion::Range(_) => bail!(
                "unexpected version range for {package} in {}",
                DartToolchain::lock_filename()
            ),
            _ => Err(error_invalid_dep(package, manager, requirement)),
            // frb-coverage:ignore-end
        }
    }

    pub(crate) fn command_extra_args(&self) -> Vec<String> {
        if self.at.join("build.dart").exists() {
            vec!["--enable-experiment=native-assets".to_owned()]
        } else {
            vec![]
        }
    }
}

// This will stop the whole generator and tell the users, so we do not care about testing it
// frb-coverage:ignore-start
fn error_missing_dep(
    package: &str,
    manager: DartDependencyMode,
    requirement: &VersionReq,
) -> anyhow::Error {
    anyhow!("MissingDep: Please add {package} to your {manager}. (version {requirement})")
}
// frb-coverage:ignore-end

// This will stop the whole generator and tell the users, so we do not care about testing it
// frb-coverage:ignore-start
fn error_invalid_dep(
    package: &str,
    manager: DartDependencyMode,
    requirement: &VersionReq,
) -> anyhow::Error {
    anyhow!(
        "InvalidDep: Please update version of {package} in your {manager}. (version {requirement})"
    )
}
// frb-coverage:ignore-end

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

impl Display for DartPackageVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DartPackageVersion::Exact(v) => v.to_string(),
            DartPackageVersion::Range(v) => v.to_string(),
        };
        write!(f, "{}", str)
    }
}

fn read_file(at: &Path, filename: &str) -> anyhow::Result<String> {
    let file = at.join(filename);
    if !file.exists() {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!("missing {filename} in {at:?}");
        // frb-coverage:ignore-end
    }
    let content = std::fs::read_to_string(file)
        .with_context(|| format!("unable to read {filename} in {at:?}"))?;
    Ok(content)
}

fn read_file_and_parse_yaml<T: DeserializeOwned>(at: &Path, filename: &str) -> anyhow::Result<T> {
    let file = read_file(at, filename)?;
    let file: T = serde_yaml::from_str(&file).map_err(|e| {
        // frb-coverage:ignore-start
        // This will stop the whole generator and tell the users, so we do not care about testing it
        anyhow!("Unable to parse {filename} in {at:?}: {e:#}")
        // frb-coverage:ignore-end
    })?;
    Ok(file)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_display() {
        assert_eq!(
            format!("{}", DartDependencyMode::Main),
            "dependencies".to_owned()
        );
        assert_eq!(
            format!("{}", DartDependencyMode::Dev),
            "dev_dependencies".to_owned()
        );

        assert_eq!(
            format!(
                "{}",
                DartPackageVersion::Exact(Version::parse("1.0.0").unwrap())
            ),
            "1.0.0".to_owned()
        );
        assert_eq!(
            format!(
                "{}",
                DartPackageVersion::Range(VersionReq::parse(">=1.0.0").unwrap())
            ),
            ">=1.0.0".to_owned()
        );
    }
}
