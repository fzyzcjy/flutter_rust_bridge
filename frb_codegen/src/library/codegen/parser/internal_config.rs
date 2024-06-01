use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::reader::internal_config::ParserReaderInternalConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserInternalConfig {
    pub reader: ParserReaderInternalConfig,
    pub hir: ParserHirInternalConfig,
    pub mir: ParserMirInternalConfig,
}
