use crate::utils::dart_repository::dart_toolchain::DartToolchain;
use crate::utils::dart_repository::pubspec::*;
use anyhow::{anyhow, bail, Context};
use cargo_metadata::{Version, VersionReq};
use log::{debug, warn};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Display;
use std::hash::Hash;
use std::path::{Path, PathBuf};

/// represents a dart / flutter repository
pub(crate) struct DartRepository {
    pub(crate) at: PathBuf,
    pub(crate) toolchain: DartToolchain,
    pub(crate) workspace_root: Option<PathBuf>,
}

impl DartRepository {
    pub(crate) fn from_path(path: &Path) -> anyhow::Result<Self> {
        debug!("Guessing toolchain the runner is run into");

        let filename = DartToolchain::manifest_filename();
        let manifest_file: PubspecYaml = read_file_and_parse_yaml(path, filename)?;
        let manifest_path = path.join(filename).to_owned();

        let toolchain = if option_hash_map_contains(
            &manifest_file.dependencies,
            &DartToolchain::Flutter.to_string(),
        ) {
            DartToolchain::Flutter
        } else {
            DartToolchain::Dart
        };

        let workspace_root = determine_workspace_root(&manifest_file, manifest_path.as_path());

        Ok(DartRepository {
            at: path.to_owned(),
            toolchain,
            workspace_root,
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
        debug!("Checking presence of {package} in {manager} at {at:?}");
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
        let at = self.workspace_root.as_ref().unwrap_or(&self.at);
        let filename = DartToolchain::lock_filename();
        debug!("Checking presence of {package} in {manager} at {at:?}");

        // We do not care about this branch
        // frb-coverage:ignore-start
        if !at.join(filename).exists() {
            log::warn!("Skip checking presence of {package} in {manager} at {at:?} since {filename} does not exist. Please check manually.");
            return Ok(());
        }
        // frb-coverage:ignore-end

        let lock_file: PubspecLock = read_file_and_parse_yaml(at, filename)?;
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
                        package, filename, e
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
            DartPackageVersion::Range(_) => {
                bail!("unexpected version range for {package} in {}", filename)
            }
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

fn determine_workspace_root(manifest_file: &PubspecYaml, manifest_path: &Path) -> Option<PathBuf> {
    debug!("Checking if {manifest_path:?} is the root of a workspace...");
    match manifest_file.resolution.as_deref() {
        Some("workspace") => {
            // Should be able to assume this path points to the manifest's dir,
            // but make sure we are robust to different inputs
            let parent_dir = if manifest_path.is_dir() {
                manifest_path.parent()
            } else {
                manifest_path.parent().and_then(|parent| parent.parent())
            };

            let parent_pubspec = parent_dir.and_then(search_for_pubspec_starting_at);

            if let Some((parent_yaml, parent_manifest_path)) = parent_pubspec {
                determine_workspace_root(&parent_yaml, &parent_manifest_path)
            } else {
                warn!("The pubspec {manifest_path:?} had resolution: workspace but no workspace root pubspec was found!");
                None
            }
        }
        Some(_) => None,
        // If the manifest we are currently reading has no `resolution: ...` then it
        // is considered the root, per: https://github.com/dart-lang/pub/blob/06e1960de9d54a6b6dce8d9a999892ef6257cda2/lib/src/entrypoint.dart#L61-L64
        None => {
            debug!("Seems like {manifest_path:?} is the workspace root pubspec");
            manifest_path.parent().map(|parent| parent.to_path_buf())
        }
    }
}

fn search_for_pubspec_starting_at(in_dir: &Path) -> Option<(PubspecYaml, PathBuf)> {
    let filename = DartToolchain::manifest_filename();
    let pubspec_path = in_dir.join(filename);
    debug!("Checking for pubspec at {pubspec_path:?}");
    if pubspec_path.exists() {
        match read_file_and_parse_yaml::<PubspecYaml>(in_dir, filename) {
            Ok(pubspec_yaml) => Some((pubspec_yaml, pubspec_path)),
            Err(e) => {
                debug!("Couldn't load pubspec: {e}");
                None
            }
        }
    } else {
        // Search all the way up recursively
        in_dir.parent().and_then(search_for_pubspec_starting_at)
    }
}

fn option_hash_map_contains<K: Hash + Eq, V: Eq>(map: &Option<HashMap<K, V>>, key: &K) -> bool {
    if let Some(map) = map.as_ref() {
        map.contains_key(key)
    } else {
        false
    }
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
