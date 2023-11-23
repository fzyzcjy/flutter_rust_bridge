use crate::codegen::config::internal_config::InternalConfig;
use crate::codegen::dumper::internal_config::{ConfigDumpContent, DumperInternalConfig};
use crate::codegen::generator::api_dart::spec_generator::ApiDartOutputSpec;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::Config;
use crate::utils::file_utils::create_dir_all_and_write;
use log::info;
use serde::Serialize;
use std::fs;

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

    pub(crate) fn dump_str(
        &self,
        content: ConfigDumpContent,
        name: &str,
        str: &str,
    ) -> anyhow::Result<()> {
        if !self.is_enabled(content) {
            return Ok(());
        }

        let path = self.0.dump_directory.join(name);
        info!("Dumping {name} into {path:?}");

        create_dir_all_and_write(path, str)
    }

    fn is_enabled(&self, content: ConfigDumpContent) -> bool {
        self.0.dump_contents.contains(&content)
    }
}
