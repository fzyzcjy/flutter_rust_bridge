pub(crate) mod consts;

/// Please refer to `TemplateArg` for doc
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Template {
    App,
    Plugin,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FvmInstallMode {
    Normal,
    Skip,
}

impl From<bool> for FvmInstallMode {
    fn from(skip_fvm_install: bool) -> Self {
        if skip_fvm_install {
            Self::Skip
        } else {
            Self::Normal
        }
    }
}
