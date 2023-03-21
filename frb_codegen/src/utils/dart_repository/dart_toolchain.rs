use crate::utils::command_runner::call_shell;
use std::path::PathBuf;
use crate::{command_args, command_run};

/// represents dart or flutter toolchain
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DartToolchain {
    Dart,
    Flutter,
}

impl ToString for DartToolchain {
    fn to_string(&self) -> String {
        match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter",
        }.to_string()
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

    pub(crate) fn as_run_command(&self) -> Vec<PathBuf> {
        match self {
            DartToolchain::Dart => command_args!("dart"),
            DartToolchain::Flutter => command_args!("flutter", "pub"),
        }
    }

    pub(crate) fn available(&self) -> bool {
        let toolchain = match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter",
        };
        command_run!(call_shell[None], toolchain, "--version")
            .unwrap()
            .status
            .success()
    }
}

