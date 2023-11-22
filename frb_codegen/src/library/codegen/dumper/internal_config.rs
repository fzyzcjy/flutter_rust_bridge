use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct DumperInternalConfig {
    pub(crate) dump_contents: Vec<ConfigDumpContent>,
    pub(crate) dump_directory: PathBuf,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    ValueEnum,
    enum_iterator::Sequence,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDumpContent {
    Config,
    SourceGraph,
    Ir,
    Spec,
}
