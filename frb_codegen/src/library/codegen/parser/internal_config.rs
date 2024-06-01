use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserInternalConfig {
    pub hir: ParserHirInternalConfig,
    pub mir: ParserMirInternalConfig,
    pub rust_crate_dir: PathBuf,
}
