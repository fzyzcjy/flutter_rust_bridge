use crate::codegen::generator::misc::target::TargetOrCommonMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct DumperInternalConfig {
    pub(crate) dump_config: bool,
    pub(crate) dump_ir: bool,
    pub(crate) dump_directory: PathBuf,
}
