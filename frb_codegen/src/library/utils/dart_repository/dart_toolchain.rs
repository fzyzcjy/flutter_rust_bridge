use crate::commands::command_runner::call_shell;
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
        write!(f, "{}", str)
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
        command_run!(call_shell[None, None], toolchain, "--version")
            .unwrap()
            .status
            .success()
    }
}
