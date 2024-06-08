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

pub(crate) struct Dumper<'a>(pub &'a DumperInternalConfig);

impl Dumper<'_> {
    pub(crate) fn dump<T: Serialize>(
        &self,
        content: ConfigDumpContent,
        name: &str,
        data: &T,
    ) -> anyhow::Result<()> {
        if !self.is_enabled(content) {
            return Ok(());
        }

        self.dump_str(content, name, &serde_json::to_string_pretty(data)?)
    }

    pub(crate) fn dump_path_texts(
        &self,
        content: ConfigDumpContent,
        partial_name: &str,
        path_texts: &PathTexts,
        base_dir: &Path,
    ) -> anyhow::Result<()> {
        if !self.is_enabled(content) {
            return Ok(());
        }

        for path_text in path_texts.0.iter() {
            self.dump_str(
                content,
                &format!(
                    "{partial_name}/{}",
                    path_to_string(
                        &diff_paths(&path_text.path, base_dir).context("cannot diff path")?
                    )?
                ),
                &path_text.text,
            )?;
        }

        Ok(())
    }

    pub(crate) fn dump_acc(
        &self,
        content: ConfigDumpContent,
        partial_name: &str,
        extension: &str,
        acc: &Acc<Option<String>>,
    ) -> anyhow::Result<()> {
        if !self.is_enabled(content) {
            return Ok(());
        }

        for target in TargetOrCommon::iter() {
            self.dump_str(
                content,
                &format!("{partial_name}/{target}.{extension}"),
                &acc[target].clone().unwrap_or_default(),
            )?;
        }

        Ok(())
    }

    pub(crate) fn dump_str(
        &self,
        content: ConfigDumpContent,
        name: &str,
        str: &str,
    ) -> anyhow::Result<()> {
        if !self.is_enabled(content) {
            return Ok(());
        }

        let path = self
            .0
            .dump_directory
            .join(content.to_string().to_case(Case::Snake))
            .join(name);
        debug!("Dumping {name} into {path:?}");

        create_dir_all_and_write(path, str)
    }

    fn is_enabled(&self, content: ConfigDumpContent) -> bool {
        self.0.dump_contents.contains(&content)
    }
}
