use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub(crate) struct DumperInternalConfig {
    pub(crate) dump_contents: Vec<ConfigDumpContent>,
    pub(crate) dump_directory: PathBuf,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, ValueEnum, EnumIter, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDumpContent {
    Config,
    Source,
    Hir,
    Mir,
    GeneratorInfo,
    GeneratorSpec,
    GeneratorText,
}

#[cfg(test)]
mod tests {
    use super::{ConfigDumpContent, DumperInternalConfig};
    use std::path::PathBuf;

    /// Serializes dump content names with their documented snake-case spelling.
    #[test]
    fn serializes_dump_content_names() {
        assert_eq!(
            serde_json::to_string(&ConfigDumpContent::GeneratorSpec).unwrap(),
            "\"generator_spec\""
        );
        assert_eq!(
            ConfigDumpContent::GeneratorText.to_string(),
            "GeneratorText"
        );
    }

    /// Defaults to no dump content and an empty output directory.
    #[test]
    fn defaults_to_an_inert_dumper_configuration() {
        assert_eq!(
            DumperInternalConfig::default(),
            DumperInternalConfig {
                dump_contents: vec![],
                dump_directory: PathBuf::new(),
            }
        );
    }
}
