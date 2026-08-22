use crate::codegen::dumper::internal_config::{ConfigDumpContent, DumperInternalConfig};
use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::path_texts::PathTexts;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::utils::file_utils::create_dir_all_and_write;
use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use convert_case::{Case, Casing};
use log::debug;
use pathdiff::diff_paths;
use serde::Serialize;
use std::path::Path;
use strum::IntoEnumIterator;

pub(super) mod internal_config;

#[derive(Clone)]
pub(crate) struct Dumper<'a> {
    config: &'a DumperInternalConfig,
    content: Option<ConfigDumpContent>,
    name_prefix: String,
}

impl<'a> Dumper<'a> {
    pub(crate) fn new(config: &'a DumperInternalConfig) -> Self {
        Self {
            config,
            content: None,
            name_prefix: "".to_string(),
        }
    }

    pub(crate) fn with_content(&self, content: ConfigDumpContent) -> Self {
        Self {
            content: Some(content),
            ..self.clone()
        }
    }

    pub(crate) fn with_add_name_prefix(&self, add_name_prefix: &str) -> Self {
        Self {
            name_prefix: format!("{}{}", self.name_prefix, add_name_prefix),
            ..self.clone()
        }
    }

    pub(crate) fn dump<T: Serialize>(&self, name: &str, data: &T) -> anyhow::Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }

        self.dump_str(name, &serde_json::to_string_pretty(data)?)
    }

    pub(crate) fn dump_path_texts(
        &self,
        partial_name: &str,
        path_texts: &PathTexts,
        base_dir: &Path,
    ) -> anyhow::Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }

        for path_text in path_texts.0.iter() {
            self.dump_str(
                &format!(
                    "{partial_name}/{}",
                    path_to_string(
                        &diff_paths(&path_text.path, base_dir).context("cannot diff path")?
                    )?
                ),
                &path_text.text.all_code(),
            )?;
        }

        Ok(())
    }

    pub(crate) fn dump_acc(
        &self,
        partial_name: &str,
        extension: &str,
        acc: &Acc<Option<String>>,
    ) -> anyhow::Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }

        for target in TargetOrCommon::iter() {
            self.dump_str(
                &format!("{partial_name}/{target}.{extension}"),
                &acc[target].clone().unwrap_or_default(),
            )?;
        }

        Ok(())
    }

    pub(crate) fn dump_str(&self, partial_name: &str, text: &str) -> anyhow::Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }

        let name = format!("{}{}", self.name_prefix, partial_name);
        let path = (self.config.dump_directory)
            .join(self.content.unwrap().to_string().to_case(Case::Snake))
            .join(&name);
        debug!("Dumping {name} into {path:?}");

        create_dir_all_and_write(path, text)
    }

    fn is_enabled(&self) -> bool {
        self.config.dump_contents.contains(&self.content.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigDumpContent, Dumper, DumperInternalConfig};
    use crate::codegen::generator::acc::Acc;
    use std::fs;
    use tempfile::tempdir;

    /// Leaves the filesystem untouched when the selected dump content is disabled.
    #[test]
    fn skips_dumping_when_content_is_disabled() -> anyhow::Result<()> {
        let directory = tempdir()?;
        let config = DumperInternalConfig {
            dump_contents: vec![],
            dump_directory: directory.path().to_owned(),
        };

        Dumper::new(&config)
            .with_content(ConfigDumpContent::Config)
            .dump_str("input.json", "ignored")?;

        assert!(fs::read_dir(directory.path())?.next().is_none());
        Ok(())
    }

    /// Writes enabled dumps beneath the content directory with accumulated prefixes.
    #[test]
    fn writes_enabled_dump_with_content_and_prefix() -> anyhow::Result<()> {
        let directory = tempdir()?;
        let config = DumperInternalConfig {
            dump_contents: vec![ConfigDumpContent::GeneratorText],
            dump_directory: directory.path().to_owned(),
        };

        Dumper::new(&config)
            .with_content(ConfigDumpContent::GeneratorText)
            .with_add_name_prefix("api/")
            .with_add_name_prefix("dart/")
            .dump_str("output.dart", "generated")?;

        assert_eq!(
            fs::read_to_string(directory.path().join("generator_text/api/dart/output.dart"))?,
            "generated"
        );
        Ok(())
    }

    /// Serializes public dump data as pretty JSON in its configured content directory.
    #[test]
    fn serializes_public_dump_data_as_pretty_json() -> anyhow::Result<()> {
        let directory = tempdir()?;
        let config = DumperInternalConfig {
            dump_contents: vec![ConfigDumpContent::Config],
            dump_directory: directory.path().to_owned(),
        };

        Dumper::new(&config)
            .with_content(ConfigDumpContent::Config)
            .dump("config.json", &serde_json::json!({"answer": 42}))?;

        assert_eq!(
            fs::read_to_string(directory.path().join("config/config.json"))?,
            "{\n  \"answer\": 42\n}"
        );
        Ok(())
    }

    /// Writes common and target accumulator slots, defaulting absent values to empty text.
    #[test]
    fn dumps_all_accumulator_targets_with_empty_missing_values() -> anyhow::Result<()> {
        let directory = tempdir()?;
        let config = DumperInternalConfig {
            dump_contents: vec![ConfigDumpContent::GeneratorText],
            dump_directory: directory.path().to_owned(),
        };
        let acc = Acc {
            common: Some("common".to_owned()),
            io: None,
            web: Some("web".to_owned()),
        };

        Dumper::new(&config)
            .with_content(ConfigDumpContent::GeneratorText)
            .dump_acc("generated", "dart", &acc)?;

        let output_directory = directory.path().join("generator_text/generated");
        assert_eq!(
            fs::read_to_string(output_directory.join("Common.dart"))?,
            "common"
        );
        assert_eq!(fs::read_to_string(output_directory.join("Io.dart"))?, "");
        assert_eq!(
            fs::read_to_string(output_directory.join("Web.dart"))?,
            "web"
        );
        Ok(())
    }
}
