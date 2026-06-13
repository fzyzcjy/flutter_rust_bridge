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

#[cfg(test)]
mod tests {
    use super::*;

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
}
