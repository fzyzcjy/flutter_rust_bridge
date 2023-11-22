use crate::codegen::config::internal_config::InternalConfig;
use crate::codegen::dumper::internal_config::DumperInternalConfig;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::Config;
use serde::Serialize;
use std::fs;

pub(super) mod internal_config;

pub(crate) struct Dumper<'a>(pub &'a DumperInternalConfig);

impl Dumper<'_> {
    pub(crate) fn dump_config(
        &self,
        config: &Config,
        internal_config: &InternalConfig,
    ) -> anyhow::Result<()> {
        self.dump("config.json", config)?;
        self.dump("internal_config.json", internal_config)?;
        Ok(())
    }

    pub(crate) fn dump_ir(&self, ir_pack: &IrPack) -> anyhow::Result<()> {
        self.dump("ir_pack.json", ir_pack)
    }

    fn dump<T: Serialize>(&self, name: &str, data: &T) -> anyhow::Result<()> {
        let path = self.0.dump_directory.join(name);
        Ok(fs::write(path, serde_json::to_string(data)?)?)
    }
}
