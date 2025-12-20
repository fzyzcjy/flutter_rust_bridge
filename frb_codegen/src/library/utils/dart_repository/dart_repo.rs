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
    /// Where the dart repository is located
    pub(crate) at: PathBuf,
    pub(crate) toolchain: DartToolchain,
    /// This is the root of the workspace.
    pub(crate) workspace_root: PathBuf,
    pub(crate) is_workspace: bool,
}

impl DartRepository {
    pub(crate) fn from_path(path: &Path) -> anyhow::Result<Self> {
        debug!("Guessing toolchain the runner is run into");

        let filename = DartToolchain::manifest_filename();
        let manifest_path = path.join(filename).to_owned();
        let manifest_file: PubspecYaml = read_file_and_parse_yaml(path, filename)?;

        let toolchain = if option_hash_map_contains(
            &manifest_file.dependencies,
            &DartToolchain::Flutter.to_string(),
        ) {
            DartToolchain::Flutter
        } else {
            DartToolchain::Dart
        };

        // If we don't find a workspace, assume we are the root
        let workspace_root = determine_workspace_root(&manifest_file, manifest_path.as_path())
            .unwrap_or(path.to_owned());

        let is_workspace = if workspace_root != path {
            true
        } else {
            // If we are already the root, perform a sanity check that at least have
            // a workspace list (even if empty) meaning we have the `workspace:` field.
            manifest_file.workspace.is_some()
        };

        if is_workspace {
            debug!("{path:?} is in a workspace, with root at {workspace_root:?}");
        } else {
            debug!("{path:?} is not in a workspace");
        }

        Ok(DartRepository {
            at: path.to_owned(),
            toolchain,
            workspace_root,
            is_workspace,
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
        // The pubspec.lock modes are different than the pubspec.yaml dependency categories
        let lock_manager: DartDependencyLockMode = manager.into();

        // If we are a workspace, dev dependencies are commonly saved
        // as transitive in the root pubspec.lock, since the root pubspec.yaml
        // does not directly list the dependency, but the child package does.
        // So we need to check for both direct dev and transitive if we are a
        // workspace.
        let workspace_lock_manager = if self.is_workspace {
            Some(DartDependencyLockMode::Transitive)
        } else {
            None
        };

        let at = if self.is_workspace {
            &self.workspace_root
        } else {
            &self.at
        };

        let filename = DartToolchain::lock_filename();

        let locked_as_string = if let Some(workspace_lock_manager) = workspace_lock_manager {
            format!("'{lock_manager}' or '{workspace_lock_manager}'")
        } else {
            format!("'{lock_manager}'")
        };
        debug!("Checking presence of {package} locked as {locked_as_string} at {at:?}");

        // We do not care about this branch
        // frb-coverage:ignore-start
        if !at.join(filename).exists() {
            log::warn!("Skip checking presence of {package} locked as {locked_as_string} at {at:?} since {filename} does not exist. Please check manually.");
            return Ok(());
        }
        // frb-coverage:ignore-end

        let lock_file: PubspecLock = read_file_and_parse_yaml(at, filename)?;
        let dependency = lock_file.packages.get(package);
        let version = match dependency {
            Some(dependency) => {
                let pm = dependency.installed_in();

                let satisfies_lock = if let Some(locked_mode) = pm {
                    // We satisfy the lock mode if it matches the requested lock mode,
                    // or if we are a workspace and the locked mode is transitive
                    locked_mode == lock_manager
                        || workspace_lock_manager.is_some_and(|m| m == locked_mode)
                } else {
                    false
                };

                if !satisfies_lock {
                    // This will stop the whole generator and tell the users, so we do not care about testing it
                    // frb-coverage:ignore-start
                    return Err(error_invalid_dep(package, manager, requirement));
                    // frb-coverage:ignore-end
                }
                DartPackageVersion::try_from(dependency).map_err(|e| {
                    // This will stop the whole generator and tell the users, so we do not care about testing it
                    // frb-coverage:ignore-start
                    anyhow::Error::msg(format!(
                        "unable to parse {package} version in {filename}: {e:#}"
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
        // Only add --enable-experiment=native-assets for Dart < 3.2
        // In Dart 3.2+, native assets are stable and the flag causes errors
        // See: https://pub.dev/packages/ffigen and https://www.simonbinder.eu/posts/native_assets/
        if self.at.join("build.dart").exists() {
            if let Some(version) = self.toolchain.dart_sdk_version() {
                // Check if version < 3.2.0
                let dart_3_2 = Version::parse("3.2.0").unwrap();
                if version < dart_3_2 {
                    return vec!["--enable-experiment=native-assets".to_owned()];
                }
            } else {
                // If we can't detect version, be conservative and don't add the flag
                // (modern Dart will work without it, old Dart might need manual config)
                log::warn!("Could not detect Dart SDK version, skipping --enable-experiment=native-assets flag");
            }
        }
        vec![]
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

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum DartDependencyLockMode {
    /// Appear in `pubspec.lock` as `direct main`
    Main,
    /// Appear in `pubspec.lock` as `direct dev`
    Dev,
    /// Appear in `pubspec.lock` as `transitive`
    Transitive,
}

impl Display for DartDependencyLockMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DartDependencyLockMode::Main => write!(f, "direct main"),
            DartDependencyLockMode::Dev => write!(f, "direct dev"),
            DartDependencyLockMode::Transitive => write!(f, "transitive"),
        }
    }
}

impl From<DartDependencyMode> for DartDependencyLockMode {
    fn from(mode: DartDependencyMode) -> Self {
        match mode {
            DartDependencyMode::Main => DartDependencyLockMode::Main,
            DartDependencyMode::Dev => DartDependencyLockMode::Dev,
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
        write!(f, "{str}")
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
    debug!("Checking if {manifest_path:?} is a workspace root pubspec...");
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
        None => Some(manifest_path.parent().unwrap().to_owned()),
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
    pub(crate) fn installed_in(&self) -> Option<DartDependencyLockMode> {
        match self.dependency.as_str() {
            "direct dev" => Some(DartDependencyLockMode::Dev),
            "direct main" => Some(DartDependencyLockMode::Main),
            "transitive" => Some(DartDependencyLockMode::Transitive),
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
