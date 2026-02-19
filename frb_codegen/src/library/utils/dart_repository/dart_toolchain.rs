use crate::commands::command_runner::call_shell;
use crate::library::commands::fvm::command_arg_maybe_fvm;
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
        command_run!(call_shell[None, None], ?command_arg_maybe_fvm(None), toolchain, "--version")
            .unwrap()
            .status
            .success()
    }

    /// Get the Dart SDK version, returning None if detection fails
    pub(crate) fn dart_sdk_version(&self) -> Option<cargo_metadata::Version> {
        let output = command_run!(call_shell[None, None], ?command_arg_maybe_fvm(None), "dart", "--version")
            .ok()?;

        if !output.status.success() {
            return None;
        }

        // Parse output like: "Dart SDK version: 3.10.0 (stable) ..."
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let version_output = if !stderr.is_empty() { stderr } else { stdout };

        // Extract version number
        version_output
            .split_whitespace()
            .find_map(|word| {
                // Look for a word that looks like a version (contains digits and dots)
                if word.contains('.') && word.chars().any(|c| c.is_ascii_digit()) {
                    // Remove potential trailing characters like ')' or ','
                    let clean = word.trim_end_matches(|c: char| !c.is_ascii_digit() && c != '.');
                    cargo_metadata::Version::parse(clean).ok()
                } else {
                    None
                }
            })
    }
}
