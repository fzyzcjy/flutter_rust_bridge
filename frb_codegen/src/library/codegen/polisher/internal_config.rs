use crate::misc::FvmInstallMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PolisherInternalConfig {
    pub duplicated_c_output_path: Vec<PathBuf>,
    pub dart_format_line_length: u32,
    pub dart_format: bool,
    pub dart_fix: bool,
    pub rust_format: bool,
    pub add_mod_to_lib: bool,
    pub build_runner: bool,
    pub web_enabled: bool,
    pub dart_output: PathBuf,
    pub dart_root: PathBuf,
    pub rust_crate_dir: PathBuf,
    pub rust_output_path: PathBuf,
    pub c_output_path: Option<PathBuf>,
    pub enable_auto_upgrade: bool,
    pub fvm_install_mode: FvmInstallMode,
}

#[cfg(test)]
mod tests {
    use super::PolisherInternalConfig;
    use crate::misc::FvmInstallMode;
    use std::path::PathBuf;

    /// Round-trips the polisher declaration without invoking polishing commands.
    #[test]
    fn test_polisher_internal_config_is_a_serde_declaration_carrier() {
        let config = PolisherInternalConfig {
            duplicated_c_output_path: vec![PathBuf::from("duplicate.h")],
            dart_format_line_length: 100,
            dart_format: true,
            dart_fix: true,
            rust_format: true,
            add_mod_to_lib: true,
            build_runner: true,
            web_enabled: true,
            dart_output: PathBuf::from("dart/output.dart"),
            dart_root: PathBuf::from("dart"),
            rust_crate_dir: PathBuf::from("rust"),
            rust_output_path: PathBuf::from("rust/src/bridge.rs"),
            c_output_path: Some(PathBuf::from("bridge.h")),
            enable_auto_upgrade: true,
            fvm_install_mode: FvmInstallMode::Skip,
        };

        let encoded = serde_json::to_string(&config).unwrap();
        assert_eq!(
            serde_json::from_str::<PolisherInternalConfig>(&encoded).unwrap(),
            config
        );
    }
}
