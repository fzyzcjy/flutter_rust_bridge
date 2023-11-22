use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct DumperInternalConfig {
    pub(crate) dump_contents: Vec<ConfigDumpContent>,
    pub(crate) dump_directory: PathBuf,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, ValueEnum, enum_iterator::Sequence,
)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDumpContent {
    Config,
    Ir,
    Spec,
}
