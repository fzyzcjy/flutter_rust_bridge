use crate::utils::command_runner::call_shell;
use std::path::PathBuf;
use crate::{args, run};

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
            DartToolchain::Dart => args!("dart"),
            DartToolchain::Flutter => args!("flutter", "pub"),
        }
    }

    pub(crate) fn available(&self) -> bool {
        let toolchain = match self {
            DartToolchain::Dart => "dart",
            DartToolchain::Flutter => "flutter",
        };
        run!(call_shell[None], toolchain, "--version")
            .unwrap()
            .status
            .success()
    }
}

