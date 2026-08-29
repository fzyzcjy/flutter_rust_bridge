use crate::commands::command_runner::call_shell;
use crate::library::commands::fvm::command_arg_maybe_fvm;
use crate::misc::FvmInstallMode;
use crate::{command_args, command_run};
use std::fmt::Display;
use std::path::PathBuf;

/// represents dart or flutter toolchain
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DartToolchain {
    Dart,
    Flutter,
}

impl Display for DartToolchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter",
        }
        .to_string();
        write!(f, "{str}")
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

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn as_run_command(&self) -> Vec<PathBuf> {
        match self {
            DartToolchain::Dart => command_args!("dart"),
            DartToolchain::Flutter => command_args!("flutter", "pub"),
        }
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn available(&self) -> bool {
        let toolchain = match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter",
        };
        command_run!(call_shell[None, None], ?command_arg_maybe_fvm(None, FvmInstallMode::Normal), toolchain, "--version")
            .unwrap()
            .status
            .success()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Displays the executable name for each supported toolchain.
    fn displays_each_toolchain_name() {
        assert_eq!(DartToolchain::Dart.to_string(), "dart");
        assert_eq!(DartToolchain::Flutter.to_string(), "flutter");
    }

    #[test]
    /// Uses the standard manifest and lock filenames for both toolchains.
    fn uses_standard_pubspec_filenames() {
        assert_eq!(DartToolchain::manifest_filename(), "pubspec.yaml");
        assert_eq!(DartToolchain::lock_filename(), "pubspec.lock");
    }

    #[test]
    /// Builds commands appropriate for Dart and Flutter package operations.
    fn builds_toolchain_specific_run_commands() {
        assert_eq!(
            DartToolchain::Dart.as_run_command(),
            vec![PathBuf::from("dart")]
        );
        assert_eq!(
            DartToolchain::Flutter.as_run_command(),
            vec![PathBuf::from("flutter"), PathBuf::from("pub")]
        );
    }
}
