use crate::utils::dart_repository::dart_toolchain::DartToolchain;
use crate::utils::dart_repository::pubspec::*;
use anyhow::{anyhow, bail, Context};
use cargo_metadata::{Version, VersionReq};
use log::debug;
use std::convert::TryFrom;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

/// represents a dart / flutter repository
pub(crate) struct DartRepository {
    pub(crate) at: PathBuf,
    pub(crate) toolchain: DartToolchain,
}

// TODO it is from path, not from str
impl FromStr for DartRepository {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("Guessing toolchain the runner is run into");
        let filename = DartToolchain::lock_filename();
        let lock_file = read_file(s, filename)?;
        let lock_file: PubspecLock = serde_yaml::from_str(&lock_file)
            .map_err(|e| anyhow!("unable to parse {filename} in {s}: {e:#}"))?;
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
        let at = self.at.to_str().unwrap();
        debug!("Checking presence of {} in {} at {}", package, manager, at);
        let manifest_file = read_file(at, DartToolchain::manifest_filename())?;
        let manifest_file: PubspecYaml = serde_yaml::from_str(&manifest_file).map_err(|e| {
            // frb-coverage:ignore-start
            // This will stop the whole generator and tell the users, so we do not care about testing it
            anyhow!(
                "unable to parse {} in {at}: {e:#}",
                DartToolchain::manifest_filename()
            )
            // frb-coverage:ignore-end
        })?;
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
        let at = self.at.to_str().unwrap();
        debug!("Checking presence of {} in {} at {}", package, manager, at);
        let lock_file = read_file(at, DartToolchain::lock_filename())?;
        let lock_file: PubspecLock = serde_yaml::from_str(&lock_file).map_err(|e| {
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            anyhow!(
                "unable to parse {} in {at}: {e:#}",
                DartToolchain::lock_filename()
            )
            // frb-coverage:ignore-end
        })?;
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

#[inline]
fn read_file(at: &str, filename: &str) -> anyhow::Result<String> {
    let file = PathBuf::from(at).join(filename);
    if !file.exists() {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!("missing {filename} in {at}");
        // frb-coverage:ignore-end
    }
    let content = std::fs::read_to_string(file)
        .with_context(|| format!("unable to read {filename} in {at}"))?;
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
