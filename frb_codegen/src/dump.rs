#![cfg(feature = "serde")]

use std::collections::HashMap;

use crate::config::all_configs::AllConfigs;
use crate::config::raw_opts::Dump;
use crate::parser::ParserResult;
use enum_iterator::all;

pub fn dump_multi(all_configs: &AllConfigs, dump: Vec<Dump>) -> anyhow::Result<()> {
    let dump = if dump.is_empty() {
        all().collect()
    } else {
        dump
    };
    let data = all_configs
        .get_regular_configs()
        .iter()
        .map(|config| {
            let mut data = HashMap::new();
            for request in &dump {
                match request {
                    Dump::Config => data.insert("config", serde_yaml::to_value(config)?),
                    Dump::Ir => data.insert(
                        "ir",
                        all_configs
                            .get_ir_file(config.block_index)
                            .map(serde_yaml::to_value)??,
                    ),
                };
            }
            Ok(data)
        })
        .collect::<ParserResult<Vec<_>>>()?;
    let data = serde_yaml::to_string(&data)?;
    println!("{data}");
    Ok(())
}
