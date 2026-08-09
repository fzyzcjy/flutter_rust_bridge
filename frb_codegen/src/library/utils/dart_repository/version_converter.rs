use crate::utils::dart_repository::dart_repo::*;
use crate::utils::dart_repository::pubspec::*;
use cargo_metadata::{Version, VersionReq};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoDependencyVersion(String);

impl From<&DartDependencyVersion> for CargoDependencyVersion {
    /// convert from a dependency version in Dart syntax to Cargo syntax (to be used with VersionReq later on)
    ///
    /// be careful because this is where you can shoot yourself in the foot :)
    ///
    /// see module level comments for more information.
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

// TODO unused code found by codecov, maybe remove it
// impl PubspecYamlDependencyVersion {
//     pub(crate) fn version(&self) -> Option<DartDependencyVersion> {
//         match self {
//             PubspecYamlDependencyVersion::Inline(v) => Some(v.clone()),
//             PubspecYamlDependencyVersion::Multiline { version } => version.clone(),
//         }
//     }
// }
//
// impl TryFrom<&PubspecYamlDependencyVersion> for DartPackageVersion {
//     type Error = anyhow::Error;
//     fn try_from(version: &PubspecYamlDependencyVersion) -> Result<Self, Self::Error> {
//         if let Some(ref version) = version.version() {
//             return Self::try_from(version);
//         }
//         bail!("no version found")
//     }
// }

impl TryFrom<&PubspecLockPackage> for DartPackageVersion {
    type Error = anyhow::Error;
    fn try_from(dependency: &PubspecLockPackage) -> Result<Self, Self::Error> {
        Self::try_from(&dependency.version)
    }
}

impl TryFrom<&DartDependencyVersion> for DartPackageVersion {
    type Error = anyhow::Error;

    fn try_from(s: &DartDependencyVersion) -> Result<Self, Self::Error> {
        Self::try_from(&CargoDependencyVersion::from(s))
    }
}

impl TryFrom<&CargoDependencyVersion> for DartPackageVersion {
    type Error = anyhow::Error;

    fn try_from(s: &CargoDependencyVersion) -> Result<Self, Self::Error> {
        let range: [char; 4] = ['>', '<', '=', '~'];
        if s.0.contains(range) {
            let version_req = VersionReq::parse(&s.0)?;
            Ok(DartPackageVersion::Range(version_req))
        } else {
            let version = Version::parse(&s.0)?;
            Ok(DartPackageVersion::Exact(version))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dart_package_version(version: &str) -> DartPackageVersion {
        DartPackageVersion::try_from(&DartDependencyVersion(version.to_owned())).unwrap()
    }

    fn version_requirement(requirement: &str) -> VersionReq {
        VersionReq::parse(requirement).unwrap()
    }

    #[test]
    fn test_dart_dependency_version_to_cargo_dependency_version() {
        for (dart, cargo) in [("^1.2.3", "1.2.3"), ("^0.2.3", "0.2")] {
            assert_eq!(
                CargoDependencyVersion::from(&DartDependencyVersion(dart.to_owned())),
                CargoDependencyVersion(cargo.to_owned())
            );
        }
    }

    #[test]
    fn test_cargo_dependency_version_to_dart_package_version() {
        assert_eq!(
            DartPackageVersion::try_from(&CargoDependencyVersion(">=1.2.3".to_owned())).unwrap(),
            DartPackageVersion::Range(VersionReq::parse(">=1.2.3").unwrap())
        );
    }

    #[test]
    fn test_dart_stable_exact_version_uses_semver_requirement_matching() {
        let version = dart_package_version("1.2.3");

        for requirement in [
            "=1.2.3",
            ">=1.0.0",
            ">1.2.2",
            "<2.0.0",
            "^1.2.0",
            "~1.2.0",
            ">=1.0.0, <2.0.0",
        ] {
            assert!(
                version.matches_requirement(&version_requirement(requirement)),
                "{version} should satisfy {requirement}"
            );
        }

        for requirement in [">1.2.3", ">=2.0.0", "<1.2.3", "^2.0.0", "~1.3.0"] {
            assert!(
                !version.matches_requirement(&version_requirement(requirement)),
                "{version} should not satisfy {requirement}"
            );
        }
    }

    #[test]
    fn test_dart_range_version_matches_only_identical_requirement() {
        let version = dart_package_version(">=1.0.0");

        assert!(version.matches_requirement(&version_requirement(">=1.0.0")));
        assert!(!version.matches_requirement(&version_requirement(">1.0.0")));
        assert!(!version.matches_requirement(&version_requirement(">=1.0.0, <2.0.0")));
    }

    #[test]
    fn test_semver_rejects_prerelease_for_stable_lower_bound_without_override() {
        let version = Version::parse("4.0.0-dev.3").unwrap();
        let requirement = version_requirement(">=1.0.0");

        assert!(!requirement.matches(&version));
        assert!(dart_package_version("^4.0.0-dev.3").matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_satisfies_stable_requirement() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^4.0.0-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse(">=1.0.0").unwrap();

        assert!(version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_does_not_satisfy_release_boundary() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^4.0.0-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse(">=4.0.0").unwrap();

        assert!(!version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_stable_exact_version_does_not_satisfy_larger_stable_requirement() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("1.0.0".to_owned())).unwrap();
        let requirement = VersionReq::parse(">=2.0.0").unwrap();

        assert!(!version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_does_not_satisfy_exact_stable_requirement() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^1.0.1-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse("=1.0.1").unwrap();

        assert!(!version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_does_not_satisfy_abbreviated_release_boundary() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^1.0.0-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse(">=1").unwrap();

        assert!(!version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_satisfies_stable_greater_requirement() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^1.0.1-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse(">1.0.0").unwrap();

        assert!(version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_satisfies_stable_compound_requirement() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^1.1.0-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse(">=1.0.0, <2.0.0").unwrap();

        assert!(version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_satisfies_stable_upper_bound_requirement() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^1.1.0-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse("<2.0.0").unwrap();

        assert!(version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_does_not_satisfy_compound_upper_bound() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^2.0.0-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse(">=1.0.0, <2.0.0").unwrap();

        assert!(!version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_satisfies_stable_caret_requirement() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^1.0.1-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse("^1.0.0").unwrap();

        assert!(version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_satisfies_prerelease_greater_eq_requirement() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^1.0.1-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse(">=1.0.0-dev.1").unwrap();

        assert!(version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_satisfies_prerelease_caret_requirement() {
        let version =
            DartPackageVersion::try_from(&DartDependencyVersion("^1.0.1-dev.3".to_owned()))
                .unwrap();
        let requirement = VersionReq::parse("^1.0.0-dev.1").unwrap();

        assert!(version.matches_requirement(&requirement));
    }

    #[test]
    fn test_dart_prerelease_caret_version_stable_requirement_matrix() {
        for (version, requirement) in [
            ("^1.0.1-dev.3", ">=1.0.0"),
            ("^1.0.1-dev.3", ">1.0.0"),
            ("^1.0.1-dev.3", "^1.0.0"),
            ("^1.0.1-dev.3", "~1.0.0"),
            ("^1.2.0-dev.3", ">=1.0.0, <2.0.0"),
            ("^1.2.0-dev.3", "<2.0.0"),
            ("^0.2.4-dev.3", ">=0.2.0, <0.3.0"),
        ] {
            let version = dart_package_version(version);
            let requirement = version_requirement(requirement);

            assert!(
                version.matches_requirement(&requirement),
                "{version} should satisfy {requirement}"
            );
        }
    }

    #[test]
    fn test_dart_prerelease_caret_version_rejects_non_matching_stable_requirement_matrix() {
        for (version, requirement) in [
            ("^1.0.1-dev.3", "=1.0.1"),
            ("^1.0.1-dev.3", ">=1.0.1"),
            ("^1.0.1-dev.3", ">1.0.1"),
            ("^1.0.1-dev.3", "<1.0.1"),
            ("^1.0.1-dev.3", "^1.0.1"),
            ("^1.2.0-dev.3", ">=1.0.0, <1.2.0"),
            ("^2.0.0-dev.3", ">=1.0.0, <2.0.0"),
            ("^0.3.0-dev.3", ">=0.2.0, <0.3.0"),
        ] {
            let version = dart_package_version(version);
            let requirement = version_requirement(requirement);

            assert!(
                !version.matches_requirement(&requirement),
                "{version} should not satisfy {requirement}"
            );
        }
    }

    #[test]
    fn test_dart_prerelease_caret_version_prerelease_requirement_matrix() {
        for (version, requirement, expected) in [
            ("^1.0.1-dev.3", ">=1.0.0-dev.1", true),
            ("^1.0.1-dev.3", "^1.0.0-dev.1", true),
            ("^1.0.1-dev.3", ">=1.0.1-dev.3", true),
            ("^1.0.1-dev.3", ">1.0.1-dev.3", false),
            ("^1.0.1-dev.3", "=1.0.1-dev.3", true),
            ("^1.0.1-dev.3", "=1.0.1-dev.4", false),
        ] {
            let version = dart_package_version(version);
            let requirement = version_requirement(requirement);

            assert_eq!(
                version.matches_requirement(&requirement),
                expected,
                "{version} match result for {requirement}"
            );
        }
    }
}
