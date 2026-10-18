pub(crate) mod consts;

use serde::{Deserialize, Serialize};

/// Please refer to `TemplateArg` for doc
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Template {
    App,
    Plugin,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IntegrationBackend {
    Cargokit,
    NativeAssets,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum FvmInstallMode {
    Normal,
    Skip,
}

impl FvmInstallMode {
    pub fn from_skip_fvm_install(skip_fvm_install: bool) -> Self {
        if skip_fvm_install {
            Self::Skip
        } else {
            Self::Normal
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ToolInstallMode {
    Normal,
    Skip,
}

impl ToolInstallMode {
    pub fn from_skip_tool_install(skip_tool_install: bool) -> Self {
        if skip_tool_install {
            Self::Skip
        } else {
            Self::Normal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CLI skip flags select the matching FVM install behavior.
    #[test]
    fn from_skip_fvm_install_maps_cli_flag_to_mode() {
        assert_eq!(
            FvmInstallMode::from_skip_fvm_install(false),
            FvmInstallMode::Normal
        );
        assert_eq!(
            FvmInstallMode::from_skip_fvm_install(true),
            FvmInstallMode::Skip
        );
    }

    /// CLI skip flags select the matching Tool install behavior.
    #[test]
    fn from_skip_tool_install_maps_cli_flag_to_mode() {
        assert_eq!(
            ToolInstallMode::from_skip_tool_install(false),
            ToolInstallMode::Normal
        );
        assert_eq!(
            ToolInstallMode::from_skip_tool_install(true),
            ToolInstallMode::Skip
        );
    }
}
