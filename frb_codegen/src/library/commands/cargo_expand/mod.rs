mod pseudo;
mod real;

use crate::codegen::dumper::Dumper;
use crate::utils::crate_name::CrateName;
use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use anyhow::Result;
use log::debug;
use std::env;
use std::path::Path;

pub(crate) fn run_cargo_expand(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
    dumper: &Dumper,
    features: Option<&[String]>,
) -> Result<syn::File> {
    if can_execute_real(rust_crate_dir)? {
        real::run(rust_crate_dir, interest_crate_name, dumper, features)
    } else {
        pseudo::run(rust_crate_dir, interest_crate_name)
    }
}

fn can_execute_real(rust_crate_dir: &Path) -> anyhow::Result<bool> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    debug!("run_cargo_expand manifest_dir={manifest_dir} rust_crate_dir={rust_crate_dir:?}");
    Ok(manifest_dir.is_empty()
        || normalize_windows_unc_path(&path_to_string(rust_crate_dir)?)
            != normalize_windows_unc_path(&manifest_dir))
}

#[cfg(test)]
mod tests {
    use super::can_execute_real;
    use serial_test::serial;
    use std::env;
    use std::ffi::OsString;
    use std::path::Path;

    struct EnvVarGuard {
        previous_value: Option<OsString>,
    }

    impl EnvVarGuard {
        fn set(key: &str, value: Option<&str>) -> Self {
            let previous_value = env::var_os(key);
            match value {
                Some(value) => env::set_var(key, value),
                None => env::remove_var(key),
            }
            Self { previous_value }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            match &self.previous_value {
                Some(value) => env::set_var("CARGO_MANIFEST_DIR", value),
                None => env::remove_var("CARGO_MANIFEST_DIR"),
            }
        }
    }

    /// Uses real cargo-expand when Cargo did not provide a manifest directory.
    #[test]
    #[serial]
    fn test_can_execute_real_when_manifest_dir_is_missing() {
        let _guard = EnvVarGuard::set("CARGO_MANIFEST_DIR", None);
        assert!(can_execute_real(Path::new("/tmp/another-crate")).unwrap());
    }

    /// Uses pseudo expansion for the same normalized manifest directory.
    #[test]
    #[serial]
    fn test_can_execute_real_when_manifest_dir_matches_after_normalization() {
        let _guard = EnvVarGuard::set("CARGO_MANIFEST_DIR", Some(r"\\?\C:\crate"));
        assert!(!can_execute_real(Path::new(r"C:\crate")).unwrap());
    }

    /// Uses real cargo-expand for a distinct manifest directory.
    #[test]
    #[serial]
    fn test_can_execute_real_when_manifest_dir_differs() {
        let _guard = EnvVarGuard::set("CARGO_MANIFEST_DIR", Some("/tmp/other-crate"));
        assert!(can_execute_real(Path::new("/tmp/crate")).unwrap());
    }
}
