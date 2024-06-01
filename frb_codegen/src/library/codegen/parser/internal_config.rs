use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserInternalConfig {
    pub hir: ParserHirInternalConfig,
    pub mir: ParserMirInternalConfig,
}
